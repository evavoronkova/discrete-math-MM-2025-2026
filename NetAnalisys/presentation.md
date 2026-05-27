---
marp: true
paginate: true
theme: custom
size: 16:9
---

<!--
 ═══════════════════════════════════════════
   NetAnalisys — Презентация командного проекта
 Авторы: Мороз Владислав, Овечкин Андрей
 Группа 25Б72-мм, Команда №3, 2026
 ═══════════════════════════════════════════
-->

<style>
/* ═══════════════════════════════════════════
 CORE THEME — NetAnalisys v2
 Corporate / Academic, Clean, Minimal
 ═══════════════════════════════════════════ */
:root {
 --blue: #1e22a9;
 --blue-dark: #16188a;
 --white: #FFFFFF;
 --text-dark: #1a1a2e;
 --text-body: #333340;
 --text-muted: #6b6b7a;
 --gray-light: #e8e8ed;
 --gray-border: #d4d4db;
 --gray-footer: #9A9A9A;
 --blue-accent: #3b44d4;
}

/* ── Base section ── */
section {
 font-family: 'Inter', 'Segoe UI', 'SF Pro Display', system-ui, sans-serif;
 color: var(--text-body);
 background-color: var(--white);
 padding: 0;
 margin: 0;
 position: relative;
 box-sizing: border-box;
}

/* ── Footer on ALL slides (white bg default) ── */
section::after {
 content: 'Мороз Владислав • Овечкин Андрей • СПбГУ • 2026 \00a0\00a0\00a0\00a0 ' attr(data-marpit-pagination);
 position: absolute;
 bottom: 7px;
 left: 0;
 width: 100%;
 padding: 10px 56px 0 56px;
 box-sizing: border-box;
 border-top: 1px solid var(--gray-border);
 color: var(--gray-footer);
 font-size: 10px;
 font-weight: 300;
 letter-spacing: 0.05em;
 z-index: 10;
}

/* ── Typography (white slides) ── */
h1 {
 font-size: 38px;
 font-weight: 700;
 color: var(--blue);
 letter-spacing: -0.01em;
 line-height: 1.2;
 margin: 0 0 12px 0;
}
h2 {
 font-size: 26px;
 font-weight: 600;
 color: var(--blue);
 margin: 0 0 8px 0;
}
h3 {
 font-size: 17px;
 font-weight: 600;
 color: var(--text-dark);
 margin: 0 0 4px 0;
}
p, li {
 font-size: 15px;
 line-height: 1.4;
 color: var(--text-body);
}
strong {
 color: var(--blue);
 font-weight: 600;
}
em {
 color: var(--text-muted);
 font-style: normal;
}

/* ── Lists ── */
ul, ol {
 padding-left: 26px;
 margin: 6px 0;
}
li {
 margin-bottom: 4px;
 font-size: 15px;
 line-height: 1.4;
}
li::marker {
 color: var(--blue);
}

/* ── Content area ── */
.content-box {
 padding: 56px 64px 90px 64px;
 min-height: 100%;
 box-sizing: border-box;
 overflow: visible;
}

/* ── Section label (small gray text above title) ── */
.section-label {
 font-size: 12px;
 font-weight: 500;
 color: var(--text-muted);
 text-transform: uppercase;
 letter-spacing: 0.1em;
 margin-bottom: 6px;
}

/* ═══════════════════════════════════════════
 BLUE SECTION DIVIDER SLIDES
 ═══════════════════════════════════════════ */
section.section {
 background-color: var(--blue);
 color: var(--white);
 display: flex;
 flex-direction: column;
 justify-content: center;
 padding-bottom: 44px;
 box-sizing: border-box;
}
section.section h1 {
 font-size: 44px;
 font-weight: 700;
 color: var(--white);
 margin: 0 0 20px 80px;
 letter-spacing: -0.01em;
}
section.section .white-line {
 width: 70px;
 height: 3px;
 background: var(--white);
 margin-left: 80px;
 margin-top: 0;
}
/* Footer on blue slides */
section.section::after {
 color: rgba(255,255,255,0.55);
 border-top: 1px solid rgba(255,255,255,0.18);
 bottom: 7px;
 font-size: 10px;
 padding: 10px 56px 0 56px;
content: 'Мороз Владислав • Овечкин Андрей • СПбГУ • 2026 \00a0\00a0\00a0\00a0 ' attr(data-marpit-pagination);
}

/* ═══════════════════════════════════════════
 TITLE SLIDE — enterprise / infrastructure
 ═══════════════════════════════════════════ */
section.title-slide {
 background-color: var(--blue);
 display: flex;
 flex-direction: column;
 justify-content: center;
 padding: 0 0 44px 21%;
 text-align: left;
 color: var(--white);
 box-sizing: border-box;
}
section.title-slide .title-label {
 font-size: 14px;
 font-weight: 300;
 color: rgba(255,255,255,0.72);
 letter-spacing: 0.10em;
 text-transform: uppercase;
 margin-bottom: 36px;
}
section.title-slide h1 {
 font-size: 72px;
 font-weight: 800;
 color: var(--white);
 letter-spacing: -0.03em;
 line-height: 1.0;
 margin: 0 0 0 0;
 max-width: 90%;
}
section.title-slide .thin-line {
 width: 64px;
 height: 2px;
 background: rgba(255,255,255,0.38);
 margin: 22px 0 28px 0;
}
section.title-slide .subtitle {
 font-size: 24px;
 color: rgba(255,255,255,0.72);
 font-weight: 400;
 line-height: 1.4;
 margin-bottom: 52px;
 max-width: 68%;
}
section.title-slide .meta {
 font-size: 13px;
 color: rgba(255,255,255,0.50);
 font-weight: 300;
 line-height: 2.0;
}
/* Footer on title slide */
section.title-slide::after {
 color: rgba(255,255,255,0.50);
 border-top: 1px solid rgba(255,255,255,0.18);
 bottom: 7px;
 font-size: 10px;
 padding: 10px 56px 0 56px;
 content: 'Мороз Владислав • Овечкин Андрей \00a0\00a0 2026 \00a0\00a0\00a0\00a0 ' attr(data-marpit-pagination);
}

