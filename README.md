# actuarly

A P&C insurance actuarial learning sim. Run a fictional insurance company (Pacific Shield Insurance) chapter by chapter, making real pricing and risk decisions, learning one actuarial concept at a time.

Each chapter is a self-contained Jupyter notebook: narrative, math, interactive controls, live charts, and pass/fail feedback. Progress by making decisions that have consequences — underprice and claims eat you alive; overprice and customers walk; diversify poorly and one bad year breaks the company.

## Chapters

| # | Title                        | Concept                  | Status      |
|---|------------------------------|--------------------------|-------------|
| 1 | Your First Policy            | Expected value           | Playable    |
| 2 | Growing the Book             | Law of large numbers     | Planned     |
| 3 | Fitting the Curve            | Loss distributions       | Planned     |
| 4 | Building the Rate Manual     | Ratemaking               | Planned     |
| 5 | The Long Tail                | Reserving & IBNR         | Planned     |
| 6 | Sharing the Risk             | Reinsurance              | Planned     |
| 7 | Storm Season                 | Catastrophe modeling     | Planned     |
| 8 | The Bottom Line              | Combined ratio           | Planned     |
| 9 | Capital Allocation           | Surplus management       | Planned     |
| 10| The Full Picture             | Stochastic modeling      | Planned     |

## Scope

- P&C (property & casualty) only — no life insurance.
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
