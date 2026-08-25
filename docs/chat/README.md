# Nessy browser chat

The repository chat is a versioned static application composed of `index.html`, `response-quality.js`, `app.js`, and `runtime.json`.

The browser inference order is:

1. `HuggingFaceTB/SmolLM2-1.7B-Instruct`
2. `HuggingFaceTB/SmolLM2-360M-Instruct`
3. `HuggingFaceTB/SmolLM2-135M-Instruct`

The response-quality gate rejects repetitive readiness/task-request loops before those responses are persisted or reused as model context. Rejected generations are retried with an execution-focused instruction set.