/* Closing slide */
section.closing-slide {
 background-color: var(--blue);
 display: flex;
 flex-direction: column;
 justify-content: center;
 align-items: center;
 text-align: center;
 color: var(--white);
 padding: 0 0 44px 0;
 box-sizing: border-box;
}
section.closing-slide h1 {
 font-size: 56px;
 font-weight: 800;
 color: var(--white);
 letter-spacing: -0.02em;
 line-height: 1.1;
 margin: 0 0 28px 0;
}
section.closing-slide .closing-authors {
 font-size: 20px;
 font-weight: 300;
 color: rgba(255,255,255,0.88);
 margin-bottom: 40px;
 line-height: 1.6;
}
section.closing-slide .closing-license {
 font-size: 16px;
 font-weight: 300;
 color: rgba(255,255,255,0.78);
 line-height: 1.7;
}
section.closing-slide .closing-tagline {
 font-size: 13px;
 font-weight: 300;
 color: rgba(255,255,255,0.70);
 letter-spacing: 0.06em;
 margin-top: 36px;
}
section.closing-slide .closing-url {
 font-size: 11px;
 font-weight: 300;
 color: rgba(255,255,255,0.55);
 margin-top: 10px;
}
section.closing-slide::after {
 color: rgba(255,255,255,0.50);
 border-top: 1px solid rgba(255,255,255,0.18);
 bottom: 18px;
 font-size: 10px;
 padding: 10px 56px 0 56px;
 content: 'Мороз Владислав • Овечкин Андрей \00a0\00a0 2026 \00a0\00a0\00a0\00a0 ' attr(data-marpit-pagination);
}

/* ═══════════════════════════════════════════
 TABLES (white slides)
 ═══════════════════════════════════════════ */
table {
 width: fit-content;

 /*min-width: 700px;*/

 margin-left: auto;
 margin-right: auto;

 border-collapse: collapse;

 font-size: 12px;

 border: 1px solid var(--gray-border);

 border-radius: 4px;

 overflow: hidden;
}
th, td {
 padding: 12px 18px;
 vertical-align: middle;
 overflow-wrap: break-word;
 word-wrap: break-word;
}
th {
 background: var(--blue);
 color: var(--white);
 font-weight: 600;
 font-size: 11px;
 text-align: center;
}
td {
 padding: 6px 10px;
 border-bottom: 1px solid var(--gray-light);
 font-size: 11px;
 color: var(--text-body);
 text-align: center;
}
/* Column widths handled by content */
tr:last-child td {
 border-bottom: none;
}
tr:nth-child(even) td {
 background: #f8f8fb;
}

/* ═══════════════════════════════════════════
 CODE / PRE
 ═══════════════════════════════════════════ */
pre, code {
 font-family: 'JetBrains Mono', 'Fira Code', 'Cascadia Code', monospace;
 font-size: 13px;
}
pre {
 background: #f4f4f7;
 border: 1px solid var(--gray-light);
 border-radius: 6px;
 padding: 14px 18px;
 line-height: 1.6;
 margin: 10px 0;
 color: var(--text-dark);
}

/* ── Horizontal rule ── */
hr {
 border: none;
 height: 1px;
 background: var(--gray-light);
 margin: 14px 0;
}

/* ── Blockquote ── */
blockquote {
 border-left: 3px solid var(--blue);
 margin: 10px 0;
 padding: 8px 16px;
 background: #f8f8fb;
 border-radius: 0 4px 4px 0;
 font-size: 15px;
 color: var(--text-body);
}

/* ── Columns ── */
.columns {
 display: grid;
 grid-template-columns: 1fr 1fr;
 gap: 32px;
}
.columns-3 {
 display: grid;
 grid-template-columns: 1fr 1fr 1fr;
 gap: 20px;
}

/* ── Tags ── */
.tag {
 display: inline-block;
 background: #eef0fd;
 color: var(--blue);
 font-size: 12px;
 font-weight: 500;
 padding: 3px 10px;
 border-radius: 4px;
 margin: 2px 4px 2px 0;
 letter-spacing: 0.02em;
}

/* ── Image card (white bg, subtle border) ── */
.img-card {
 background: var(--white);
 border: 1px solid var(--gray-light);
 border-radius: 8px;
 padding: 14px;
 display: inline-block;
 margin: 8px 0;
}
.img-card img {
  display: block;
  max-width: 100%;
  max-height: 75vh;
  object-fit: contain;
  border-radius: 4px;
}
.img-dual {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 14px;
}
.img-dual .img-card img {
  max-height: 70vh;
}

/* ── Fit-image: large diagrams (80% of free space) ── */
.fit-image {
  display: block;
  margin: 0 auto;
  max-width: 100%;
  max-height: 70vh;
  object-fit: contain;
  border-radius: 6px;
}

/* ── Section callout box ── */
.callout {
 background: #f4f5fd;
 border-left: 4px solid var(--blue);
 border-radius: 0 6px 6px 0;
 padding: 14px 18px;
 margin: 12px 0;
 font-size: 15px;
 color: var(--text-body);
}

/* image row*/
.flex_container {
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: center;
  gap: 24px;
  margin-top: 14px;
}

