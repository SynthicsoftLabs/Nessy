from fastapi.testclient import TestClient

from app import app

client = TestClient(app)


def test_health() -> None:
    response = client.get("/health")
    assert response.status_code == 200
    assert response.json()["status"] == "ok"


def test_sources() -> None:
    response = client.get("/v1/sources")
    assert response.status_code == 200
    assert response.json() == {"sources": ["static"]}


def test_unknown_quote_is_explicit() -> None:
    response = client.get("/v1/quote", params={"symbol": "AAPL"})
    assert response.status_code == 404


def test_symbol_validation() -> None:
    response = client.get("/v1/quote", params={"symbol": ""})
    assert response.status_code == 422
