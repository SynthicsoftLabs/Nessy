"""OpenLedger Finance: vendor-neutral real-time market-data aggregation API."""
from __future__ import annotations

import asyncio
import os
import time
from dataclasses import dataclass
from datetime import datetime, timezone
from typing import Protocol

from fastapi import FastAPI, HTTPException, Query
from pydantic import BaseModel


class Quote(BaseModel):
    symbol: str
    bid: float | None = None
    ask: float | None = None
    last: float | None = None
    currency: str = "USD"
    source: str
    timestamp: datetime


class MarketSource(Protocol):
    name: str

    async def quote(self, symbol: str) -> Quote | None: ...


@dataclass
class StaticSource:
    name: str = "static"

    async def quote(self, symbol: str) -> Quote | None:
        return Quote(
            symbol=symbol.upper(), source=self.name, timestamp=datetime.now(timezone.utc)
        )


class TokenBucket:
    def __init__(self, rate: float = 10.0, capacity: int = 20) -> None:
        self.rate = rate
        self.capacity = capacity
        self.tokens = float(capacity)
        self.updated = time.monotonic()
        self.lock = asyncio.Lock()

    async def acquire(self) -> None:
        async with self.lock:
            now = time.monotonic()
            self.tokens = min(self.capacity, self.tokens + (now - self.updated) * self.rate)
            self.updated = now
            if self.tokens < 1:
                await asyncio.sleep((1 - self.tokens) / self.rate)
                self.tokens = 0
            self.tokens -= 1


class Aggregator:
    def __init__(self, sources: list[MarketSource]) -> None:
        self.sources = sources
        self.limiters = {source.name: TokenBucket() for source in sources}

    async def quote(self, symbol: str) -> Quote:
        if not symbol or len(symbol) > 32:
            raise ValueError("invalid symbol")
        symbol = symbol.upper()
        errors: list[str] = []
        for source in self.sources:
            try:
                await self.limiters[source.name].acquire()
                result = await source.quote(symbol)
                if result and any(v is not None for v in (result.bid, result.ask, result.last)):
                    return result
            except Exception as exc:  # isolate one provider from the aggregation plane
                errors.append(f"{source.name}: {type(exc).__name__}")
        detail = f"no quote available for {symbol}"
        if errors:
            detail += f"; source failures: {', '.join(errors)}"
        raise LookupError(detail)


def build_sources() -> list[MarketSource]:
    sources: list[MarketSource] = []
    if os.getenv("OPENLEDGER_BLOOMBERG_ENABLED", "0").lower() in {"1", "true", "yes"}:
        from bloomberg_source import BloombergSource

        sources.append(BloombergSource())
    sources.append(StaticSource())
    return sources


app = FastAPI(title="OpenLedger Finance", version="0.2.0")
aggregator = Aggregator(build_sources())


@app.get("/health")
async def health() -> dict[str, object]:
    return {"status": "ok", "service": "openledger-finance", "sources": [s.name for s in aggregator.sources]}


@app.get("/v1/quote", response_model=Quote)
async def quote(symbol: str = Query(..., min_length=1, max_length=32)) -> Quote:
    try:
        return await aggregator.quote(symbol)
    except ValueError as exc:
        raise HTTPException(status_code=400, detail=str(exc)) from exc
    except LookupError as exc:
        raise HTTPException(status_code=404, detail=str(exc)) from exc


@app.get("/v1/sources")
async def sources() -> dict[str, list[str]]:
    return {"sources": [source.name for source in aggregator.sources]}