</style>

<!--
 ═══════════════════════════════════════════
  SLIDE 1 — Титульный
 ═══════════════════════════════════════════
-->
<!-- _class: title-slide -->

<div class="title-label">Командный проект</div>

# NetAnalisys

<div class="thin-line"></div>

<div class="subtitle">Анализ структуры и приближённых расстояний<br>в больших графах</div>

<div class="meta">
Группа 25Б72-мм<br>
Команда №3<br>
СПбГУ<br>
2026
</div>

---

<!--
 ═══════════════════════════════════════════
  SLIDE 2 — Постановка задачи
 ═══════════════════════════════════════════
-->
<div class="content-box">

<div class="section-label">Введение</div>
<h1>Постановка задачи</h1>

**Проблема:** Графы социальных сетей содержат миллионы вершин и сотни миллионов рёбер. Точный расчёт кратчайших путей (APSP) непрактичен.

<br>

<h3>Три цели проекта</h3>

1. Разработать инструмент для **структурного анализа графов** — связность, кластеризация, устойчивость
2. Реализовать и сравнить **Landmark-алгоритмы оценки расстояний** (Basic LM, BFS LM) с 3 стратегиями выбора ориентиров
3. Провести эксперименты на **13 датасетах** — от 889 до ~3M вершин

<br>

<h3>Ключевые исследовательские вопросы</h3>

- Как устроена **топология реальных сетей**?
- Каков **trade-off между скоростью и точностью** landmark-алгоритмов?
- Какая **стратегия выбора ориентиров** оптимальна?

</div>

---

<!--
 ═══════════════════════════════════════════
  SLIDE 3 — Архитектура (разделитель)
 ═══════════════════════════════════════════
-->
<!-- _class: section -->

# Архитектура

<div class="white-line"></div>

---

<!--
 ═══════════════════════════════════════════
  SLIDE 4 — Архитектура NetAnalisys
 ═══════════════════════════════════════════
-->
<div class="content-box">

<div class="section-label">Архитектура</div>
<h1>Архитектура NetAnalisys</h1>

**Технологический стек:**
<span class="tag">Rust (stable 1.88-1.95)</span>
<span class="tag">crossterm </span>
<span class="tag">tokio</span>
<span class="tag">rayon</span>
<span class="tag">plotters</span>
<br>

<div class="columns">
<div>

<h3>Модули</h3>

- **<code>graph/</code>** — списки смежности, маппинг ID, степени
- **<code>parser/</code>** — .txt / .csv / .mtx, автоопределение directed/undirected
- **<code>analysis/</code>** — 6 подмодулей: связность → степени → диаметр → треугольники → кластеризация → устойчивость
- **<code>landmarks/</code>** — Basic LM + BFS LM, 3 стратегии выбора
</div>
<div>

<h3>Интерфейс</h3>


- **<code>ui/</code>** — TUI-меню, файловый браузер
- **<code>interactive_landmarks/</code>** — запрос расстояний, accuracy- и speed-бенчмарки

</div>
</div>

<br>

<h3>Pipeline обработки</h3>

<pre>выбор датасета → парсинг → параллельный подсчёт метрик → вывод + PNG → интерактивный режим</pre>

</div>


---

<!--
  ═══════════════════════════════════════════
  SLIDE 5 — Стек
  ═══════════════════════════════════════════
-->
<div class="content-box" style="text-align:center;">

<div class="section-label">Архитектура</div>
<h1 style="text-align:center;">Стек</h1>

<div class="flex_container">
<div>
<img class="fit-image" src="./for_presentaion/borrow_checker_ne_ponat.webp" alt="Borrow checker">
</div>
<div>
<img class="fit-image" src="./for_presentaion/феррис.png" alt="Ferris">
</div>
<div>
<img class="fit-image" src="./for_presentaion/ownership_eto_svoboda.webp" alt="Ownership">
</div>
</div>

<div style="text-align:center; margin-top:10px;">Выбор очевиден</div>

</div>

---

<!--
  ═══════════════════════════════════════════
  SLIDE 6 — Pipeline диаграмма
  ═══════════════════════════════════════════
-->
<div class="content-box" style="text-align:center;">

<div class="section-label">Архитектура</div>
<h1 style="text-align:center;">Pipeline обработки</h1>

<div style="margin-top:10px;">
<img class="fit-image" src="./for_presentaion/diagram.png" alt="Pipeline diagram" style="max-height:80vh; border-radius:6px;">
</div>

</div>

---

<!--
 ═══════════════════════════════════════════
  SLIDE 7 — Окружение (железо)
  ═══════════════════════════════════════════
-->
<div class="content-box">

<div class="section-label">Тестирование</div>
<h1>Окружение</h1>

<br>
<br>

<div style="display:flex; justify-content:center;">
<div style="width:70%;">
<table style="font-size:18px; width:100%;">
<thead>
<tr><th style="width:30%; font-size:17px;">Компонент</th><th style="width:70%; font-size:17px;">Характеристика</th></tr>
</thead>
<tbody>
<tr><td><strong>Ноутбук</strong></td><td>ASUS Vivobook 16X</td></tr>
<tr><td><strong>CPU</strong></td><td>AMD Ryzen 7 7730U (8 ядер / 16 потоков, 2.0–4.5 GHz)</td></tr>
<tr><td><strong>GPU</strong></td><td>Radeon Vega 8 (integrated)</td></tr>
<tr><td><strong>RAM</strong></td><td>16 GB DDR4</td></tr>
<tr><td><strong>ОС</strong></td><td>Linux</td></tr>
<tr><td><strong>Язык / Бэкенд</strong></td><td>Rust (stable), tokio + rayon</td></tr>
</tbody>
</table>
</div>
</div>



