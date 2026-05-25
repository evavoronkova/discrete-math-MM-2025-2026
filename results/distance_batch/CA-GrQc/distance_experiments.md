# Distance estimation experiments

| Algorithm | Strategy | Landmarks | Exact match ratio | MAE | Max error | Build time (s) | Speedup |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| basic | random | 8 | 0.1333 | 3.5000 | 6 | 0.0949 | 484.05 |
| basic | random | 16 | 0.2667 | 1.9667 | 5 | 0.1935 | 424.21 |
| basic | highest_degree | 8 | 0.0667 | 1.9333 | 5 | 0.1044 | 733.66 |
| basic | highest_degree | 16 | 0.0667 | 1.6667 | 3 | 0.1664 | 471.70 |
| lca | random | 8 | 0.7000 | 0.5000 | 3 | 0.0812 | 101.73 |
| lca | random | 16 | 0.8333 | 0.2333 | 2 | 0.1825 | 57.53 |
| lca | highest_degree | 8 | 0.1667 | 1.7000 | 5 | 0.0891 | 87.18 |
| lca | highest_degree | 16 | 0.2000 | 1.3667 | 3 | 0.1813 | 49.58 |

- Best accuracy: `lca` + `random` + 16 landmarks, exact match ratio 0.8333, MAE 0.2333.
- Best speedup: `basic` + `highest_degree` + 8 landmarks, speedup 733.66x.

- `basic` is usually faster at query time, but can overestimate distances.
- `lca` uses landmark shortest-path trees and LCA queries, and is often more accurate.
- More landmarks usually improve accuracy, but increase preprocessing time.
