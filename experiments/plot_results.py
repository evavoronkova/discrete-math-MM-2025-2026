# построение всех графиков
import os

import matplotlib.pyplot as plt

def plot_degree_distribution(dist, dataset_name, output_dir="results"):
    if not dist:
        return
    os.makedirs(output_dir, exist_ok=True)
    degrees = sorted(dist.keys())
    probs = [dist[k] for k in degrees]

    # обычный график
    plt.figure()
    plt.plot(degrees, probs, "o-", markersize=3)
    plt.xlabel("степень")
    plt.ylabel("доля вершин")
    plt.title(f"Степенное распределение – {dataset_name}")
    plt.grid(True)
    plt.savefig(os.path.join(output_dir, "degree_distribution.png"))
    plt.close()

    # log-log
    plt.figure()
    plt.loglog(degrees, probs, "o-", markersize=3)
    plt.xlabel("степень (log)")
    plt.ylabel("доля вершин (log)")
    plt.title(f"Степенное распределение (log-log) – {dataset_name}")
    plt.grid(True)
    plt.savefig(os.path.join(output_dir, "degree_distribution_loglog.png"))
    plt.close()

def plot_robustness(rand_res, deg_res, name="robustness", output_dir="results"):
    os.makedirs(output_dir, exist_ok=True)
    p_vals = [x[0] for x in rand_res]
    rand_frac = [x[1] for x in rand_res]
    deg_frac = [x[1] for x in deg_res]

    plt.figure()
    plt.plot(p_vals, rand_frac, "o-", label="Случайное удаление")
    plt.plot(p_vals, deg_frac, "s-", label="Удаление по степени")
    plt.xlabel("Процент удалённых вершин")
    plt.ylabel("Доля вершин в наибольшей компоненте")
    plt.title(f"Устойчивость – {name}")
    plt.legend()
    plt.grid(True)
    plt.savefig(os.path.join(output_dir, "robustness.png"))
    plt.close()

def plot_landmarks_results(
    graph_name,
    ks,
    strategies,
    basic_mre,
    sc_mre,
    basic_exact_frac,
    sc_exact_frac,
    basic_query_times,
    sc_query_times,
    basic_prep_times,
    sc_prep_times,
    output_dir="results",
):
    os.makedirs(output_dir, exist_ok=True)

    # один цвет на стратегию, чтобы пара Basic+SC одной стратегии
    # визуально читалась как пара; разные маркеры — на случай ч/б печати;
    # сплошная линия — Basic, пунктир — SC
    markers = {"random": "o", "degree": "s", "coverage": "^"}
    colors = {"random": "C0", "degree": "C1", "coverage": "C2"}

    def plot_pair(ax, dict_basic, dict_sc):
        for strat in strategies:
            y_basic = [dict_basic[(k, strat)] for k in ks]
            y_sc = [dict_sc[(k, strat)] for k in ks]
            ax.plot(ks, y_basic, marker=markers[strat], linestyle="-",
                    color=colors[strat], label=f"Basic {strat}")
            ax.plot(ks, y_sc, marker=markers[strat], linestyle="--",
                    color=colors[strat], label=f"SC {strat}")

    fig, axes = plt.subplots(2, 2, figsize=(12, 10))

    ax = axes[0, 0]
    plot_pair(ax, basic_mre, sc_mre)
    ax.set_title("Средняя относительная ошибка")
    ax.set_xlabel("Число ориентиров")
    ax.set_ylabel("MRE")
    ax.legend()
    ax.grid(True)

    ax = axes[0, 1]
    plot_pair(ax, basic_exact_frac, sc_exact_frac)
    ax.set_title("Доля точных оценок")
    ax.set_xlabel("Число ориентиров")
    ax.set_ylabel("Доля")
    ax.legend()
    ax.grid(True)

    ax = axes[1, 0]
    plot_pair(ax, basic_query_times, sc_query_times)
    ax.set_title("Суммарное время запросов")
    ax.set_xlabel("Число ориентиров")
    ax.set_ylabel("Время (с)")
    ax.legend()
    ax.grid(True)

    ax = axes[1, 1]
    plot_pair(ax, basic_prep_times, sc_prep_times)
    ax.set_title("Время препроцессинга")
    ax.set_xlabel("Число ориентиров")
    ax.set_ylabel("Время (с)")
    ax.legend()
    ax.grid(True)

    plt.suptitle(f"Анализ алгоритмов Landmarks – {graph_name}")
    plt.tight_layout()
    plt.savefig(os.path.join(output_dir, "landmarks.png"))
    plt.close()