<div class="callout" style="text-align:center; font-size:16px;">
Все тесты проводились на одном устройстве при стандартных условиях. Длительность замеров фиксировалась через ручное логирование в отдельынй файл <code>performance.log</code>.
</div>
<div class="callout">
<strong>Ключевые выводы по производительности:</strong><br>
• Треугольники и Random 500 BFS — самые дорогие операции: до 120 с и <strong>20 мин</strong> на orkut<br>
• Double BFS на 3 порядка быстрее Random 500 BFS: 6 с vs 20 мин (<strong>×200</strong>)<br>
• На малых графах (&lt; 10K вершин) все pipeline-метрики считаются за доли секунды
</div>
</div>

---

<!--
  ═══════════════════════════════════════════
  SLIDE 8 — Время выполнения этапов
  ═══════════════════════════════════════════
-->
<div class="content-box">

<div class="section-label">Тестирование</div>
<h1>Производительность: время выполнения этапов</h1>

<br>

<table style="font-size:11px; width:100%;">
<thead>
<tr>
<th style="font-size:11px;">Этап</th>
<th style="font-size:11px;">Very Large (orkut)<br>3.07M V, 117M E</th>
<th style="font-size:11px;">Large<br>876K–3.2M V</th>
<th style="font-size:11px;">Medium<br>18K–326K V</th>
<th style="font-size:11px;">Small<br>889–7K V</th>
</tr>
</thead>
<tbody>
<tr><td><strong>Парсинг</strong> (parse_file)</td><td>81 s</td><td>4.8 s</td><td>70 ms</td><td>15 ms</td></tr>
<tr><td><strong>Слабая связность</strong> (WCC)</td><td>2.5 s</td><td>0.2 s</td><td>4 ms</td><td>0.3 ms</td></tr>
<tr><td><strong>Сильная связность</strong> (SCC)</td><td>—</td><td>0.7 s</td><td>1.6 ms</td><td>0.2 ms</td></tr>
<tr><td><strong>Степени</strong> (degree distribution)</td><td>14 ms</td><td>3 ms</td><td>0.1 ms</td><td>0.02 ms</td></tr>
<tr><td><strong>Треугольники</strong> (triangle count)</td><td>120 s</td><td>2.5 s</td><td>32 ms</td><td>6 ms</td></tr>
<tr><td><strong>Кластеризация</strong> (clustering coeff.)</td><td>9 ms</td><td>2 ms</td><td>0.2 ms</td><td>0.04 ms</td></tr>
<tr><td><strong>Double BFS</strong> (диаметр)</td><td>6.1 s</td><td>0.3 s</td><td>3 ms</td><td>80 µs</td></tr>
<tr><td><strong>P90 расстояний</strong> (percentile)</td><td>30 s</td><td>1 s</td><td>12 ms</td><td>0.3 ms</td></tr>
<tr><td><strong>Hub removal</strong> (robustness)</td><td>109 s</td><td>0.1 s</td><td>40 ms</td><td>80 ms</td></tr>
<tr><td><strong>Random removal</strong> (robustness)</td><td>140 s</td><td>0.15 s</td><td>60 ms</td><td>80 ms</td></tr>
<tr><td><strong>Random 500 BFS</strong> (отд. замер)</td><td><strong>~20 min</strong></td><td>5–10 s</td><td>0.5 s</td><td>35 ms</td></tr>
<tr style="background:#eef0fd;"><td><strong>Total runtime</strong></td><td><strong>~28 min</strong></td><td><strong>~5 s</strong></td><td><strong>~0.9 s</strong></td><td><strong>~0.4 s</strong></td></tr>
</tbody>
</table>




</div>

---

<!--
  ═══════════════════════════════════════════
  SLIDE 9 — Диаграмма: время обработки
  ═══════════════════════════════════════════
-->
<div class="content-box" style="display:flex; align-items:center; justify-content:center;">

<div style="width:100%; text-align:center;">
<div class="section-label">Диаграммы</div>
<h1 style="text-align:center;">Сводка времени обработки по категориям</h1>

<div style="margin-top:18px;">
<img class="fit-image" src="./for_presentaion/perf_timing.png" alt="Perf timing" style="max-height:50vh; border-radius:8px;">

---
<!--
 ═══════════════════════════════════════════
  SLIDE 10 — Orkut breakdown
 ═══════════════════════════════════════════
-->
<div class="content-box" style="display:flex; align-items:center; justify-content:center;">

<div style="width:100%; text-align:center;">
<div class="section-label">Диаграммы</div>
<h1 style="text-align:center;">Поэтапное время: Very Large граф (com-orkut)</h1>

<div style="margin-top:18px;">
<img class="fit-image" src="./for_presentaion/perf_orkut_breakdown.png" alt="Orkut breakdown" style="max-height:80vh; border-radius:8px;">
</div>
</div>

</div>

---

<!--
 ═══════════════════════════════════════════
  SLIDE 11 — Датасеты
 ═══════════════════════════════════════════
-->
<div class="content-box">

<div class="section-label">Данные</div>
<h1>Датасеты</h1>

**13 графов, 3 категории** — широкий охват предметных областей

<br>

| Категория | Кол-во | Диапазон вершин | Диапазон рёбер | Примеры |
|-----------|--------|------------------|-----------------|---------|
| **Directed** | 5 | 7K – 876K | 2.9K – 5.1M | web-Google, web-Stanford, wiki-Vote, soc-wiki-Vote, web-NotreDame |
| **Undirected** | 5 | 889 – 1.7M | 29K – 15.2M | ca-coauthors-dblp, CA-GrQc, CA-AstroPh, musae-git-edges, ca-HepTh |
| **Large** | 3 | 1.1M – 3.07M | 3M – **117M** | com-youtube, com-orkut, vk |

