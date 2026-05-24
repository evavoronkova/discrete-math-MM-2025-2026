from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parent
SRC = ROOT / "src"
if str(SRC) not in sys.path:
    sys.path.insert(0, str(SRC))

from network_analysis.datasets import DATASETS
from network_analysis.io_utils import detect_directed_from_name, load_edge_list
from network_analysis.report import build_report


def main() -> int:
    data_dir = ROOT / "data"
    results_dir = ROOT / "results"
    for dataset in DATASETS:
        input_path = data_dir / dataset["filename"]
        if not input_path.exists():
            continue
        directed = dataset.get("directed", detect_directed_from_name(input_path))
        output_dir = results_dir / input_path.stem
        graph = load_edge_list(
            input_path,
            directed=directed,
            file_format=dataset.get("format", "auto"),
        )
        build_report(
            graph=graph,
            output_dir=output_dir,
            distance_sample_size=500,
            snowball_size=500,
            attack_steps=[0, 5, 10, 15, 20, 25, 30, 40, 50],
            seed=42,
        )
        print(f"Completed: {input_path.name} -> {output_dir}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
