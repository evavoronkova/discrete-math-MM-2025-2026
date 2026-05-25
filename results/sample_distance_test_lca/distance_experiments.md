# Distance estimation experiments

| Algorithm | Strategy | Landmarks | Exact match ratio | MAE | Max error | Build time (s) | Speedup |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| basic | random | 2 | 0.8000 | 0.4000 | 2 | 0.0001 | 4.07 |
| basic | highest_degree | 2 | 1.0000 | 0.0000 | 0 | 0.0000 | 6.29 |
| lca | random | 2 | 1.0000 | 0.0000 | 0 | 0.0000 | 0.73 |
| lca | highest_degree | 2 | 1.0000 | 0.0000 | 0 | 0.0000 | 1.48 |

- Best accuracy: `basic` + `highest_degree` + 2 landmarks, exact match ratio 1.0000, MAE 0.0000.
- Best speedup: `basic` + `highest_degree` + 2 landmarks, speedup 6.29x.

- `basic` is usually faster at query time, but can overestimate distances.
- `lca` uses landmark shortest-path trees and LCA queries, and is often more accurate.
- More landmarks usually improve accuracy, but increase preprocessing time.