<br>

От коллабораций учёных → веб-графы → социальные сети → мессенджеры.

</div>

---

<!--
 ═══════════════════════════════════════════
  SLIDE 12 — Результаты (разделитель)
 ═══════════════════════════════════════════
-->
<!-- _class: section -->

# Результаты анализа

<div class="white-line"></div>

---

<!--
 ═══════════════════════════════════════════
  SLIDE 13 — Pipeline анализа и вычисляемые метрики
 ═══════════════════════════════════════════
-->
<div class="content-box">

<div class="section-label">Метрики</div>
<h1>Pipeline анализа и вычисляемые метрики</h1>

<div class="columns-3">

<div>

<h3>Блок 1. Базовая статистика</h3>

- \|V\|, \|E\|, плотность
- Weak / Strong компоненты
- LWCC / LSCC

</div>
<div>

<h3>Блок 2. Степени</h3>

- min / max / avg degree
- Распределение степеней
- Regular + log-log шкалы

</div>
<div>

<h3>Блок 3. Диаметр</h3>

- Double BFS
- Random 500 BFS
- Snowball sampling
- 90-й перцентиль

</div>

</div>

<div class="columns-3" style="margin-top: 18px;">

<div>

<h3>Блок 4. Треугольники<br>и кластеризация</h3>

- Полный подсчёт треугольников
- Средний коэффициент
- Глобальный коэффициент

</div>
<div>

<h3>Блок 5. Устойчивость</h3>

- Random removal (5%–100%)
- Degree-targeted removal
- Доля вершин в giant component

</div>
<div>

<h3>Визуализация</h3>

- PNG-графики распределений
- Терминальные графики
- Интерактивные запросы

</div>

</div>

---

<!--
 ═══════════════════════════════════════════
  SLIDE 14 — Распределение степеней
 ═══════════════════════════════════════════
-->
<div class="content-box" style="display:flex; align-items:center; justify-content:center;">

<div style="width:100%; text-align:center;">
<div class="section-label">Метрики</div>
<h1 style="text-align:center;">Распределение степеней</h1>

<div class="img-dual" style="margin-top: 24px;">
<div class="img-card">
<img src="for_presentaion/regular_the_bigest_graph.jpg" alt="Degree distribution">
</div>
<div class="img-card">
<img src="for_presentaion/log_log_thebigest_graph.jpg" alt="Log-log degree distribution">
</div>
</div>
</div>

</div>

---

<!--
 ═══════════════════════════════════════════
  SLIDE 15 — Результаты: базовая статистика
 ═══════════════════════════════════════════
-->
<div class="content-box">

<div class="section-label">Результаты</div>
<h1>Базовая статистика графов</h1>

Все графы крайне разрежены: density ~10⁻⁵…10⁻³

<br>

| Параметр | Undirected (mean ± σ) | Directed (mean ± σ) | Large/youtube | Large/orkut |
|----------|------------------------|----------------------|---------------|-------------|
| **Density** | 0.00067 ± 0.00050 | 0.00151 ± 0.00171 | 0.000005 | **0.000025** |
| **LWCC** | 0.937 ± 0.098 | 0.975 ± 0.046 | 1.000 | 1.000 |
| **LSCC** | — | 0.221 ± 0.224 | — | — |
| **Avg Degree** | 24.59 ± 22.17 | 15.28 ± 10.15 | 5.27 | **76.28** |
| **Diameter (DBFS)** | 16.25 ± 5.12 | 12.0 ± 21.4 | **24** | **10** |

<br>

> **Ключевой инсайт:** у directed-графов слабая связность высока (<strong>LWCC ~0.97</strong>), но сильная — крайне низкая (<strong>LSCC ~0.22</strong>). Разрыв LWCC − LSCC ≈ <strong>0.75</strong>.

**Outlier:** soc-wiki-Vote — LSCC ≈ **0.001** (сильная компонента практически отсутствует).

</div>

---

<!--
 ═══════════════════════════════════════════
  SLIDE 16 — Результаты: кластеризация и треугольники
 ═══════════════════════════════════════════
-->
<div class="content-box">

<div class="section-label">Результаты</div>
<h1>Кластеризация и треугольники</h1>

Диапазон clustering coefficient — от **0.08** до **0.80**.

<br>

| Граф | Треугольники | Avg CC | Global CC | Тип |
|------|-------------|--------|-----------|-----|
| **ca-coauthors-dblp** | Гигантское | 0.80 | 0.08 (при 1.7M вершин) | Плотная сообществная структура |
| **com-orkut** | **627M** | **0.1666** | 0.041 (при 3M вершин) | Плотный, хабово-кластеризованный |
| **Wiki-vote / soc-wiki-Vote** | Умеренное | 0.15–0.25 | Низкий | Локальные кластеры без глобального замыкания |
| **web-NotreDame / youtube** | Низкое | Низкий | Низкий | Разреженная backbone-структура |

<br>

<div class="callout">
<strong>Clustering — маркер типа сети:</strong> высокий → community-heavy, низкий → hub-dominated.
</div>

</div>

---

<!--
 ═══════════════════════════════════════════
  SLIDE 17 — Результаты: устойчивость к удалению вершин
 ═══════════════════════════════════════════
-->
<div class="content-box">

<div class="section-label">Результаты</div>
<h1>Устойчивость к удалению вершин</h1>

Два сценария: **random removal** vs **degree-targeted removal**

<br>

