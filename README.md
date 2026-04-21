# actuarly

A cyber insurance actuarial learning sim. Run a fictional company (Arclight Cyber Insurance) chapter by chapter, making real pricing and risk decisions, learning one actuarial concept at a time.

Each chapter is a self-contained Jupyter notebook: narrative, math, interactive controls, live charts, and pass/fail feedback. Progress by making decisions that have consequences — underprice and ransomware claims eat you alive; overprice and clients walk; model correlation poorly and one systemic attack breaks the company.

## Chapters

| # | Title                        | Concept                          | Status      |
|---|------------------------------|----------------------------------|-------------|
| 1 | Your First Policy            | Expected value                   | Playable    |
| 2 | Growing the Book             | Law of large numbers             | Playable    |
| 3 | Fitting the Curve            | Loss distributions (heavy tails) | Playable    |
| 4 | Building the Rate Manual     | Cyber ratemaking & controls      | Playable    |
| 5 | The Long Tail                | Reserving & IBNR                 | Playable    |
| 6 | Sharing the Risk             | Cyber reinsurance                | Planned     |
| 7 | Systemic Attack Season       | Cyber catastrophe modeling       | Planned     |
| 8 | The Bottom Line              | Combined ratio                   | Planned     |
| 9 | Capital Allocation           | Surplus management               | Planned     |
| 10| The Full Picture             | Stochastic cyber modeling        | Planned     |

## Scope

- Cyber insurance only — standalone cyber liability and first-party coverage.
- Compliance and regulatory detail is out of scope; the sim assumes a legal department handles it so the player can focus on actuarial judgment.
- Pedagogical, not production. Numbers are tuned for clarity, not industry realism.

## Running

Requires Python 3.12+ and [uv](https://docs.astral.sh/uv/).

```bash
uv sync
uv run jupyter lab notebooks/chapter1.ipynb
```

## Project layout

- `notebooks/` — the sim, one notebook per chapter
- `src/` — an earlier Rust/TUI prototype of chapters 1–2, kept as a reference while the notebook version is built out

## License

MIT. See [LICENSE](LICENSE).
