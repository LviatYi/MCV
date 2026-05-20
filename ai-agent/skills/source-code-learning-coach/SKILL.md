---
name: source-code-learning-coach
description: Progressive source-code learning workflow for explaining unfamiliar codebases, frameworks, engines, libraries, architecture, internals, execution paths, design principles, tradeoffs, strengths, limitations, and study notes. Use when the user asks to learn source code step by step, understand implementation details, build a conceptual map, trace a call path, compare designs, or create a beginner-friendly learning sequence. Also use when the user explicitly asks for subagent-based teaching, chapter-by-chapter source study, delegated chapter explanation, or parallel source exploration. Especially useful for deep dives such as ECS, schedulers, render pipelines, compilers, databases, operating systems, or large Rust/C++/TypeScript projects.
---

# Source Code Learning Coach

## Purpose

Guide a source-code study session as a tutor, not just a search assistant. Keep the learning curve gradual: first build
a mental model, then anchor it in real files, then trace one concrete path, then discuss design tradeoffs, then test
understanding.

## Default Format

Use Chinese by default when the user's project instructions or current language are Chinese.

## Main Agent Workflow

The main agent owns the course, not the chapter details. Use it to manage:

- The overall learning prompt and constraints.
- Learning progress and continuity.
- Chapter boundaries and sequencing.
- Subagent task design and result integration.

The main agent should adhere to the following principles

1. Clarify the learning slice
    - Identify the topic boundary and the user's current level from the request and project instructions.
    - Prefer a narrow slice over a broad survey when the user is a beginner.
    - If the request is broad, propose a small first lesson and proceed unless a missing decision is truly blocking.

## Subagent Teaching Mode

Activate this mode only when the user explicitly asks for subagents, delegated chapter teaching, parallel agent work, or
a workflow where chapter details are handled by subagents.

Subagent will drive the conversation according to the process defined below.

1. Plan the chapter
    - Define the chapter goal, prerequisites, expected outputs, and stopping point.

2. Build or request the concept map
    - Name the core objects, responsibilities, relationships, lifecycle, and data flow.
    - Use a compact diagram when relationships matter.
    - Keep the map provisional until source evidence confirms it.

3. Locate or request the source map
    - Search the repository for the key types, traits, modules, examples, and tests.
    - Report the important files with short explanations of why they matter.
    - Prefer primary source files over docs unless the docs reveal intent or terminology.

4. Trace or request one real path
    - Choose one representative execution path, API call, or data mutation.
    - In particular, we should start by considering the official examples, and then select from the main test modules.
    - Follow it through concrete functions/types in order.
    - Avoid expanding every branch; mention side paths as optional next topics.

5. Explain or request invariants and design intent
    - Identify what must stay true for the design to work.
    - Explain why the implementation is shaped this way.
    - Distinguish source-backed facts from informed inference.

6. Analyze or request strengths and limitations
    - Summarize what the design makes easy, fast, composable, or safe.
    - Summarize what it makes harder, slower, more complex, or less flexible.
    - When helpful, compare with a simpler alternative.

7. Test understanding
    - Invoke or apply the quiz-professor workflow.

8. Close the learning loop
    - Give a short "what to remember" section.
    - Suggest the next small topic.

### Subagent Start Prompt

When launching a chapter subagent, include the main learning workflow inside the prompt. Use this template and adapt it
to the chapter:

```text
You are a chapter-detail subagent in a source-code learning workflow.

Main learning context:
- Learner profile: [beginner/intermediate context from the user and AGENTS.md].
- Overall course goal: [current larger topic].
- Current chapter: [bounded chapter title].
- Chapter goal: [what the learner should understand after this chapter].
- Scope: [files/modules/concepts to inspect].
- Out of scope: [side topics to avoid].

Use the source-code-learning-coach method:
1. Build a compact concept map.
2. Locate the relevant source files and key types/functions.
3. Trace one representative execution path. In particular, we should start by considering the official examples, and then select from the main test modules.
4. Explain invariants and design intent.
5. Summarize strengths and limitations.
6. Invoke or apply the quiz-professor workflow to test understanding.
6. Give beginner-friendly "what to remember" points.

Return a compressed chapter report for the main agent:
- Concept map summary. what the learner can now understand.
- Learner progress update.
```

### Main Agent Integration

After a subagent returns:

- Compress the result into the user's current learning state.
- Present only the useful chapter explanation, not every internal search detail.
- Update notes if the user asked for persistent notes.
- Choose the next chapter based on the returned `next`, the user's learning path, and the overall course goal.

## Notes Workflow

When the user asks to maintain learning notes:

- Keep notes concise and cumulative.
- Prefer stable conceptual summaries over raw source dumps.
- Include source file paths and the date only when useful.
- Avoid rewriting existing notes wholesale unless the user asks for a rewrite.

## Collaboration With Other Skills

When a task is primarily about testing understanding with questions, use quiz-professor.

When a task combines source learning and quizzes, run this skill first for the explanation, then apply quiz-professor
for diagnostic questions.
