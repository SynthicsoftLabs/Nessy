// Copyright 2026 Adam Joseph Rivers, CEO, Synthicsoft Labs LLC.
// Licensed under the Apache License, Version 2.0.
const fs = require('node:fs');
const vm = require('node:vm');
const source = fs.readFileSync('docs/chat/response-quality.js', 'utf8');
const context = { window: {} };
vm.runInNewContext(source, context, { filename: 'response-quality.js' });
const quality = context.window.NessyResponseQuality;
if (!quality) throw new Error('NessyResponseQuality export missing');

const cases = [
  {
    input: '"Ah, I\'m ready to assist you. What\'s the task you\'d like to perform? Please provide the details, and I\'ll get started immediately."',
    expectedBad: true,
  },
  {
    input: '"Hello. How can I assist you today? Do you have a task in mind?"',
    expectedBad: true,
  },
  {
    input: 'An autonomous Python system can use a planner, tool registry, durable state, an execution loop, and a verifier. Start with a typed task model and a bounded scheduler.',
    expectedBad: false,
  },
];

for (const test of cases) {
  const result = quality.evaluate(test.input);
  if (result.lowQuality !== test.expectedBad) {
    throw new Error(`quality regression for ${JSON.stringify(test.input)}: got ${result.lowQuality}`);
  }
}

const cleaned = quality.clean('"Hello. How can I assist you today?"');
if (cleaned !== 'Hello. How can I assist you today?') {
  throw new Error(`quote cleanup regression: ${JSON.stringify(cleaned)}`);
}

console.log('chat-response-quality: PASS');
