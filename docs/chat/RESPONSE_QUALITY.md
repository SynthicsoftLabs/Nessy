<!-- Copyright 2026 Adam Joseph Rivers, CEO, Synthicsoft Labs LLC. Licensed under Apache-2.0. -->
# Chat response quality

The browser runtime treats repetitive readiness prompts as failed inference rather than valid answers.

The quality gate is applied before a response enters durable conversation context. Rejected assistant turns are also excluded from future model context so a bad generation cannot reinforce its own loop.

The primary browser model is `HuggingFaceTB/SmolLM2-1.7B-Instruct`, followed by the smaller SmolLM2 360M and 135M models as browser-side fallback engines.