| Граф | Random (50% удалено) | Targeted (50% удалено) | Характер |
|------|---------------------|------------------------|----------|
| **ca-coauthors** | Giant comp. сохранена | Медленная деградация, затем резкий обвал | Умеренная hub-зависимость |
| **musae-git-edges** | Giant comp. сохранена | Быстрый распад | Выраженная hub-архитектура |
| **web-Stanford** | Giant comp. сохранена | Резкий пороговый распад | Сильная hub-архитектура |
| **web-NotreDame** | Giant comp. сохранена | Экстремально быстрый коллапс | Экстремальный hub-dominance |

<br>

<div class="callout">
<strong>Вывод:</strong> все графы — hub-driven / scale-free. Случайное удаление разрушает медленно, удаление хабов — резкий перколяционный порог.
</div>

</div>

---

<!--
 ═══════════════════════════════════════════
  SLIDE 18 — Large: контрастная пара
 ═══════════════════════════════════════════
-->
<div class="content-box">

<div class="section-label">Результаты</div>
<h1>Large-группа: неоднородность внутри категории</h1>

Large — не «один тип», а <strong>контрастная пара</strong>: два очень разных режима генерации сети

<br>

| Параметр | com-youtube | com-orkut | Δ |
|----------|-------------|-----------|----|
| **V** | 1.13M | 3.07M | ×2.7 |
| **E** | 2.99M | **117.19M** | ×39 |
| **Density** | 5×10⁻⁶ | 2.5×10⁻⁵ | ×5 |
| **Avg Degree** | 5.27 | **76.28** | ×14 |
| **Diameter (DBFS)** | 24 | **10** | **−58%** |
| **P90 distance** | 7 | **5** | −29% |
| **CC avg** | 0.0808 | **0.1666** | ×2 |
| **Треугольники** | ~0 | **627M** | Взрыв |
| **k_max** | — | **33 313** | Гигантский хаб |

<br>

<div class="callout">
<strong>Ключевой вывод:</strong> масштаб не ведёт к автоматическому разрежению. Orkut <em>больше</em> youtube, но <strong>плотнее в 5 раз</strong>, <strong>короче по путям на 58%</strong> и <strong>сильнее кластеризован в 2 раза</strong>. Large-категория смешивает принципиально разные структурные режимы.
</div>

---

<!--
 ═══════════════════════════════════════════
  SLIDE 19 — Диаграмма: Large контраст

  ═══════════════════════════════════════════
-->
<div class="content-box" style="display:flex; align-items:center; justify-content:center;">

<div style="width:100%; text-align:center;">
<div class="section-label">Диаграммы</div>
<h1 style="text-align:center;">Large-группа: контраст youtube vs orkut</h1>

<div style="margin-top:18px;">
<img class="fit-image" src="./for_presentaion/large_contrast.png" alt="Large contrast" style="max-height:80vh; border-radius:8px;">
</div>
</div>

</div>
---

<!--
 ═══════════════════════════════════════════
  SLIDE 20 — Типология исследованных графов
 ═══════════════════════════════════════════
-->
<div class="content-box">

<div class="section-label">Классификация</div>
<h1>Типология исследованных графов</h1>

4 типа на основе метрик:

<br>

| Тип | Представители | Признаки |
|-----|---------------|----------|
| **Community-heavy** | ca-coauthors-dblp, wiki-Vote, soc-wiki-Vote | Высокий clustering, много треугольников, небольшой диаметр, чувствительность к хабам |
| **Hub-dominated** | musae-git-edges, web-Stanford, web-NotreDame, youtube, **com-orkut** | Низкая плотность, большой k_max, быстрая потеря связности при targeted removal |
| **Фрагментированные** | CA-GrQc, CA-AstroPh | Много weak components, но giant WCC доминирует; неоднородная структура |
| **Асимметричные directed** | soc-wiki-Vote, wiki-Vote | Огромный разрыв weak ↔ strong connectivity; направление сильно ограничивает достижимость |

</div>

---

<!--
 ═══════════════════════════════════════════
  SLIDE 21 — Landmark (разделитель)
 ═══════════════════════════════════════════
-->
<!-- _class: section -->

# Landmark-алгоритмы

<div class="white-line"></div>

---

<!--
 ═══════════════════════════════════════════
  SLIDE 22 — Landmark-алгоритмы: постановка задачи
 ═══════════════════════════════════════════
-->
<div class="content-box">

<div class="section-label">Landmarks</div>
<h1>Landmark-алгоритмы: постановка задачи</h1>

<h3>Зачем нужны landmarks?</h3>

Точный BFS на графе с **3M вершин и 117M рёбер** — **1.26–1.57 с** на один запрос. Для тысячи запросов — часы.

**Идея:** выбрать **k ориентиров** (landmarks), предпосчитать расстояния от них, оценивать расстояние между любыми двумя вершинами через ориентиры за **O(k)**.

<br>

<div class="columns">
<div>

<h3>Два алгоритма</h3>

- **Basic LM:** d(s,t) ≈ minₗ [d(s,l) + d(l,t)]
- **BFS LM:** сохраняем BFS-деревья, на запросе строим подграф пересечения путей → BFS

</div>
<div>

<h3>Три стратегии выбора</h3>

1. **Random** — случайный выбор
2. **Highest-degree** — вершины с максимальной степенью
3. **Farthest-first coverage** — максимальное геодезическое покрытие

</div>
</div>

</div>

---

<!--
 ═══════════════════════════════════════════
  SLIDE 23 — Landmark-алгоритмы: архитектура реализации
 ═══════════════════════════════════════════
-->
<div class="content-box">

<div class="section-label">Landmarks</div>
<h1>Landmark: архитектура реализации</h1>

