<!-- Copyright 2026 Synthicsoft Labs LLC; Licensed under Apache-2.0. -->

# 🐢 Frontier Capability Fabric

> **Nessy continuously tracks frontier model capabilities and maps them into one provider-neutral runtime fabric.**

This document is an implementation map, not a claim that Nessy reproduces proprietary model internals. Publicly documented capabilities are translated into interfaces, adapters, routing strategies, evaluation tasks, and runtime services that Nessy can actually execute.

## ◈ Capability Matrix

| Frontier | Public capability signal | Nessy capability |
|:--|:--|:--|
| **Claude Fable 5 / Glasswing** | Long-horizon reasoning, coding, vision, research, adaptive thinking, task budgets | `reasoning`, `long_horizon`, `coding`, `vision`, `research`, `effort`, `budgeted_execution` |
| **Qwen** | Hybrid thinking, 1M context, multimodal reasoning, agent execution, search/code tools, realtime audio, image/video generation | `thinking`, `long_context`, `multimodal`, `tool_use`, `search`, `code_exec`, `audio_realtime`, `media_generation` |
| **Gemini 3.1** | Multimodal input, agentic coding, search, code execution, structured output, function calling, 1M context | `multimodal`, `agentic_coding`, `search`, `code_exec`, `structured_output`, `function_calling`, `long_context` |
| **Grok 4.1** | Reinforcement-trained collaboration, personality, intent sensitivity, large-scale reward evaluation | `collaboration`, `adaptive_style`, `intent_modeling`, `response_evaluation` |
| **Seed 2.0** | Long-horizon agents, multimodal reasoning, coding, GUI interaction, professional workflows | `long_horizon`, `multimodal`, `coding`, `gui`, `workflow_execution` |
| **Seedance 2.5** | Audio-video generation, multimodal references, long-form storytelling, editing and extension | `video_generation`, `audio_video`, `reference_generation`, `video_edit`, `video_extend` |
| **Perplexity Computer** | Search-native agents, hundreds of connectors, document creation, persistent memory, scheduling, sandboxed execution, multi-model orchestration | `web_research`, `connectors`, `artifact_generation`, `memory`, `scheduling`, `sandbox`, `model_council` |
| **Z.ai / GLM** | Long-context reasoning, coding, agent workloads, cybersecurity evaluation, open-weight deployment | `long_context`, `reasoning`, `coding`, `agent_tasks`, `security_analysis`, `open_weight` |
| **Gemma 4** | Multimodal reasoning, 128K/256K context, function calling, coding, configurable thinking, MoE, speculative decoding | `multimodal`, `long_context`, `function_calling`, `coding`, `thinking`, `moe`, `speculative_decode` |

## ⚡ AGI Capability Stack

Nessy treats advanced intelligence as a composition of continuously evaluated capabilities rather than one monolithic model.

```text
                         ┌─────────────────────────┐
                         │       INTELLIGENCE      │
                         │   GOAL → MODEL → ACT   │
                         └────────────┬────────────┘
                                      │
              ┌───────────────────────┼───────────────────────┐
              ▼                       ▼                       ▼
        ┌───────────┐           ┌───────────┐           ┌───────────┐
        │ REASONING │           │  MEMORY   │           │  WORLD    │
        │ planning  │           │ episodic  │           │  MODELS   │
        │ reflection│           │ semantic  │           │ vision    │
        └─────┬─────┘           └─────┬─────┘           └─────┬─────┘
              └────────────────────────┼────────────────────────┘
                                       ▼
                              ┌─────────────────┐
                              │    TOOL FABRIC  │
                              │ web · code · MCP│
                              │ GUI · APIs · P2P │
                              └────────┬────────┘
                                       ▼
                              ┌─────────────────┐
                              │ EXECUTION FABRIC│
                              │ Turtle + Koopa  │
                              └────────┬────────┘
                                       ▼
                              ┌─────────────────┐
                              │  EVALUATE /     │
                              │  LEARN / ROUTE  │
                              └────────┬────────┘
                                       └──────────► CONTINUE
```

## 🧠 Research Loop

The capability registry is intended to be updated from current public documentation and benchmark evidence.

```text
PUBLIC RESEARCH
      ↓
CAPABILITY EXTRACTION
      ↓
NORMALIZED FEATURE
      ↓
ADAPTER / SERVICE
      ↓
INTEGRATION TEST
      ↓
BENCHMARK
      ↓
ROUTER UPDATE
      ↓
RUNTIME DEPLOYMENT
      ↓
OBSERVE
      └──────────────→ RESEARCH
```

## 🔭 Active Domains

The fabric should continuously cover:

- reasoning and test-time compute;
- planning and long-horizon task execution;
- persistent and self-improving memory;
- web search and evidence synthesis;
- code execution and repository engineering;
- GUI and computer-use agents;
- multimodal text/image/audio/video understanding;
- image generation and editing;
- audio generation, recognition, and realtime dialogue;
- video generation, continuation, and editing;
- MCP and general tool calling;
- model councils and multi-model consensus;
- connector ecosystems;
- scheduled and asynchronous execution;
- sandboxed execution;
- structured outputs and function calling;
- speculative and efficient inference;
- model discovery, evaluation, and dynamic routing;
- scientific and technical research workflows;
- security analysis and defensive code auditing;
- autonomous software engineering;
- capability benchmarking and continuous improvement.

## ✦ Provider-Neutral Contract

A provider advertises capabilities; Nessy selects the best available implementation.

```text
Provider → CapabilitySet → Health → Cost/Latency → Router → Task
```

The same task contract can therefore move between Qwen, Gemini, Gemma, GLM, Kairos, Perplexity-backed services, other compatible providers, or future systems without changing the orchestration layer.

## 📚 Current Research Sources

The implementation baseline is refreshed against public sources including:

- Anthropic's Fable 5 documentation and release material;
- Google DeepMind Gemini 3.1 documentation and model cards;
- Alibaba/Qwen model releases and documentation;
- ByteDance Seed and Seedance releases;
- Perplexity Computer and Agent API documentation;
- Google Gemma 4 documentation and model cards;
- Z.ai / GLM public releases and evaluations;
- xAI Grok public model announcements.

The registry is deliberately extensible: new models and capabilities are added as provider-neutral capability records rather than requiring architectural rewrites.
