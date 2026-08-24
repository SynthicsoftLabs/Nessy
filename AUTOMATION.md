# BowserAI Autonomous Engineering

BowserAI/Nessy is maintained by automated agents and machine-enforced CI gates.

## Automation contract

- Every change is validated by deterministic formatting, compilation, tests, linting, dependency auditing, license policy, and SBOM generation.
- No credentials are stored in the repository.
- The project license is Apache License 2.0 only.
- Automation may open, validate, repair, and merge changes when repository policy permits.
- Production execution remains deny-by-default until an explicit sandbox backend is enabled.

## Failure handling

A failed gate blocks promotion. Automated repair jobs may create a corrective commit and rerun the complete validation pipeline. Persistent failures remain blocked rather than being promoted as known-bad artifacts.
