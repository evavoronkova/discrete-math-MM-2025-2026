#!/usr/bin/env python3
"""plots.py — построение графиков по CSV-файлам из results/"""
import pandas as pd
import matplotlib.pyplot as plt
import glob, os

RESULTS = "results"

datasets = sorted(set(
    f.replace('_deletion_random.csv','').replace('_deletion_high_degree.csv','')
    .replace('_degree_dist.csv','').replace('_degree_loglog.csv','')
    for f in glob.glob(f'{RESULTS}/*.csv')
))

if not datasets:
    print("Нет CSV-файлов в results/. Сначала запусти run_all.sh.")
    exit(1)

# --- График 1: Устойчивость к удалению узлов (слайд 8) ---
fig, axes = plt.subplots(1, len(datasets), figsize=(5*len(datasets), 4))
if len(datasets) == 1:
    axes = [axes]

for ax, name in zip(axes, datasets):
    rand_file = f'{RESULTS}/{name}_deletion_random.csv'
    high_file = f'{RESULTS}/{name}_deletion_high_degree.csv'
    if os.path.exists(rand_file):
        rand = pd.read_csv(rand_file)
        ax.plot(rand['percent_deleted'], rand['largest_component_fraction'],
                'o-', label='Random', linewidth=2)
    if os.path.exists(high_file):
        high = pd.read_csv(high_file)
        ax.plot(high['percent_deleted'], high['largest_component_fraction'],
                's-', label='High-degree', linewidth=2)
    ax.set_xlabel('% deleted nodes')
    ax.set_ylabel('Largest component fraction')
    ax.set_title(name[:25])
    ax.legend()
    ax.grid(True, alpha=0.3)

plt.tight_layout()
plt.savefig('robustness.png', dpi=150, bbox_inches='tight')
print("Сохранено: robustness.png")

# --- График 2: Log-log распределение степеней (слайд 7) ---
plt.figure(figsize=(8, 6))
for name in datasets:
    loglog_file = f'{RESULTS}/{name}_degree_loglog.csv'
    if os.path.exists(loglog_file):
        df = pd.read_csv(loglog_file)
        plt.plot(df['log_degree'], df['log_probability'], 'o', alpha=0.4,
                 markersize=3, label=name[:25])
plt.xlabel('ln(degree)')
plt.ylabel('ln(probability)')
plt.title('Degree distribution (log-log)')
plt.legend(fontsize=7)
plt.grid(True, alpha=0.3)
plt.savefig('degree_loglog.png', dpi=150, bbox_inches='tight')
print("Сохранено: degree_loglog.png")

# --- График 3: Точность Landmarks (слайд 10, если есть результаты Part 2) ---
# Строится вручную по таблице из build_report.py

print("Готово. Графики: robustness.png, degree_loglog.png")
