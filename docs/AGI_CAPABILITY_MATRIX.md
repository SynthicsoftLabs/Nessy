<!-- Copyright 2026 Adam Joseph Rivers, CEO, Synthicsoft Labs LLC. Licensed under Apache-2.0. -->

# 🧠 Nessy AGI & Frontier Capability Matrix

## Purpose

Nessy converts research into executable, composable capabilities. The capability graph is the common language between models, runtimes, tools, memory, research systems, execution environments, and autonomous workflows.

## Capability domains

| Domain | Capability surface |
|:--|:--|
| **Reasoning** | structured reasoning, verification, reflection, hypothesis generation, theorem proving |
| **Planning** | decomposition, hierarchical planning, long-horizon planning, replanning, scheduling, dependency graphs |
| **Memory** | working, episodic, semantic, procedural, persistent memory, retrieval, consolidation |
| **Learning** | online adaptation, preference learning, self-evaluation, skill acquisition, curriculum generation, continual improvement |
| **Agents** | tool use, delegation, parallel agents, teams, councils, role specialization, asynchronous tasks |
| **Computer Use** | browser, GUI, terminal, filesystem, IDE, remote desktop |
| **Coding** | generation, repository understanding, refactoring, debugging, testing, benchmarking, worktree orchestration |
| **Research** | search, fetch, synthesis, literature analysis, experiment planning, citation/evidence graphs |
| **Multimodal** | vision, OCR, image, audio, speech, video, cross-modal reasoning |
| **Generation** | text, code, image, video, audio, document generation and editing |
| **Interaction** | realtime voice, streaming, structured output, function calling, MCP, connectors, notifications |
| **Scientific** | symbolic math, numerical computation, simulation, data analysis, model discovery, experiments |
| **Security** | code audit, dependency analysis, threat modeling, secret detection, policy evaluation, telemetry |
| **Infrastructure** | scheduling, peer discovery, failover, checkpointing, replication, content addressing, workload migration |
| **Governance** | provenance, attribution, licensing metadata, audit trails, reproducible builds, artifact signing |
| **Autonomous Engineering** | issue ingestion, failure diagnosis, regression testing, repair, validation, atomic commits, PR promotion, post-merge verification |

## Frontier research coverage

Nessy maintains normalized research coverage for the project-defined frontier systems and expands the registry as additional systems are studied.

| System | Capability areas represented |
|:--|:--|
| **Fable** | long-horizon reasoning, research, coding, agent workflows |
| **Glasswing** | multimodal and interactive agent workflows |
| **Qwen** | reasoning, multimodality, coding, tools, structured interaction, agents |
| **Gemini** | multimodal reasoning, long context, search, code execution, agentic coding, structured interaction |
| **Grok** | reasoning, search, coding, multimodal interaction, tool workflows |
| **Seed** | long-horizon agents, research, coding, computer interaction |
| **Seedance** | multimodal video generation, editing, transformation |
| **Perplexity** | web research, persistent context, connectors, computer agents, asynchronous workflows |
| **Z.ai / GLM** | reasoning, coding, tools, multimodality, agent workflows |
| **Gemma** | reasoning, multimodality, long context, function calling, deployable inference |

The registry is intentionally extensible across additional model and provider families.

## Free/open model fabric

`docs/models/free-models.json` is the machine-readable free/open discovery registry. Its initial source is the public [`12britz/awesome-free-models`](https://github.com/12britz/awesome-free-models) catalog, whose current organization includes open-weight models, free API providers, image/video generation, routers, local runtimes, chat UIs, audio/speech, coding assistants, code models, embeddings, RAG/vector databases, agentic frameworks, MCP tools, fine-tuning, prompt engineering, evaluation/observability, datasets, model hosting, learning resources, and discovery/community resources. citeturn219762view4

Catalog metadata includes roles such as reasoning, coding, multimodal, local inference, routing, and evaluation. Provider records should preserve access/credential requirements and licensing information. The existence of a catalog entry is not itself a guarantee of direct browser access or permanent free availability.

## Capability contract

Each registered capability exposes:

```text
identity
version
provider
inputs
outputs
requirements
quality
latency
health
authorization
telemetry
recovery
composition rules
evaluation suite
provenance
```

## Composition

```text
                 TASK / GOAL
                     │
                     ▼
              REQUIREMENTS GRAPH
                     │
                     ▼
             CAPABILITY MATCHING
                     │
          ┌──────────┼──────────┐
          ▼          ▼          ▼
       PROVIDER    PROVIDER    PROVIDER
          │          │          │
          └──────────┼──────────┘
                     ▼
              COMPOSITE PLAN
                     │
                     ▼
            PARALLEL / SERIAL RUN
                     │
                     ▼
             OBSERVE / EVALUATE
                     │
                     ▼
             MEMORY / SKILL STORE
                     │
                     └──────────► NEXT TASK
```

## Autonomous engineering graph

```text
                   EVENT / FAILURE
                         │
                         ▼
                 CONTROL PLANE
                         │
                         ▼
                   ENGINEER WORKER
                         │
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
      REPRODUCE      REGRESSION      CONTEXT
          │            COVERAGE       ANALYSIS
          └──────────────┬──────────────┘
                         ▼
                    ROOT-CAUSE FIX
                         │
                         ▼
                  FULL VALIDATION
                         │
                         ▼
                    ATOMIC COMMIT
                         │
                         ▼
                         PR
                         │
                         ▼
                     PROMOTE
                         │
                         ▼
                    VERIFY MAIN
```

## AGI runtime graph

```text
WORLD / INPUT
     ↓
PERCEPTION
     ↓
MEMORY GRAPH ↔ KNOWLEDGE GRAPH
     ↓
REASON / PLAN
     ↓
ACT / DELEGATE / CREATE
     ↓
OBSERVE RESULT
     ↓
EVALUATE
     ↓
LEARN / STORE / CONSOLIDATE
     ↓
CONTINUOUS DISCOVERY
```

## Research-to-runtime methodology

```text
RESEARCH
   ↓
EXTRACT FEATURE / FUNCTION
   ↓
DEFINE NORMALIZED CAPABILITY
   ↓
IMPLEMENT ADAPTER
   ↓
ADD TESTS / EVALUATION
   ↓
ADD TELEMETRY
   ↓
ADD PERSISTENCE / RECOVERY
   ↓
REGISTER IN CAPABILITY GRAPH
   ↓
COMPOSE WITH EXISTING CAPABILITIES
   ↓
DEPLOY THROUGH AUTONOMOUS RUNTIME
   ↓
OBSERVE / LEARN / UPDATE
```

## Source of truth

The executable capability graph lives in `crates/capability/src/agi.rs`. Runtime/provider routing lives in `docs/chat/runtime.json`. Free/open model discovery lives in `docs/models/free-models.json`. This matrix describes the capability vocabulary and research coverage; implementation changes must keep those machine-readable registries and canonical documentation synchronized.
