#!/usr/bin/env python3
"""build_report.py — парсинг results/*.txt и построение итоговых таблиц для презентации."""
import re, glob, os
from collections import defaultdict

RESULTS = "results"

def read(name):
    path = os.path.join(RESULTS, name)
    if os.path.exists(path):
        return open(path).read()
    return ""

def parse_basic(text):
    m = re.search(r'Vertices:\s*(\d+).*?Edges:\s*(\d+).*?Density:\s*([\d.]+).*?'
                  r'Weak components:\s*(\d+).*?Largest weak.*?:\s*([\d.]+)',
                  text, re.DOTALL)
    if not m: return None
    return dict(vertices=int(m[1]), edges=int(m[2]), density=float(m[3]),
                wcc=int(m[4]), wcc_frac=float(m[5]))

def parse_diameter(text):
    m = re.search(r'Double sweep diameter:\s*(\d+).*?'
                  r'Random pairs 90th.*?:\s*(\d+).*?diameter:\s*(\d+).*?'
                  r'Snowball 90th.*?:\s*(\d+).*?diameter:\s*(\d+)',
                  text, re.DOTALL)
    if not m: return None
    return dict(ds_diam=int(m[1]), rp_p90=int(m[2]), rp_diam=int(m[3]),
                sn_p90=int(m[4]), sn_diam=int(m[5]))

def parse_clustering(text):
    m = re.search(r'Triangle count:\s*(\d+).*?'
                  r'Average clustering.*?:\s*([\d.]+).*?'
                  r'Global clustering.*?:\s*([\d.]+)',
                  text, re.DOTALL)
    if not m: return None
    return dict(triangles=int(m[1]), avg_cluster=float(m[2]), global_cluster=float(m[3]))

def parse_degree(text):
    m = re.search(r'Min degree:\s*(\d+).*?Max degree:\s*(\d+).*?Average degree:\s*([\d.]+)',
                  text, re.DOTALL)
    if not m: return None
    return dict(min_deg=int(m[1]), max_deg=int(m[2]), avg_deg=float(m[3]))

def parse_landmarks(text, k):
    rows = []
    for line in text.split('\n'):
        line = line.strip()
        if '|' not in line or 'AvgError' in line or '---' in line:
            continue
        parts = [p.strip() for p in line.split('|')]
        if len(parts) < 6:
            continue
        try:
            algo = parts[0]; strategy = parts[1]
            avg_err = float(parts[2]); max_err = int(parts[3])
            exact_pct = float(parts[4].replace('%',''))
            prep_ms = float(parts[5])
            rows.append(dict(algo=algo, strategy=strategy, K=k,
                             avg_error=avg_err, max_error=max_err,
                             exact_rate=exact_pct, prep_ms=prep_ms))
        except (ValueError, IndexError):
            continue
    return rows

# --- Сбор имён файлов ---
names = sorted(set(
    f.replace('_part1.txt','').replace('_part2_k10.txt','').replace('_part2_k20.txt','')
    for f in glob.glob(f'{RESULTS}/*_part*.txt')
))

if not names:
    print("Сначала запусти run_all.sh и скопируй results/ в эту папку.")
    exit(1)

# --- ТАБЛИЦА 1: Базовые характеристики (слайд 5) ---
print("=" * 85)
print("ТАБЛИЦА 1 — Базовые характеристики сети")
print("=" * 85)
print(f"{'Сеть':<25} {'V':>10} {'E':>12} {'Плотность':>10} {'WCC':>5} {'WCC%':>8}")
print("-" * 85)
for name in names:
    bs = parse_basic(read(f'{name}_part1.txt'))
    if bs:
        print(f"{name:<25} {bs['vertices']:>10} {bs['edges']:>12} "
              f"{bs['density']:>10.8f} {bs['wcc']:>5} {bs['wcc_frac']:>8.4f}")

# --- ТАБЛИЦА 2: Диаметр (слайд 6) ---
print("\n" + "=" * 75)
print("ТАБЛИЦА 2 — Оценка диаметра и 90-й перцентиль")
print("=" * 75)
print(f"{'Сеть':<25} {'DS diam':>8} {'RP 90%':>8} {'SN 90%':>8}")
print("-" * 75)
for name in names:
    dm = parse_diameter(read(f'{name}_part1.txt'))
    if dm:
        print(f"{name:<25} {dm['ds_diam']:>8} {dm['rp_p90']:>8} {dm['sn_p90']:>8}")

# --- ТАБЛИЦА 3: Кластеризация + степени (слайд 7) ---
print("\n" + "=" * 95)
print("ТАБЛИЦА 3 — Кластерные коэффициенты и степени")
print("=" * 95)
print(f"{'Сеть':<25} {'Треуг.':>10} {'AvgClust':>10} {'GlobClust':>10} {'MinDeg':>7} {'MaxDeg':>8} {'AvgDeg':>8}")
print("-" * 95)
for name in names:
    cl = parse_clustering(read(f'{name}_part1.txt'))
    dg = parse_degree(read(f'{name}_part1.txt'))
    if cl and dg:
        print(f"{name:<25} {cl['triangles']:>10} {cl['avg_cluster']:>10.6f} "
              f"{cl['global_cluster']:>10.6f} {dg['min_deg']:>7} {dg['max_deg']:>8} {dg['avg_deg']:>8.2f}")

# --- ТАБЛИЦА 4: Landmarks (слайд 10) ---
print("\n" + "=" * 95)
print("ТАБЛИЦА 4 — Точность Landmarks (K=10,20)")
print("=" * 95)

all_lm = []
for name in names:
    for k in [10, 20]:
        rows = parse_landmarks(read(f'{name}_part2_k{k}.txt'), k)
        for r in rows:
            r['file'] = name
            all_lm.append(r)

if all_lm:
    print(f"{'Файл':<22} {'Алгоритм':<16} {'Стратегия':<12} {'K':>3} {'AvgErr':>8} {'MaxErr':>7} {'Exact%':>8}")
    print("-" * 95)
    for r in sorted(all_lm, key=lambda x: (x['file'], x['K'], x['algo'])):
        print(f"{r['file']:<22} {r['algo']:<16} {r['strategy']:<12} {r['K']:>3} "
              f"{r['avg_error']:>8.3f} {r['max_error']:>7} {r['exact_rate']:>7.1f}%")
else:
    print("  (нет данных — запусти run_all.sh с флагом --part2)")

# --- Итоги ---
print(f"\nВсе CSV-файлы лежат в {RESULTS}/")
print("Для графиков запусти:  python plots.py")
