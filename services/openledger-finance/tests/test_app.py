import asyncio
from datetime import datetime, timezone

from fastapi.testclient import TestClient

from app import Aggregator, Quote, StaticSource, app

client = TestClient(app)


def test_health() -> None:
    response = client.get("/health")
    assert response.status_code == 200
    assert response.json()["status"] == "ok"
    assert "static" in response.json()["sources"]


def test_sources() -> None:
    response = client.get("/v1/sources")
    assert response.status_code == 200
    assert "static" in response.json()["sources"]


def test_unknown_quote_is_explicit() -> None:
    response = client.get("/v1/quote", params={"symbol": "AAPL"})
    assert response.status_code == 404


def test_symbol_validation() -> None:
    response = client.get("/v1/quote", params={"symbol": ""})
    assert response.status_code == 422


def test_aggregator_accepts_first_real_quote() -> None:
    class FakeSource:
        name = "fake"

        async def quote(self, symbol: str) -> Quote:
            return Quote(
                symbol=symbol,
                last=123.45,
                source=self.name,
                timestamp=datetime.now(timezone.utc),
            )

    result = asyncio.run(Aggregator([FakeSource(), StaticSource()]).quote("AAPL"))
    assert result.last == 123.45
    assert result.source == "fake"
