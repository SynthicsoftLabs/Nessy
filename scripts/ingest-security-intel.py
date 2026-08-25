#!/usr/bin/env python3
# Copyright 2026 Synthicsoft Labs LLC
# Licensed under the Apache License, Version 2.0.
"""Normalize security/interference observations into tamper-evident JSONL.

Input: JSON objects, one per line, from stdin or a file.
Output: normalized JSONL with deterministic SHA-256 evidence digests.
"""
import argparse
import hashlib
import json
import sys
from datetime import datetime, timezone

FIELDS = ("event_id", "timestamp", "source", "event_type", "object", "expected", "observed", "commit_sha", "tree_sha", "classification")


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def normalize(raw):
    now = datetime.now(timezone.utc).isoformat().replace("+00:00", "Z")
    event = {k: raw.get(k) for k in FIELDS}
    event["timestamp"] = event["timestamp"] or now
    event["event_id"] = event["event_id"] or hashlib.sha256(canonical(raw).encode()).hexdigest()[:32]
    event["source"] = event["source"] or "unknown"
    event["event_type"] = event["event_type"] or "observation"
    event["object"] = event["object"] or "unknown"
    event["classification"] = event["classification"] or classify(event)
    evidence = canonical(event)
    event["evidence_sha256"] = hashlib.sha256(evidence.encode()).hexdigest()
    return event


def classify(event):
    expected, observed = event.get("expected"), event.get("observed")
    if expected is not None and observed is not None and expected != observed:
        return "anomaly"
    return "observed"


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("input", nargs="?", default="-")
    parser.add_argument("-o", "--output", default="-")
    args = parser.parse_args()
    src = sys.stdin if args.input == "-" else open(args.input, encoding="utf-8")
    dst = sys.stdout if args.output == "-" else open(args.output, "w", encoding="utf-8")
    try:
        for lineno, line in enumerate(src, 1):
            if not line.strip():
                continue
            try:
                raw = json.loads(line)
                if not isinstance(raw, dict):
                    raise ValueError("event must be an object")
                dst.write(canonical(normalize(raw)) + "\n")
            except Exception as exc:
                print(f"ingest error line {lineno}: {exc}", file=sys.stderr)
                return 2
    finally:
        if src is not sys.stdin:
            src.close()
        if dst is not sys.stdout:
            dst.close()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
