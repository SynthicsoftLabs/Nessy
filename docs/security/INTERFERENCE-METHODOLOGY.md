# Nessy Interference Detection, Ingestion, and Response Methodology

Copyright 2026 Synthicsoft Labs LLC
Licensed under the Apache License, Version 2.0.

## Purpose

Nessy treats repository, build, runtime, dependency, and control-plane integrity as continuously observable properties. The methodology combines established public guidance from NIST, CISA, NSA/ODNI/ESF, and the UK NCSC with repository-native cryptographic verification and automated evidence collection.

This is an implementation methodology, not an attribution engine. A signal is preserved as evidence first; attribution is established only from corroborating telemetry.

## Design principles

1. **Independent evidence:** compare multiple independent representations of the same state: event SHA, checked-out SHA, Git tree, GitHub API state, workflow source, dependency graph, and artifact hashes.
2. **Continuous verification:** validate before integration, during execution, after mutation, and after publication.
3. **Least privilege and segmentation:** workflows receive only the permissions required for their job; security telemetry is separated from mutable application artifacts.
4. **Immutable evidence:** every security event records timestamp, source, observed object, expected value, observed value, and SHA-256 evidence digest.
5. **Ingest → normalize → correlate → decide → respond → verify:** raw signals are never silently discarded.
6. **Fail closed for integrity:** an integrity mismatch prevents promotion of the affected artifact while an independent verification path continues collecting evidence.
7. **Redundant observations:** a single GitHub API response is never the sole source of truth for a critical mutation.

## Agency-derived controls

### NIST

NIST SP 800-61 Rev. 3 provides the incident-response lifecycle and integration of preparation, detection, response, and recovery into cybersecurity risk management. Nessy maps this to continuous repository telemetry, automated correlation, containment of affected artifacts, and post-recovery verification.

NIST SP 800-218 establishes secure software-development practices intended to reduce vulnerabilities and address their root causes. Nessy maps this to dependency verification, source integrity checks, build provenance, automated testing, and release verification.

Sources:
- https://csrc.nist.gov/pubs/sp/800/61/r3/final
- https://csrc.nist.gov/pubs/sp/800/218/r1/ipd

### CISA / NSA / ODNI / ESF

The Enduring Security Framework software-supply-chain guidance emphasizes securing source code, third-party components, build environments, and delivery. Current SBOM guidance also emphasizes author signatures, versions, and component hashes. Nessy maps this to SBOM generation, dependency/license/advisory checks, content hashes, and artifact verification.

Sources:
- https://www.nsa.gov/press-room/press-releases-statements/press-release-view/article/3146465/nsa-cisa-odni-release-software-supply-chain-guidance-for-developers/
- https://www.nsa.gov/Press-Room/Press-Releases-Statements/Press-Release-View/Article/4558391/nsa-collaborates-with-cisa-to-co-author-the-updated-software-bill-of-materials/

### NSA Zero Trust Implementation Guidelines

The NSA Zero Trust guidance emphasizes application/code identification, versioned source-code integrity, secure and segmented development infrastructure, signed code, hash verification, SBOM use, continuous monitoring, threat-intelligence ingestion, correlation, and automated isolation/mitigation of suspected supply-chain compromise.

Nessy implements those concepts as repository integrity gates, object-hash comparison, dependency inventory, telemetry ingestion, correlation rules, and automated quarantine of anomalous artifacts.

Source:
- https://www.nsa.gov/Cybersecurity/ZIG/Capabilities/Application-and-Workload/

### UK NCSC

NCSC logging guidance treats logging as the foundation for security monitoring and situational awareness and recommends retaining enough context to reconstruct an incident. Nessy therefore stores normalized event records with source, object identity, expected/observed state, and evidence hashes.

Sources:
- https://www.ncsc.gov.uk/sites/default/files/pdfs/publication/introduction-logging-security-purposes.pdf
- https://www.ncsc.gov.uk/sites/default/files/2026-03/What-exactly-should-we-be-logging.pdf

## Ingestion pipeline

```text
GitHub events / CI logs / Git objects / dependency metadata / runtime probes
                 |
                 v
             COLLECT
                 |
                 v
             NORMALIZE
                 |
                 v
       SHA-256 evidence digest
                 |
                 v
             CORRELATE
          /       |       \
     identity   timing   state
                 |
                 v
             CLASSIFY
        expected / drift / anomaly
                 |
       +---------+---------+
       |                   |
   expected             anomaly
       |                   |
       v                   v
 continue             quarantine
                         + evidence
                         |
                         v
                    independent
                    re-verification
```

## High-value interference signals

- Event SHA differs from checked-out HEAD.
- Remote branch ref changes unexpectedly during a mutation window.
- Commit tree differs from the previously recorded tree for the same commit.
- Critical workflow file hash changes without an associated expected commit.
- Git object database fails strict verification.
- Generated artifact hash differs between independent builds.
- Dependency graph changes without corresponding manifest/lock changes.
- A workflow requests permissions inconsistent with its declared purpose.
- A runtime endpoint changes identity, certificate, or response provenance unexpectedly.
- Repeated API conflicts/timeouts correlate with unexpected ref movement.
- Security telemetry disappears while application telemetry continues.

## Response state machine

```text
OBSERVED
   |
   v
CORRELATED
   |
   +--> EXPECTED --> VERIFIED --> CONTINUE
   |
   +--> ANOMALOUS --> SNAPSHOT --> QUARANTINE --> REVERIFY
                                      |
                                      v
                              RESTORE VERIFIED TREE
                                      |
                                      v
                                POST-RESTORE CHECK
```

## Evidence record

The ingestion format is JSON Lines. Each record should contain:

- `event_id`
- `timestamp`
- `source`
- `event_type`
- `object`
- `expected`
- `observed`
- `commit_sha`
- `tree_sha`
- `evidence_sha256`
- `classification`

## Operational outcome

The methodology is deliberately evidence-first. It detects tampering, stale writes, unexpected concurrent mutations, dependency drift, workflow changes, artifact substitution, and telemetry gaps without requiring a particular explanation for the event. Detection, evidence preservation, automated quarantine, alternate verification, and recovery are all repository-native capabilities.
