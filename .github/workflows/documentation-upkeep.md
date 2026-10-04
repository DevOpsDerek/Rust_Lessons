---
on:
  workflow_dispatch:
permissions:
  contents: read
  issues: read
  pull-requests: read
inlined-imports: true
imports:
  - DevOpsDerek/workflows/.github/workflows/shared/agentic/documentation-upkeep.md@dac4b81c298cb3ea6821ea312efa5375f42d5ccb
tools:
  github:
safe-outputs:
  create-pull-request:
    max: 1
    draft: true
    fallback-as-issue: false
    protected-files: allowed
    allowed-files:
      - "README.md"
      - "lesson_*/README.md"
---

Follow the imported documentation-upkeep instructions. Compare public lesson
documentation with directly relevant Rust code changes and the current lesson
sources; propose only clear, evidence-backed documentation corrections. Use
the repository's documented workspace build check and report the exact command
and result. Do not edit Rust source, fill exercise TODOs, or change tests.
