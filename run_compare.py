from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parent
SRC = ROOT / "src"
if str(SRC) not in sys.path:
    sys.path.insert(0, str(SRC))

from network_analysis.comparison import save_comparison_artifacts


def main() -> int:
    summary = save_comparison_artifacts(ROOT / "results")
    print(f"Compared datasets: {len(summary['datasets'])}")
    print(f"Saved: {ROOT / 'results' / 'comparison_summary.json'}")
    print(f"Saved: {ROOT / 'results' / 'comparison_report.md'}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
