// Copyright 2026 Adam Joseph Rivers, CEO, Synthicsoft Labs LLC.
// Licensed under the Apache License, Version 2.0.
const runtime = JSON.parse(require('node:fs').readFileSync('docs/chat/runtime.json','utf8'));
const models = runtime.browser.models.map((m) => m.model);
const expected = [
  'HuggingFaceTB/SmolLM2-1.7B-Instruct',
  'HuggingFaceTB/SmolLM2-360M-Instruct',
  'HuggingFaceTB/SmolLM2-135M-Instruct',
];
if (JSON.stringify(models.slice(0, 3)) !== JSON.stringify(expected)) {
  throw new Error(`browser fallback order mismatch: ${JSON.stringify(models)}`);
}
console.log('chat-model-routing: PASS');
