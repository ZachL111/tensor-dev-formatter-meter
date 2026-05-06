# tensor-dev-formatter-meter

`tensor-dev-formatter-meter` explores developer tools with a small Rust codebase and local fixtures. The technical goal is to build a Rust toolkit that studies formatter behavior through fixture event logs, with golden state snapshots and no network dependency.

## Why It Exists

This is intentionally local and self-contained so it can be inspected without credentials, services, or seeded history.

## Tensor Dev Formatter Meter Review Notes

`edge` and `recovery` are the cases worth reading first. They show the optimistic and cautious ends of the fixture.

## Features

- `fixtures/domain_review.csv` adds cases for change width and diagnostic quality.
- `metadata/domain-review.json` records the same cases in structured form.
- `config/review-profile.json` captures the read order and the two review questions.
- `examples/tensor-dev-formatter-walkthrough.md` walks through the case spread.
- The Rust code includes a review path for `review cost` and `safe rewrite`.
- `docs/field-notes.md` explains the strongest and weakest cases.

## Architecture Notes

The repository has two validation layers: the original compact policy fixture and the domain review fixture. They are separate so one can change without hiding failures in the other.

The Rust implementation avoids hidden state so fixture changes are easy to reason about.

## Usage

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

## Tests

That command is also the regression path. It verifies the domain cases and catches mismatches between the CSV, metadata, and code.

## Limitations And Roadmap

This remains a local project with deterministic fixtures. It does not depend on credentials, hosted services, or live data. Future work should add richer malformed inputs before widening the public API.