<pre>
<span style="color:#1e22a9;font-weight:600;">LandmarkSelection</span> (enum)
 ├── <span style="color:#3b44d4;">Random</span>
 ├── <span style="color:#3b44d4;">HighestDegree</span>
 └── <span style="color:#3b44d4;">Coverage</span> (farthest-first)

<span style="color:#1e22a9;font-weight:600;">LandmarkBasic</span> <span style="color:#1e22a9;font-weight:600;">LandmarkBFS</span>
 │ │
 ├── <span style="color:#6b6b7a;">precompute()</span> ├── <span style="color:#6b6b7a;">precompute()</span>
 │ └── BFS от каждого LM │ └── BFS + parent tree
 │ │
 └── <span style="color:#6b6b7a;">estimate(s, t)</span> └── <span style="color:#6b6b7a;">estimate(s, t)</span>
 └── min( d(s,LM) + └── collect_subgraph(s→LM + t→LM)
 d(LM, t) ) → <span style="color:#6b6b7a;">BFS на подграфе</span>
</pre>

<br>

| Характеристика | Basic LM | BFS LM |
|---------------|----------|--------|
| **Precomputation** | O(k · (V+E)) | O(k · (V+E)) |
| **Запрос** | O(k) | O(\|subgraph\|) |
| **Точность** | Ниже | Выше |
| **Память** | k · \|V\| расстояний | k · \|V\| расстояний + parent trees |

</div>

---

<!--
 ═══════════════════════════════════════════
  SLIDE 24 — Landmark: скорость на большом графе
 ═══════════════════════════════════════════
-->
<div class="content-box">

<div class="section-label">Результаты</div>
<h1>Landmark: скорость на большом графе (~1.5 GB)</h1>

Условия: **~10 и ~100 landmarks**, 50 случайных пар, 3 стратегии

<br>

| Метод | 10 LM | 100 LM | Ускорение vs Exact |
|-------|-------|--------|---------------------|
| **Exact BFS** | 1.26–1.43 s | 1.46–1.57 s | 1× |
| **Basic LM** | **3.1–3.3 µs** | **0.23–1.61 ms** | **×400 000 → ×1 000** |
| **BFS LM** | 11.6–11.9 ms | 15.0–18.0 ms | ×100 → ×80 |

<br>

- Basic LM ускоряет exact BFS в **~400 000 раз** (10 LM) и в **~1 000 раз** (100 LM)
- BFS LM — компромисс: **×80–100** быстрее exact, но точнее Basic
- Рост LM с 10 до 100 → Basic LM замедляется в **~500 раз** — trade-off speed vs quality

---
<!--
 ═══════════════════════════════════════════
  SLIDE 25 — Landmark: скорость на малом графе
 ═══════════════════════════════════════════
-->
<div class="content-box">

<div class="section-label">Результаты</div>
<h1>Landmark: скорость на малом графе (Wiki-Vote)</h1>

Условия: **Wiki-Vote (~7K вершин)**, 300 случайных пар

<br>

| Метод | Среднее время | vs Exact |
|-------|--------------|----------|
| **Exact BFS** | **107–165 µs** | 1× (эталон) |
| **Basic LM** | 0.3–134 µs | ×1.5–500 (быстрее) |
| **BFS LM** | 1.14–3.71 ms | **×10–25 медленнее exact** |

<br>

<div class="callout">
<strong>Критический вывод:</strong> на маленьких графах BFS LM теряет смысл как ускоритель — overhead precomputation + построение подграфа <strong>дороже</strong>, чем просто запустить exact BFS. Basic LM при этом всё ещё выигрывает.
</div>

</div>

---

<!--
 ═══════════════════════════════════════════
  SLIDE 26 — Диаграмма: скорость

  ═══════════════════════════════════════════
-->
<div class="content-box">
<div class="section-label">Результаты</div>
<h1>Диаграммы по скорости</h1>
<div style="text-align:center; margin-top:14px;">
<img class="fit-image" src="./for_presentaion/landmark_speed.png" alt="Landmark speed">
</div>
</div>


---

<!--
 ═══════════════════════════════════════════
  SLIDE 27 — Landmark: точность
 ═══════════════════════════════════════════
-->
<div class="content-box">

<div class="section-label">Результаты</div>
<h1>Landmark: точность</h1>

<div class="columns">
<div>

<h3>Большой граф (~1.5 GB, 50 пар)</h3>

| Параметр | Basic LM | BFS LM |
|----------|----------|--------|
| **Exact-match rate (10 LM)** | 0–26% | 28–48% |
| **Exact-match rate (100 LM)** | 0–58% | **74–88%** |
| **Средняя ошибка** | 0.46–4.14 | **0.00–0.26** |

</div>
<div>

<h3>Wiki-Vote (≈7K, 300 пар)</h3>

| Параметр | Basic LM | BFS LM |
|----------|----------|--------|
| **Exact-match rate** | 71–80% | **100%** |
| **Средняя ошибка** | Малая | **0 (идеально)** |

</div>
</div>

<br>

**Закономерности:**
- BFS LM стабильно точнее Basic LM на всех размерах
- Увеличение LM с 10 → 100 резко повышает точность (BFS LM: **28–48% → 74–88%**)
- На малом графе BFS LM достигает **100% совпадения** с exact — идеальная точность
- Средняя ошибка Basic LM (0.46–4.14) на порядок выше, чем BFS LM (0.00–0.26)

</div>

---

<!--
 ═══════════════════════════════════════════
  SLIDE 28 — Диаграмма: точность
 ═══════════════════════════════════════════
-->
<div class="content-box" style="display:flex; align-items:center; justify-content:center;">

<div style="width:100%; text-align:center;">
<div class="section-label">Диаграммы</div>
<h1 style="text-align:center;">Точность Landmark-алгоритмов</h1>

