# Distance estimation experiments

| Algorithm | Strategy | Landmarks | Exact match ratio | MAE | Max error | Build time (s) | Speedup |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| basic | random | 8 | 0.1333 | 1.5000 | 2 | 1.3297 | 3406.07 |
| basic | random | 16 | 0.2667 | 1.2333 | 2 | 2.9287 | 1526.70 |
| basic | highest_degree | 8 | 0.6333 | 0.3667 | 1 | 0.3581 | 2261.29 |
| basic | highest_degree | 16 | 0.7000 | 0.3000 | 1 | 0.7056 | 1121.89 |
| lca | random | 8 | 0.5000 | 0.5667 | 2 | 0.3125 | 368.27 |
| lca | random | 16 | 0.7333 | 0.2667 | 1 | 0.6998 | 223.36 |
| lca | highest_degree | 8 | 0.6333 | 0.3667 | 1 | 0.3553 | 365.99 |
| lca | highest_degree | 16 | 0.7000 | 0.3000 | 1 | 0.6462 | 199.37 |

- Best accuracy: `lca` + `random` + 16 landmarks, exact match ratio 0.7333, MAE 0.2667.
- Best speedup: `basic` + `random` + 8 landmarks, speedup 3406.07x.

- `basic` is usually faster at query time, but can overestimate distances.
- `lca` uses landmark shortest-path trees and LCA queries, and is often more accurate.
- More landmarks usually improve accuracy, but increase preprocessing time.
