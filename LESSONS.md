# Lessons

Written by /aar-loop after each session's After Action Review. Read this file before starting a new task in this project. Every entry should be concrete and checkable, never vague.

## 2026-09-06 -- Design-question answers about FHIR structure (Element/Primitive relationship, invariants like ele-1) were given from recall instead of fetching hl7.org/fhir/R5 first, even though the repo's spec-driven rule already applies to primitive grammars. The gap (primitives are Elements too, need id/extension) only surfaced because the user manually triggered a second advisor opinion.
- Expected: Repo rule 'verify grammar against hl7.org/fhir/R5, not memory' would cover any FHIR spec claim, including type-framework design discussion.
- Actual: First answer on Element vs Primitive<T> design was reasoning/recall-based; the missing constraint (primitives carry id+extension per Element) was only caught after the user said 'check with advisor.'
- Why: The verification rule was read as applying to primitive grammar/regex specifically (matches existing module-doc pattern), not to broader FHIR structural/design claims, so no WebFetch was triggered before answering the design question.
- tags: fhir-spec,verification,design