<div style="margin-top:18px;">
<img class="fit-image" src="./for_presentaion/landmark_accuracy.png" alt="Landmark accuracy">
</div>
</div>

</div>

---

<!--
 ═══════════════════════════════════════════
  SLIDE 29 — Landmark: влияние стратегий выбора
 ═══════════════════════════════════════════
-->
<div class="content-box">

<div class="section-label">Результаты</div>
<h1>Landmark: влияние стратегий выбора</h1>

<div class="columns">
<div>

<h3>На большом графе (~1.5 GB)</h3>

- **Random:** хороший baseline, конкурентен для BFS LM
- **Highest-degree:** самая **сбалансированная** для Basic LM — лучший exact-match rate, ниже ошибка
- **Farthest-first:** не универсальный лидер — иногда уступает degree по accuracy

</div>
<div>

<h3>На малом графе (Wiki-Vote)</h3>

- **Highest-degree:** стабильна, хороша по accuracy
- **Farthest-first:** лучшая **latency** для Basic LM, но по точности не всегда выигрывает
- **Random:** приемлема, но не оптимальна

</div>
</div>

<br>

<div class="callout">
<strong>Статистическая оговорка:</strong> на 50 парах разница 44% vs 48% — шум. Разница 0% vs 58% или 28% vs 85% — реальный эффект. На 300 парах выводы надёжнее.
</div>

</div>

---

<!--
 ═══════════════════════════════════════════
  SLIDE 30 — Диаграмма: стратегии
 ═══════════════════════════════════════════
-->
<div class="content-box" style="display:flex; align-items:center; justify-content:center;">

<div style="width:100%; text-align:center;">
<div class="section-label">Диаграммы</div>
<h1 style="text-align:center;">Влияние стратегий выбора ориентиров</h1>

<div style="margin-top:18px;">
<img class="fit-image" src="./for_presentaion/landmark_strategies.png" alt="Landmark strategies">
</div>
</div>

</div>

---

<!--
 ═══════════════════════════════════════════
  SLIDE 31 — Практические рекомендации
 ═══════════════════════════════════════════
-->
<div class="content-box">

<div class="section-label">Рекомендации</div>
<h1>Практические рекомендации</h1>

<br>

| Если ваша цель | Алгоритм | Стратегия | k | Ожидаемый результат |
|---------------|----------|-----------|----|---------------------|
| **Максимальная скорость** | Basic LM | Random / Farthest-first | 10 | ×400 000 ускорение, точность 0–26% |
| **Максимальная точность** | BFS LM | Highest-degree | 100 | до 88% exact-match, ×80 ускорение |
| **Баланс speed/quality** | BFS LM | Highest-degree | 30–50 | ×200 ускорение, 60–70% точности |
| **Маленький граф (< 50K)** | **Exact BFS** | — | — | дешевле, чем BFS LM |
| **Low-latency на малом графе** | Basic LM | Farthest-first | 10 | микросекунды, accuracy ~71–80% |

<br>

<div class="callout">
<strong>Правило большого пальца:</strong> Basic LM → взрывная скорость, умеренная точность. BFS LM → высокая точность, хорошее ускорение (но не на малых графах). Degree-based — самая устойчивая стратегия в большинстве сценариев.
</div>

</div>

---

<!--
 ═══════════════════════════════════════════
  SLIDE 32 — Заключение (разделитель)
 ═══════════════════════════════════════════
-->
<!-- _class: section -->

# Заключение

<div class="white-line"></div>

---

<!--
 ═══════════════════════════════════════════
  SLIDE 33 — Заключение
 ═══════════════════════════════════════════
-->
<div class="content-box">

<div class="section-label">Итоги</div>
<h1>Заключение</h1>

<div class="columns">
<div>

<h3>Что сделано</h3>

- Разработано **Rust TUI-приложение** для полного цикла анализа графов
- Реализованы **3 метода оценки диаметра** + **2 landmark-алгоритма** + **3 стратегии**
- Проведены эксперименты на **13 датасетах**: от 889 до **3.07M вершин**

</div>
<div>

<h3>Ключевые выводы</h3>

1. Реальные графы — **scale-free** с hub-архитектурой; directed — разрыв LWCC/LSCC ~0.75
2. Large-группа **неоднородна**: youtube (diam=24) vs orkut (diam=10, CC×2)
3. **Basic LM:** до **4×10⁵×** ускорение, точность 0–58%
4. **BFS LM:** до **88%** точности при 100 LM, ×80–100
5. **Degree-based** — самая устойчивая стратегия; на малых графах — exact BFS или Basic LM

</div>
</div>

<br>

<h3>Перспективы</h3>

<span class="tag">CI/CD</span>
<span class="tag">Тестовое покрытие</span>
<span class="tag">Библиотечный crate</span>
<span class="tag">WebAssembly</span>

</div>

---

<!--
 ═══════════════════════════════════════════
  SLIDE 34 — Закрытие
 ═══════════════════════════════════════════
-->
<!-- _class: closing-slide -->

<h1>NetAnalisys</h1>

<div class="closing-authors">
Мороз Владислав &nbsp;•&nbsp; Овечкин Андрей<br>
Санкт-Петербургский государственный университет &nbsp;•&nbsp; 2026
</div>

<div class="closing-license">
&copy; 2026<br>
Creative Commons Attribution 4.0 International<br>
(CC BY 4.0)
</div>

<div class="closing-tagline">
Research &nbsp;•&nbsp; Engineering &nbsp;•&nbsp; Open Knowledge
</div>

<div class="closing-url">
creativecommons.org/licenses/by/4.0
</div>
