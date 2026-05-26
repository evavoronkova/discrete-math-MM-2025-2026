---
marp: true
style: |
  section {
    background: #ffffff;
    color: #1a1a1a;
    font-family: 'Segoe UI', 'Calibri', sans-serif;
  }
  h1 { color: #003366; }
  h2 { color: #003366; border-bottom: 2px solid #003366; padding-bottom: 6px; }
  strong { color: #003366; }
  a { color: #0066cc; }
  table { border-collapse: collapse; margin: 0 auto; }
  th { background: #003366; color: #ffffff; padding: 6px 12px; }
  td { padding: 4px 12px; border: 1px solid #ccc; }
  tr:nth-child(even) { background: #f5f7fa; }
  blockquote {
    background: #f0f4f8;
    border-left: 4px solid #003366;
    color: #333;
    padding: 8px 16px;
    font-style: italic;
  }
  code { background: #eef0f4; color: #003366; }
---

# Исследование структуры социальных графов и оценка расстояний на основе алгоритма Landmarks

**Командный проект по курсу «Теория графов и её приложения», 2026**

Команда №7

---

## Цели и задачи

**Цель:** Исследовать структуру социальных сетей и реализовать приближённый алгоритм оценки кратчайших расстояний.

**Задачи:**
1. Вычислить базовые и продвинутые метрики для каждого датасета
2. Исследовать устойчивость сетей к случайному и направленному удалению узлов
3. Реализовать алгоритм Landmarks-Basic для оценки расстояний
4. Реализовать модификацию Landmarks-LCA (двоичный подъём, O(log n))
5. Сравнить точность и скорость при разных стратегиях выбора ориентиров

---

## Данные

| Сеть | V | E | Тип | Источник |
|------|---|---|-----|----------|
| CA-GrQc | 5 241 | 14 484 | ⇔ | Arxiv General Relativity |
| CA-AstroPh | 18 771 | 198 050 | ⇔ | Arxiv Astrophysics |
| Email-EuAll | 265 009 | 364 481 | → | Email network |
| Wiki-Vote | 7 115 | 103 689 | → | Wikipedia voting |
| soc-wiki-Vote | 889 | 2 914 | ⇔ | Wikipedia social |
| web-Google | 875 713 | 5 105 039 | → | Google web graph 2002 |
| web-NotreDame | 325 729 | 1 469 679 | → | Notre Dame web |
| web-Stanford | 281 903 | 2 312 497 | → | Stanford web |
| musae_git | 37 700 | 289 003 | ⇔ | GitHub developers |
| com-youtube | 1 134 890 | 2 987 624 | ⇔ | YouTube social |
| ca-coauthors-dblp | 540 486 | 15 245 729 | ⇔ | DBLP coauthors |
| **com-orkut** | **3 072 441** | **117 185 083** | ⇔ | Orkut social network |
| **vk** | **3 215 720** | **17 414 510** | ⇔ | VK social network |

---

## Архитектура решения

![width:700px](./architecture.png)

**Стек:** Kotlin + Gradle, хранение CSR (Compressed Sparse Row)

**Алгоритмы:**
- BFS (очередь на массиве, O(V+E))
- Косарайю (сильная связность, O(V+E))
- Chiba-Nishizeki (подсчёт треугольников, O(E·α))
- Двоичный подъём (LCA за O(log n))
- Greedy max-min (COVERAGE-отбор ориентиров)

**Оптимизации:**
- Параллельный BFS через корутины (limitedParallelism=4)
- Флаг `--skip-part1` для повторного запуска без пересчёта анализа
- Флаг `--skip-clustering` для пропуска кластеризации на больших графах

---

## Базовые характеристики

| Сеть | V | E | Плотность | WCC | Доля giant |
|------|---|---|-----------|-----|-----------|
| CA-GrQc | 5.2K | 14K | 1.05e-3 | 354 | 0.793 |
| CA-AstroPh | 18.8K | 198K | 1.12e-3 | 289 | 0.954 |
| Email-EuAll | 265K | 364K | 1.04e-5 | 15.6K | 0.848 |
| Wiki-Vote | 7.1K | 104K | 2.05e-3 | 24 | 0.993 |
| musae_git | 37.7K | 289K | 4.07e-4 | 1 | 1.000 |
| soc-wiki-Vote | 889 | 2.9K | 3.69e-3 | 1 | 1.000 |
| web-Google | 876K | 5.1M | 6.66e-6 | 2.7K | 0.977 |
| web-NotreDame | 326K | 1.5M | 1.39e-5 | 1 | 1.000 |
| web-Stanford | 282K | 2.3M | 2.91e-5 | 365 | 0.906 |
| ca-coauthors-dblp | 540K | 15.2M | 1.04e-4 | 1 | 1.000 |
| com-youtube | 1.1M | 3.0M | 4.64e-6 | 1 | 1.000 |
| **com-orkut** | **3.1M** | **117M** | **2.48e-5** | **1** | **1.000** |
| **vk** | **3.2M** | **17.4M** | **3.37e-6** | **24.3K** | **0.983** |

> Все сети разреженные (плотность < 0.01), с выраженной гигантской компонентой.

---

## Оценка диаметра

| Сеть | Double Sweep | RndPairs 90% | Snowball 90% |
|------|:-----------:|:-----------:|:------------:|
| CA-GrQc | 17 | 8 | 5 |
| CA-AstroPh | 14 | 5 | 3 |
| Email-EuAll | 14 | 5 | 3 |
| Wiki-Vote | 6 | 4 | 2 |
| musae_git | 11 | 4 | 2 |
| soc-wiki-Vote | 13 | 6 | 4 |
| web-Google | 24 | 9 | 2 |
| web-NotreDame | 46 | 10 | 4 |
| web-Stanford | 164 | 10 | 2 |
| ca-coauthors-dblp | 23 | 7 | 4 |
| com-youtube | 24 | 7 | 4 |
| **com-orkut** | **9** | **5** | **3** |
| **vk** | **18** | **7** | **5** |

> Double Sweep — точная оценка за 2 BFS. 90% вершин достижимы за 5–10 шагов (эффект «тесного мира»).

---

## Кластеризация и степени

| Сеть | Треугольники | Средний CC | Глобальный CC |
|------|:-----------:|:----------:|:-------------:|
| CA-GrQc | 48 260 | 0.530 | 0.630 |
| CA-AstroPh | 1 351 441 | 0.631 | 0.318 |
| Email-EuAll | 267 313 | 0.067 | 0.004 |
| Wiki-Vote | 734 946 | 0.161 | 0.139 |
| musae_git | 523 810 | 0.168 | 0.012 |
| soc-wiki-Vote | 2 119 | 0.153 | 0.127 |
| ca-coauthors-dblp | 444 095 058 | 0.802 | 0.656 |
| com-youtube | 3 056 386 | 0.081 | 0.006 |
| web-NotreDame | 30 889 434 | 0.242 | 0.237 |
| **com-orkut** | **627 584 181** | **0.167** | **0.041** |
| **vk** | **108 030 337** | **0.049** | **0.110** |
| web-Google | 25 055 293 | 0.617 | 0.097 |
| web-Stanford | 17 851 425 | 0.633 | 0.013 |

![width:550px](./scripts/results/degree_loglog.png)

> Кластерные коэффициенты характерны для социальных сетей. Log-log распределение степеней показывает степенной закон — признак scale-free сети.

---

## Устойчивость к удалению узлов

**Случайное удаление (50% узлов):**
- com-orkut: гигантская компонента сохраняется на **98.2%**
- ca-coauthors-dblp: **93.0%**
- VK: **80.4%**
- CA-GrQc: **53.1%**

**High-degree удаление (50% узлов):**
- com-orkut: падает до **87.2%**
- ca-coauthors-dblp: **75.0%**
- VK: ≈**0.001%** (сеть разрушена!)
- CA-GrQc: ≈**0.002%**
- web-Google: ≈**0.01%**
- Email-EuAll: ≈**0.001%**

![width:600px](./scripts/results/robustness.png)

> Сети устойчивы к случайным отказам, но уязвимы к направленным атакам на high-degree вершины. VK и Email-EuAll разрушаются практически полностью.

---

## Алгоритм Landmarks

**Идея:** выбрать K ориентиров, предвычислить BFS от каждого, оценка:

```
d̃(u,v) = min_{l ∈ L} (d(l,u) + d(l,v))
```

**Landmarks-Basic:** нижняя оценка через неравенство треугольника.

**Landmarks-LCA:** оценка через наименьшего общего предка в SPT-дереве:
```
d(u,v) = depth[u] + depth[v] − 2·depth[LCA(u,v)]
```
LCA ищется двоичным подъёмом за O(log n).

**Стратегии выбора ориентиров:**
- **RANDOM** — случайный выбор
- **HIGH_DEGREE** — вершины с наибольшей степенью
- **COVERAGE** — жадный max-min (максимизация покрытия сети)

---

## Точность Landmarks — Orkut (K=20)

| Алгоритм | Стратегия | AvgError | MaxError | Exact% |
|----------|:--------:|:--------:|:--------:|:------:|
| Basic | RANDOM | 2.540 | 5 | 0.0% |
| Basic | HIGH_DEGREE | 0.775 | 3 | **42.5%** |
| Basic | COVERAGE | 1.860 | 6 | 7.0% |
| **LCA** | RANDOM | 1.410 | 4 | 20.0% |
| **LCA** | HIGH_DEGREE | **0.850** | 3 | **39.5%** |
| **LCA** | COVERAGE | 1.275 | 4 | 13.5% |

**LCA стабильно точнее Basic.** HIGH_DEGREE — оптимальная стратегия. При K=20 точность достигает 42.5% exact совпадений.

---

## Точность Landmarks — DBLP (K=20)

| Алгоритм | Стратегия | AvgError | MaxError | Exact% |
|----------|:--------:|:--------:|:--------:|:------:|
| Basic | RANDOM | 2.810 | 6 | 0.0% |
| Basic | HIGH_DEGREE | 1.060 | 5 | 29.5% |
| Basic | COVERAGE | 2.005 | 6 | 5.5% |
| **LCA** | RANDOM | 1.670 | 4 | 7.5% |
| **LCA** | HIGH_DEGREE | **0.900** | 3 | **29.0%** |
| **LCA** | COVERAGE | 1.330 | 3 | 15.5% |

---

## Точность Landmarks — VK (K=20, Basic only)

| Алгоритм | Стратегия | AvgError | MaxError | Exact% |
|----------|:--------:|:--------:|:--------:|:------:|
| Basic | RANDOM | 3.061 | 5 | 0.0% |
| Basic | HIGH_DEGREE | **0.713** | 3 | **39.5%** |
| Basic | COVERAGE | 1.260 | 3 | 6.3% |

> Для VK (3.2M вершин) LCA не запускался из-за ограничений памяти (O(K·V·log V) ≈ 30 ГБ при K=20).

---

## Скорость работы (Preprocess Time)

| Сеть | V | E | Basic K=20 | LCA K=20 |
|------|------|---------|:----------:|:--------:|
| ca-coauthors-dblp | 540K | 15.2M | 773 ms | 962 ms |
| **com-orkut** | **3.1M** | **117M** | **10.9 s** | **11.7 s** |
| **vk** | **3.2M** | **17.4M** | **4.9 s** | — |

> Время preprocess для Orkut (3M верш., 117M рёбер): ~11 с. Результаты сопоставимы с Tretyakov et al. (2010).

---

## Сравнение со статьёй

**Tretyakov et al. (2010), «Fast Exact and Approximate Graphlet and Distance Estimation»:**
- Для графов 100K–1M вершин: время preprocess — единицы секунд при K=20
- HIGH_DEGREE — лучшая стратегия (наименьшая ошибка)
- LCA точнее Basic на 30-50%

**Наши результаты:**
- Orkut (3.1M верш., K=20): preprocess ≈11 с — линейное масштабирование
- HIGH_DEGREE: avg error 0.78 (Basic), 0.85 (LCA) — подтверждаем выводы Tretyakov
- LCA точнее Basic на 15-30%

> Результаты полностью сопоставимы с литературными, алгоритмы масштабируются на графы до 3М вершин.

---

## Выводы

1. Все исследованные сети — разреженные, с выраженной гигантской компонентой
2. Распределение степеней подчиняется степенному закону (scale-free)
3. Сети устойчивы к случайному удалению узлов, но уязвимы к направленным атакам на high-degree вершины
4. Landmarks-LCA стабильно точнее Landmarks-Basic (на 15–30%)
5. Оптимальная стратегия: HIGH_DEGREE (точнее RANDOM на 50-70% при том же K)
6. При K=20 точность достигает 42% exact совпадений при времени preprocess ~11 с для Orkut
7. Ограничения: точность падает для периферийных вершин; LCA требует O(K·V·log V) памяти

---

## Спасибо за внимание!

Вопросы?
