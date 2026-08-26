"""Optional Bloomberg BLPAPI market-data adapter.

The adapter requires an authorized Bloomberg SAPI/B-PIPE environment and the
official ``blpapi`` package. It never attempts to bypass Bloomberg
authentication, entitlements, network controls, or throttling.
"""
from __future__ import annotations

import asyncio
import os
from datetime import datetime, timezone
from typing import Any

from app import Quote


class BloombergSource:
    name = "bloomberg"

    def __init__(
        self,
        host: str | None = None,
        port: int | None = None,
        service: str | None = None,
    ) -> None:
        self.host = host or os.getenv("BLOOMBERG_HOST", "127.0.0.1")
        self.port = port or int(os.getenv("BLOOMBERG_PORT", "8194"))
        self.service = service or os.getenv("BLOOMBERG_SERVICE", "//blp/refdata")

    async def quote(self, symbol: str) -> Quote | None:
        return await asyncio.to_thread(self._quote_sync, symbol)

    def _quote_sync(self, symbol: str) -> Quote | None:
        try:
            import blpapi
        except ImportError as exc:
            raise RuntimeError(
                "Bloomberg adapter requires the official blpapi package"
            ) from exc

        options = blpapi.SessionOptions()
        options.setServerHost(self.host)
        options.setServerPort(self.port)
        session = blpapi.Session(options)
        if not session.start():
            return None

        try:
            if not session.openService(self.service):
                return None
            service = session.getService(self.service)
            request = service.createRequest("ReferenceDataRequest")
            request.getElement("securities").appendValue(symbol.upper())
            fields = request.getElement("fields")
            for field in ("PX_BID", "PX_ASK", "PX_LAST", "CRNCY"):
                fields.appendValue(field)

            session.sendRequest(request)
            while True:
                event = session.nextEvent(5000)
                for message in event:
                    if not message.hasElement("securityData"):
                        continue
                    security_data = message.getElement("securityData")
                    if security_data.numValues() == 0:
                        continue
                    row = security_data.getValueAsElement(0)
                    field_data = row.getElement("fieldData")

                    def value(name: str) -> Any:
                        if not field_data.hasElement(name):
                            return None
                        element = field_data.getElement(name)
                        return element.getValue()

                    bid = value("PX_BID")
                    ask = value("PX_ASK")
                    last = value("PX_LAST")
                    currency = value("CRNCY") or "USD"
                    if bid is None and ask is None and last is None:
                        return None
                    return Quote(
                        symbol=symbol.upper(),
                        bid=float(bid) if bid is not None else None,
                        ask=float(ask) if ask is not None else None,
                        last=float(last) if last is not None else None,
                        currency=str(currency),
                        source=self.name,
                        timestamp=datetime.now(timezone.utc),
                    )
                if event.eventType() == blpapi.Event.RESPONSE:
                    break
            return None
        finally:
            session.stop()
