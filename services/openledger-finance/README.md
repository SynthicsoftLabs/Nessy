# OpenLedger Finance

Vendor-neutral real-time market-data aggregation service for Nessy.

## Design

```text
licensed/public feeds
        |
        v
 source adapters -> rate limiter -> normalizer -> fallback router -> FastAPI
                                             |
                                             v
                                      unified Quote schema
```

### Source contract

Every adapter implements `MarketSource.quote(symbol)` and returns the normalized `Quote` model. New feeds can therefore be added without changing API consumers.

The service is intentionally **not** an access-control bypass. Bloomberg and other commercial feeds remain available only through their documented authentication, entitlement, and rate-limit contracts. OpenLedger supplies the aggregation, normalization, caching, routing, and API layer so applications can combine lawful public/open and licensed sources behind one interface.

## Run

```bash
cd services/openledger-finance
python -m venv .venv
. .venv/bin/activate
pip install -r requirements.txt
uvicorn app:app --reload
```

## Test

```bash
cd services/openledger-finance
pytest -q
```

## Attribution and license

Created for Synthicsoft Labs. Repository-level licensing and attribution controls remain authoritative; this service does not replace or override the repository's existing `LICENSE` or `NOTICE` files.
