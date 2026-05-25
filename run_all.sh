#!/bin/bash
# run_all.sh — прогон всех назначенных файлов для участника
# Раскомментируй свои строки, остальные оставь закомментированными.

PROJECT_DIR="$(cd "$(dirname "$0")" && pwd)"
OUTDIR="$PROJECT_DIR/results"
mkdir -p "$OUTDIR"

run_file() {
    local FILE="$1"
    local EXTRA="${2:-}"          # --directed или пусто
    local NUM_PAIRS="${3:-500}"   # размер выборки для диаметра
    local NAME
    NAME=$(basename "$FILE" .txt)

    echo "=============================================="
    echo "  $NAME"
    echo "=============================================="

    # --- Часть 1 (анализ сети) + Часть B (robustness) ---
    echo "[1/2] Part 1 analysis..."
    ./gradlew run --args="\"$FILE $EXTRA --num-pairs=$NUM_PAIRS --snowball-size=$NUM_PAIRS\"" \
        > "$OUTDIR/${NAME}_part1.txt" 2>&1

    mv degree_dist.csv      "$OUTDIR/${NAME}_degree_dist.csv"      2>/dev/null
    mv degree_loglog.csv    "$OUTDIR/${NAME}_degree_loglog.csv"    2>/dev/null
    mv deletion_random.csv  "$OUTDIR/${NAME}_deletion_random.csv"  2>/dev/null
    mv deletion_high_degree.csv "$OUTDIR/${NAME}_deletion_high_degree.csv" 2>/dev/null

    # --- Часть 2 (Landmarks) K=10 ---
    echo "[2/2] Part 2 K=10..."
    ./gradlew run --args="\"$FILE $EXTRA --part2 --compare-landmarks --landmarks=10\"" \
        > "$OUTDIR/${NAME}_part2_k10.txt" 2>&1

    # --- Часть 2 (Landmarks) K=20 ---
    echo "[2/2] Part 2 K=20..."
    ./gradlew run --args="\"$FILE $EXTRA --part2 --compare-landmarks --landmarks=20\"" \
        > "$OUTDIR/${NAME}_part2_k20.txt" 2>&1

    echo ""
}

# ============================================================
#  НИКИТА (8 GB, Ryzen 5) — раскомментируй этот блок
# ============================================================
# run_file "datasets/undirected/CA-GrQc.txt"
# run_file "datasets/undirected/musae_git.txt"
# run_file "datasets/undirected/CA-AstroPh.txt"
# run_file "datasets/undirected/Email-EuAll.txt"
# run_file "datasets/directed/Wiki-Vote.txt"           "--directed"
# run_file "datasets/directed/soc-wiki-Vote.txt"       "--directed"
# run_file "datasets/directed/web-NotreDame.txt"       "--directed"
# run_file "datasets/directed/web-Stanford.txt"        "--directed"

# ============================================================
#  ГРИША (16 GB, i7-1355U) — раскомментируй этот блок
# ============================================================
# run_file "datasets/very_large_graphs/com-youtube.ungraph.txt"
# run_file "datasets/directed/web-Google.txt"           "--directed"
# run_file "datasets/undirected/ca-coauthors-dblp.txt"

# ============================================================
#  АНДРЕЙ (32 GB, i9-13900H) — раскомментируй этот блок
# ============================================================
# run_file "datasets/very_large_graphs/com-orkut.ungraph.txt"  ""  "200"
# run_file "datasets/very_large_graphs/vk.txt"                 ""  "200"

echo "Done. Results saved to $OUTDIR/"
