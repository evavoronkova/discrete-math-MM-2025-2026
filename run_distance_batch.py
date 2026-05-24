from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parent
SRC = ROOT / "src"
if str(SRC) not in sys.path:
    sys.path.insert(0, str(SRC))

from network_analysis.distance_batch import run_distance_batch


def main() -> int:
    summary = run_distance_batch(
        root_dir=ROOT,
        algorithms=["basic", "bfs"],
        strategies=["random", "highest_degree", "coverage"],
        landmark_counts=[8, 16],
        pair_count=30,
        seed=42,
        dataset_names=["CA-GrQc", "CA-AstroPh", "soc-wiki-Vote", "musae_git_edges"],
        skip_existing=True,
    )
    print(f"Processed datasets: {summary['dataset_count']}")
    print(f"Saved: {ROOT / 'results' / 'distance_batch' / 'distance_batch_summary.json'}")
    print(f"Saved: {ROOT / 'results' / 'distance_batch' / 'distance_batch_summary.md'}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
