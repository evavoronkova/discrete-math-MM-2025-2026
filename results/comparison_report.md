# Network comparison (task 1C)

## Summary table

| Network | Directed | Nodes | Edges | Density | Weak components | Largest weak share | Mean degree | Avg clustering | Diameter (double sweep) | P90 |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| CA-AstroPh | no | 18771 | 198050 | 0.001124 | 289 | 0.9538 | 21.1017 | 0.6306 | 14 | 6.00 |
| ca-coauthors-dblp | no | 540486 | 15245729 | 0.000104 | 1 | 1.0000 | 56.4149 | 0.8019 | 23 | 7.00 |
| CA-GrQc | no | 5241 | 14484 | 0.001055 | 354 | 0.7934 | 5.5272 | 0.5297 | 15 | 8.00 |
| com-youtube.ungraph | no | 1134890 | 2987624 | 0.000005 | 1 | 1.0000 | 5.2650 | 0.0808 | 24 | 7.00 |
| Email-EuAll | no | 265009 | 364481 | 0.000010 | 15631 | 0.8484 | 2.7507 | 0.0671 | 14 | 5.00 |
| musae_git_edges | no | 37700 | 289003 | 0.000407 | 1 | 1.0000 | 15.3317 | 0.1675 | 11 | 4.00 |
| soc-wiki-Vote | yes | 889 | 2914 | 0.003691 | 1 | 1.0000 | 6.5557 | 0.1528 | 13 | 6.00 |
| vk | no | 3215720 | 17414510 | 0.000003 | 24337 | 0.9834 | 10.8309 | 0.0494 | 19 | 7.00 |
| web-Google | yes | 875713 | 5105039 | 0.000007 | 2746 | 0.9773 | 9.8709 | 0.5143 | 24 | 8.00 |
| web-NotreDame | yes | 325729 | 1469679 | 0.000014 | 1 | 1.0000 | 6.6933 | 0.2346 | 46 | 10.00 |
| web-Stanford | yes | 281903 | 2312497 | 0.000029 | 365 | 0.9055 | 14.1370 | 0.5976 | 164 | 10.00 |
| Wiki-Vote | yes | 7115 | 103689 | 0.002049 | 24 | 0.9931 | 28.3238 | 0.1409 | 7 | 4.00 |

## Observations

- Largest network by node count: vk (3215720 nodes).
- Highest density: soc-wiki-Vote (0.003691); this indicates a larger share of possible local ties.
- Highest average clustering coefficient: ca-coauthors-dblp (0.8019).
- Largest double-sweep diameter estimate: web-Stanford (164).
- Most fragmented network by weak component count: vk (24337 components).
- Under random node removal, ca-coauthors-dblp keeps the largest weak component best (0.9296 after removing 50% of nodes).
- Under targeted removal by highest degree, com-youtube.ungraph is damaged the most (largest weak component share 0.0000).

## Robustness

The comparison uses the share of nodes in the largest weak component after the last removal step.

- CA-AstroPh: random removal -> 0.8490, highest-degree removal -> 0.0076.
- ca-coauthors-dblp: random removal -> 0.9296, highest-degree removal -> 0.7494.
- CA-GrQc: random removal -> 0.4945, highest-degree removal -> 0.0023.
- com-youtube.ungraph: random removal -> 0.5942, highest-degree removal -> 0.0000.
- Email-EuAll: random removal -> 0.4351, highest-degree removal -> 0.0000.
- musae_git_edges: random removal -> 0.8842, highest-degree removal -> 0.0007.
- soc-wiki-Vote: random removal -> 0.8135, highest-degree removal -> 0.0135.
- vk: random removal -> 0.8048, highest-degree removal -> 0.0000.
- web-Google: random removal -> 0.7193, highest-degree removal -> 0.0001.
- web-NotreDame: random removal -> 0.4515, highest-degree removal -> 0.0002.
- web-Stanford: random removal -> 0.6329, highest-degree removal -> 0.0054.
- Wiki-Vote: random removal -> 0.8001, highest-degree removal -> 0.0008.

## Interpretation

- Higher density and clustering usually indicate denser local communities.
- Larger diameter and P90 distance indicate longer paths between typical vertex pairs.
- A sharp drop under targeted removal shows that the network depends on a small set of high-degree hubs.
