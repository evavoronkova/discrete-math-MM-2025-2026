# discrete-math-MM-2025-2026

Групповой проект по курсу «Теория графов и её приложения», 2026. Команда №7.

## О проекте

Исследование структуры социальных графов и реализация приближённого алгоритма оценки кратчайших расстояний на основе ориентиров (Landmarks).

**Возможности:**
- Анализ структуры сети: плотность, диаметр, кластеризация, распределение степеней
- Эксперименты по устойчивости: случайное и направленное удаление узлов
- Оценка расстояний через Landmarks-Basic и Landmarks-LCA
- Сравнение стратегий выбора ориентиров (RANDOM, HIGH_DEGREE, COVERAGE)

## Быстрый старт

```bash
# Сборка
./gradlew build

# Базовый анализ графа
./gradlew run --args="путь/к/графу.txt"

# Анализ + сравнение Landmarks (K=10)
./gradlew run --args="путь/к/графу.txt --part2 --compare-landmarks --landmarks=10"

# Только Landmarks K=20 (Part 1 уже посчитан ранее)
./gradlew run --args="путь/к/графу.txt --skip-part1 --part2 --compare-landmarks --landmarks=20"

# Орграф, 100 случайных пар, без кластеризации
./gradlew run --args="путь/к/графу.txt --directed --num-pairs=100 --skip-clustering"
```

## Формат входного файла

Edge list: два целых числа через пробел или табуляцию на каждой строке.

```
0 1
1 2
0 3
2 4
```

При загрузке автоматически удаляются: `\r`, петли (`u == v`), дубликаты рёбер. Строки `#` пропускаются. Требуется `bash` (Linux/macOS/WSL).

## Интерфейс командной строки

```
./gradlew run --args="<граф> [флаги]"
```

### Часть 1 — Анализ сети (по умолчанию)

Выполняется всегда, если не указан `--skip-part1`.

| Что вычисляется | Описание |
|-----------------|----------|
| A.1 Базовые характеристики | V, E, плотность, компоненты слабой/сильной связности |
| A.2 Оценка диаметра | Double Sweep, Random Pairs (90-й перцентиль), Snowball |
| A.3–4 Кластеризация | Число треугольников, средний и глобальный кластерный коэффициент |
| A.5 Степени | Min/avg/max степень, распределение в обычной и log-log шкале |
| B.1–2 Устойчивость | Случайное и high-degree удаление 0–50% узлов |

### Часть 2 — Landmarks (опционально)

Включается флагом `--part2` или `--compare-landmarks`.

| Алгоритм | Оценка расстояния |
|----------|------------------|
| Landmarks-Basic | `min_l (d(l,u) + d(l,v))` — через неравенство треугольника |
| Landmarks-LCA | `depth[u] + depth[v] − 2·depth[LCA]` — через SPT-дерево |

**Стратегии выбора ориентиров:** RANDOM, HIGH_DEGREE (хабы), COVERAGE (жадный max-min).

### Все флаги

| Флаг | По умолчанию | Назначение |
|------|:-----------:|------------|
| `--directed` | — | Загрузить граф как ориентированный (для undirected — без флага) |
| `--num-pairs=N` | 500 | Число случайных вершин для оценки диаметра и 90-го перцентиля |
| `--snowball-size=N` | 500 | Целевой размер snowball-подграфа |
| `--landmarks=N` | 10 | Число ориентиров для Part 2 |
| `--landmark-strategy=S` | RANDOM | Стратегия: `RANDOM`, `HIGH_DEGREE`, `COVERAGE` |
| `--part2` | — | Запустить Part 2 (Landmarks-Basic + Landmarks-LCA) |
| `--compare-landmarks` | — | Сравнить все 6 комбинаций (Basic/LCA × 3 стратегии) |
| `--algorithm=A` | both | Ограничить: `basic` или `lca` (для `--compare-landmarks`) |
| `--skip-part1` | — | Пропустить Part 1 (для повторного прогона с другим K) |
| `--skip-clustering` | — | Пропустить кластеризацию (экономит ~20 мин на больших графах) |

### Примеры флагов

```bash
# Полный анализ неориентированного графа
./gradlew run --args="graph.txt"

# Орграф с уменьшенной выборкой
./gradlew run --args="graph.txt --directed --num-pairs=200"

# Анализ + Landmarks, COVERAGE, K=20
./gradlew run --args="graph.txt --part2 --landmark-strategy=COVERAGE --landmarks=20"

# Только Landmarks (Part 1 уже сделан), оба алгоритма, все стратегии
./gradlew run --args="graph.txt --skip-part1 --part2 --compare-landmarks --landmarks=20"

# Орграф, Part 2 только Basic, без кластеризации
./gradlew run --args="graph.txt --directed --part2 --compare-landmarks --algorithm=basic --skip-clustering"
```

## Вывод

**stdout:** все метрики и таблица сравнения Landmarks.

**CSV-файлы** в `app/`:
- `degree_dist.csv` — распределение степеней
- `degree_loglog.csv` — log-log распределение
- `deletion_random.csv` — случайное удаление
- `deletion_high_degree.csv` — high-degree удаление

## Системные требования

- Java 21+
- Kotlin (через Gradle)
- `bash` (Linux/macOS/WSL)

## Авторы

Команда №7 — Никита, Гриша, Андрей
