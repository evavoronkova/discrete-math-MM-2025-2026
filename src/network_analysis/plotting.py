import csv
from pathlib import Path


def plot_degree_distribution(distribution: dict[int, float], output_path: str | Path, loglog: bool = False) -> None:
    output = Path(output_path)
    output.parent.mkdir(parents=True, exist_ok=True)
    x_values = list(distribution.keys())
    y_values = list(distribution.values())

    try:
        import matplotlib.pyplot as plt
    except ModuleNotFoundError:
        csv_path = output.with_suffix(".csv")
        with csv_path.open("w", encoding="utf-8", newline="") as handle:
            writer = csv.writer(handle)
            writer.writerow(["degree", "probability", "scale"])
            scale = "loglog" if loglog else "linear"
            for degree, probability in zip(x_values, y_values):
                writer.writerow([degree, probability, scale])
        return

    plt.figure(figsize=(8, 5))
    if loglog:
        filtered = [(x, y) for x, y in zip(x_values, y_values) if x > 0 and y > 0]
        x_values = [x for x, _ in filtered]
        y_values = [y for _, y in filtered]
        plt.loglog(x_values, y_values, marker="o", linestyle="None")
        plt.title("Degree Distribution (log-log)")
    else:
        plt.plot(x_values, y_values, marker="o", linestyle="-")
        plt.title("Degree Distribution")
    plt.xlabel("Degree")
    plt.ylabel("Probability")
    plt.grid(True, alpha=0.3)
    plt.tight_layout()
    plt.savefig(output, dpi=150)
    plt.close()
