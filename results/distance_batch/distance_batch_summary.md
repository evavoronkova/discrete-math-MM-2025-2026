# Distance estimation batch summary

| Dataset | Best accuracy setup | Exact match | MAE | Best speed setup | Speedup |
| --- | --- | ---: | ---: | --- | ---: |
| CA-GrQc | lca / random / 16 | 0.8333 | 0.2333 | basic / highest_degree / 8 | 733.66 |
| CA-AstroPh | lca / highest_degree / 16 | 0.5000 | 0.5000 | basic / random / 8 | 8110.83 |
| Email-EuAll | lca / highest_degree / 16 | 0.7000 | 0.3000 | basic / highest_degree / 8 | 25259.73 |
| musae_git_edges | lca / highest_degree / 16 | 0.8667 | 0.1333 | basic / random / 8 | 13831.09 |
| soc-wiki-Vote | lca / random / 16 | 0.9667 | 0.0333 | basic / random / 8 | 304.67 |
| web-NotreDame | lca / random / 16 | 1.0000 | 0.0000 | basic / highest_degree / 8 | 61480.01 |
| Wiki-Vote | lca / random / 16 | 0.7333 | 0.2667 | basic / random / 8 | 3406.07 |

## Conclusions

- Best accuracy overall: web-NotreDame / lca / random / 16 landmarks, exact match = 1.0000, MAE = 0.0000.
- Best speedup overall: web-NotreDame / basic / highest_degree / 8 landmarks, speedup = 61480.01x.
- `basic` is usually faster at query time, but can overestimate distances.
- `lca` often improves accuracy because it uses landmark shortest-path trees and LCA queries.
- More landmarks usually improve accuracy, but increase preprocessing time.
