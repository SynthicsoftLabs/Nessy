<!--
Copyright 2026 Synthicsoft Labs LLC
Licensed under the Apache License, Version 2.0; and MIT License.
-->
# Security

## Runtime posture

BowserAI is local-first and deny-by-default at privileged execution boundaries. Credentials must be supplied through the runtime secret mechanism and must never be committed to the repository.

Network-capable components must enforce bounded timeouts and explicit error handling. Tool metadata is validated before dispatch. Destructive operations must be represented explicitly rather than inferred from free-form descriptions.

## Dependency security

CI runs RustSec auditing and `cargo-deny` policy checks. Dependency additions should include their license and source implications in the review.

## Secrets

Use environment injection, operating-system credential stores, or the future encrypted Bowser secret store. Never add private keys, access tokens, passwords, session cookies, or production configuration containing secrets to Git.

## Reporting

For vulnerabilities, use the repository's private GitHub security-advisory mechanism when available. Do not publish exploit details in a public issue before coordinated remediation.
