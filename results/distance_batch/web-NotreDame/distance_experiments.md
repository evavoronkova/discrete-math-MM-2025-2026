# Distance estimation experiments

| Algorithm | Strategy | Landmarks | Exact match ratio | MAE | Max error | Build time (s) | Speedup |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| basic | random | 8 | 0.1333 | 4.3333 | 8 | 23.8034 | 45706.41 |
| basic | random | 16 | 0.2667 | 2.7000 | 7 | 97.4436 | 18986.12 |
| basic | highest_degree | 8 | 0.4333 | 0.9333 | 8 | 22.6974 | 61480.01 |
| basic | highest_degree | 16 | 0.9000 | 0.1333 | 2 | 144.4753 | 31744.56 |
| lca | random | 8 | 0.9333 | 0.0667 | 1 | 69.1960 | 8170.33 |
| lca | random | 16 | 1.0000 | 0.0000 | 0 | 100.5882 | 4024.72 |
| lca | highest_degree | 8 | 0.6000 | 0.4000 | 1 | 22.1119 | 7210.61 |
| lca | highest_degree | 16 | 0.9333 | 0.0667 | 1 | 113.4735 | 5334.97 |

- Best accuracy: `lca` + `random` + 16 landmarks, exact match ratio 1.0000, MAE 0.0000.
- Best speedup: `basic` + `highest_degree` + 8 landmarks, speedup 61480.01x.

- `basic` is usually faster at query time, but can overestimate distances.
- `lca` uses landmark shortest-path trees and LCA queries, and is often more accurate.
- More landmarks usually improve accuracy, but increase preprocessing time.
