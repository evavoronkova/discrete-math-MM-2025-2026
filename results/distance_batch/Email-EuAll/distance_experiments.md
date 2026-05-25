# Distance estimation experiments

| Algorithm | Strategy | Landmarks | Exact match ratio | MAE | Max error | Build time (s) | Speedup |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| basic | random | 8 | 0.1333 | 2.2667 | 4 | 12.7616 | 23456.45 |
| basic | random | 16 | 0.2667 | 1.8000 | 3 | 35.9452 | 15473.43 |
| basic | highest_degree | 8 | 0.3000 | 1.0333 | 2 | 33.1335 | 25259.73 |
| basic | highest_degree | 16 | 0.4667 | 0.6000 | 2 | 82.6021 | 14721.76 |
| lca | random | 8 | 0.4000 | 0.7667 | 2 | 40.3782 | 5192.77 |
| lca | random | 16 | 0.7000 | 0.3667 | 2 | 83.7540 | 3102.37 |
| lca | highest_degree | 8 | 0.5333 | 0.5333 | 2 | 15.6743 | 3864.42 |
| lca | highest_degree | 16 | 0.7000 | 0.3000 | 1 | 38.4412 | 2784.05 |

- Best accuracy: `lca` + `highest_degree` + 16 landmarks, exact match ratio 0.7000, MAE 0.3000.
- Best speedup: `basic` + `highest_degree` + 8 landmarks, speedup 25259.73x.

- `basic` is usually faster at query time, but can overestimate distances.
- `lca` uses landmark shortest-path trees and LCA queries, and is often more accurate.
- More landmarks usually improve accuracy, but increase preprocessing time.
