# Distance estimation experiments

| Algorithm | Strategy | Landmarks | Exact match ratio | MAE | Max error | Build time (s) | Speedup |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| basic | random | 8 | 0.1333 | 1.6667 | 3 | 10.3241 | 13831.09 |
| basic | random | 16 | 0.2667 | 1.2667 | 3 | 10.1007 | 7610.57 |
| basic | highest_degree | 8 | 0.8000 | 0.2000 | 1 | 2.7614 | 12319.43 |
| basic | highest_degree | 16 | 0.8333 | 0.1667 | 1 | 5.2874 | 6158.46 |
| lca | random | 8 | 0.6333 | 0.3667 | 1 | 2.2621 | 1877.47 |
| lca | random | 16 | 0.8000 | 0.2000 | 1 | 4.4163 | 1093.90 |
| lca | highest_degree | 8 | 0.8333 | 0.1667 | 1 | 2.2544 | 1997.69 |
| lca | highest_degree | 16 | 0.8667 | 0.1333 | 1 | 4.5983 | 1109.68 |

- Best accuracy: `lca` + `highest_degree` + 16 landmarks, exact match ratio 0.8667, MAE 0.1333.
- Best speedup: `basic` + `random` + 8 landmarks, speedup 13831.09x.

- `basic` is usually faster at query time, but can overestimate distances.
- `lca` uses landmark shortest-path trees and LCA queries, and is often more accurate.
- More landmarks usually improve accuracy, but increase preprocessing time.
