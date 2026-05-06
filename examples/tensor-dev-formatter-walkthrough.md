# Tensor Dev Formatter Meter Walkthrough

The fixture is intentionally compact, so the review starts with the cases that pull farthest apart.

| Case | Focus | Score | Lane |
| --- | --- | ---: | --- |
| baseline | change width | 141 | ship |
| stress | diagnostic quality | 191 | ship |
| edge | review cost | 223 | ship |
| recovery | safe rewrite | 122 | watch |
| stale | change width | 211 | ship |

Start with `edge` and `recovery`. They create the widest contrast in this repository's fixture set, which makes them better review anchors than the middle cases.

The next useful expansion would be a malformed fixture around diagnostic quality and safe rewrite.
