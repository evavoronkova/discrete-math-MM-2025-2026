# Distance estimation experiments

| Algorithm | Strategy | Landmarks | Exact match ratio | MAE | Max error | Build time (s) | Speedup |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| basic | random | 8 | 0.1333 | 1.6000 | 3 | 1.3027 | 8110.83 |
| basic | random | 16 | 0.2667 | 1.2333 | 3 | 1.9115 | 3339.57 |
| basic | highest_degree | 8 | 0.4000 | 0.6333 | 2 | 0.9338 | 5941.59 |
| basic | highest_degree | 16 | 0.5000 | 0.5333 | 2 | 2.1210 | 3360.01 |
| lca | random | 8 | 0.1667 | 1.2000 | 3 | 1.1008 | 1007.59 |
| lca | random | 16 | 0.3667 | 0.8000 | 2 | 2.0671 | 548.16 |
| lca | highest_degree | 8 | 0.4000 | 0.6333 | 2 | 1.0887 | 1025.01 |
| lca | highest_degree | 16 | 0.5000 | 0.5000 | 1 | 2.0051 | 555.83 |

- Best accuracy: `lca` + `highest_degree` + 16 landmarks, exact match ratio 0.5000, MAE 0.5000.
- Best speedup: `basic` + `random` + 8 landmarks, speedup 8110.83x.

- `basic` is usually faster at query time, but can overestimate distances.
- `lca` uses landmark shortest-path trees and LCA queries, and is often more accurate.
- More landmarks usually improve accuracy, but increase preprocessing time.
