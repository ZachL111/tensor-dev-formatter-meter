# Review Journal

I treated `tensor-dev-formatter-meter` as a project where the smallest useful behavior should still be inspectable.

The local checks classify each case as `ship`, `watch`, or `hold`. That gives the project a small review vocabulary that matches its developer tools focus without claiming live deployment or external usage.

## Cases

- `baseline`: `change width`, score 141, lane `ship`
- `stress`: `diagnostic quality`, score 191, lane `ship`
- `edge`: `review cost`, score 223, lane `ship`
- `recovery`: `safe rewrite`, score 122, lane `watch`
- `stale`: `change width`, score 211, lane `ship`

## Note

A future change should add new cases before it changes the scoring rule.
