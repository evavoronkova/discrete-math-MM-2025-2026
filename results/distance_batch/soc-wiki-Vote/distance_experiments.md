# Distance estimation experiments

| Algorithm | Strategy | Landmarks | Exact match ratio | MAE | Max error | Build time (s) | Speedup |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| basic | random | 8 | 0.2333 | 1.2000 | 3 | 0.0146 | 304.67 |
| basic | random | 16 | 0.4667 | 0.7000 | 2 | 0.0282 | 177.73 |
| basic | highest_degree | 8 | 0.9000 | 0.1667 | 2 | 0.0138 | 293.01 |
| basic | highest_degree | 16 | 0.9000 | 0.1333 | 2 | 0.0282 | 131.80 |
| lca | random | 8 | 0.7333 | 0.2667 | 1 | 0.0138 | 37.49 |
| lca | random | 16 | 0.9667 | 0.0333 | 1 | 0.0268 | 22.13 |
| lca | highest_degree | 8 | 0.9333 | 0.1000 | 2 | 0.0158 | 36.47 |
| lca | highest_degree | 16 | 0.9667 | 0.0333 | 1 | 0.0274 | 21.13 |

- Best accuracy: `lca` + `random` + 16 landmarks, exact match ratio 0.9667, MAE 0.0333.
- Best speedup: `basic` + `random` + 8 landmarks, speedup 304.67x.

- `basic` is usually faster at query time, but can overestimate distances.
- `lca` uses landmark shortest-path trees and LCA queries, and is often more accurate.
- More landmarks usually improve accuracy, but increase preprocessing time.
