---
name: quiz-professor
description: Enabled only when the user explicitly uses the skill! Diagnostic quiz and active-recall workflow for testing understanding with judgment questions, true/false questions, misconception traps, short-answer prompts, answer keys, explanations, and difficulty progression. Use when the user asks for quiz questions, 判断题, self-checks, exercises, comprehension tests, active recall, learning assessment, or when another learning workflow needs to close a lesson by verifying understanding.
---

# Quiz Professor

## Purpose

Design questions that reveal whether the learner understands the concept, not whether they memorized wording. Prefer small sets of high-signal questions with explanations.

## Question Design

1. Test one idea per question.
2. Mix obvious fundamentals with carefully designed misconception traps.
3. Make false statements plausible, not silly.
4. Avoid ambiguity unless the goal is to discuss nuance.
5. Tie each answer back to the concept, source behavior, or design tradeoff.
6. For source-code learning, include questions about relationships, execution order, ownership boundaries, invariants, and limitations.

## Default Format

For Chinese learning sessions, prefer 判断题 unless the user asks otherwise.

Use this shape:

```md
## 判断题

1. [DifficultyLadder] Statement
2. [DifficultyLadder] Statement
3. [DifficultyLadder] Statement

## 答案与解析

1. 对/错. Explanation. 
2. 对/错. Explanation. 
3. 对/错. Explanation. 
```

## Difficulty Ladder

- Easy: terminology, object relationships, direct behavior, common misconceptions.
- Difficult: execution order, invariants, why the design works, tradeoffs, limitations, edge cases.

## Calibration

For beginners:

- Start with 5-8 questions.
- Keep statements short.
- Explain wrong options gently and concretely.

For advanced users:

- Include edge cases and design alternatives.
- Ask for justification before revealing answers when the user wants practice mode.

## Interaction Modes

- Always provide questions and answers together.

Use practice mode when the user says "考我", "不要先给答案", "练习模式", or asks to answer interactively.

## Source-Code Quiz Focus

Choose any suitable range of questions to examine. Here are some possible questions:

- Which type owns which responsibility.
- What happens before or after a call.
- Whether data is stored densely, sparsely, globally, or per-system.
- Which guarantees are compile-time, runtime, scheduling-time, or convention-based.
- What design tradeoff follows from an implementation choice.
