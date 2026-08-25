// Copyright 2026 Adam Joseph Rivers, CEO, Synthicsoft Labs LLC.
// Licensed under the Apache License, Version 2.0.
const fs = require('node:fs');
const html = fs.readFileSync('docs/chat/index.html', 'utf8');
const app = fs.readFileSync('docs/chat/app.js', 'utf8');
const quality = fs.readFileSync('docs/chat/response-quality.js', 'utf8');

for (const src of [app, quality]) {
  if (!src.includes("'use strict'")) throw new Error('strict mode missing');
}
for (const asset of ['./response-quality.js', './app.js']) {
  if (!html.includes(`src="${asset}"`)) throw new Error(`missing script asset ${asset}`);
}
for (const id of ['newChat','exportAll','importBtn','settingsBtn','clearAll','shareBtn','copyLast','regenerate','form','input','mode','runtime']) {
  if (!app.includes(`id('${id}')`)) throw new Error(`missing controller binding ${id}`);
}
for (const required of [
  'HuggingFaceTB/SmolLM2-1.7B-Instruct',
  'HuggingFaceTB/SmolLM2-360M-Instruct',
  'HuggingFaceTB/SmolLM2-135M-Instruct',
  'NessyResponseQuality',
  'repetition_penalty:1.08',
  'top_p:0.9'
]) {
  if (!app.includes(required)) throw new Error(`missing runtime quality feature ${required}`);
}
console.log('chat-engineering: PASS');
