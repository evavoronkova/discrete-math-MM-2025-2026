# discrete-math-MM-2025-2026

Проект по курсу «Теория графов», команда №7. Анализ структуры социальных графов и оценка расстояний через алгоритм Landmarks.

## Запуск

```bash
./gradlew run --args="путь/к/графу.txt [флаги]"
```

Формат графа — edge list: два целых числа через пробел на строке. При загрузке автоматически удаляются `\r`, петли, дубликаты. Строки с `#` игнорируются. Нужен `bash`.

## Флаги

| Флаг | По умолчанию | Что делает |
|------|:-----------:|------------|
| `--directed` | — | Орграф (по умолчанию — неориентированный) |
| `--num-pairs=N` | 500 | Размер выборки для оценки диаметра |
| `--snowball-size=N` | 500 | Размер snowball-подграфа |
| `--part2` | — | Запустить Landmarks |
| `--compare-landmarks` | — | Сравнить все комбинации (Basic/LCA × 3 стратегии) |
| `--landmarks=N` | 10 | Число ориентиров |
| `--landmark-strategy=S` | RANDOM | RANDOM / HIGH_DEGREE / COVERAGE |
| `--algorithm=A` | both | basic / lca (для `--compare-landmarks`) |
| `--skip-part1` | — | Пропустить Part 1 (повторный прогон с другим K) |
| `--skip-clustering` | — | Пропустить кластеризацию (экономия на больших графах) |

## Примеры

```bash
# Базовый анализ
./gradlew run --args="graph.txt"

# Анализ + Landmarks, K=10, сравнение стратегий
./gradlew run --args="graph.txt --part2 --compare-landmarks --landmarks=10"

# Только Landmarks K=20 (Part 1 уже сделан)
./gradlew run --args="graph.txt --skip-part1 --part2 --compare-landmarks --landmarks=20"

# Орграф, 200 пар, без кластеризации
./gradlew run --args="graph.txt --directed --num-pairs=200 --skip-clustering"
```

## Как работает

**Часть 1** — анализ сети: плотность, диаметр (Double Sweep + Random Pairs + Snowball), кластеризация, степени, устойчивость к удалению узлов.

**Часть 2** — Landmarks: K ориентиров, BFS от каждого. Оценка расстояния через `min_l (d(l,u) + d(l,v))` (Basic) или через LCA в SPT-дереве (LCA).

## Авторы

Никита, Гриша, Андрей
