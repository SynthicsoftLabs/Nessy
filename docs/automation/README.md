# Nessy Automation

This directory documents repository automation invariants and execution contracts.

The autonomous supervisor must execute GitHub CLI commands with an explicit `--repo` target and from a checked-out repository workspace. This prevents GitHub CLI repository discovery from depending on runner working-directory state.
