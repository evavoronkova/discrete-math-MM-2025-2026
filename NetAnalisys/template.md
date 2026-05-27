---
marp: true
paginate: true
theme: custom
size: 16:9
---

<!--
╔══════════════════════════════════════════════════════════════════╗
║           M A R P   T E M P L A T E   —   NetAnalisys          ║
║                                                                ║
║  Используйте этот файл как основу для новых презентаций.       ║
║  Инструкции — ниже.                                            ║
╚══════════════════════════════════════════════════════════════════╝

  ███████████████████████████████████████████████████████████████
       КАК ПОЛЬЗОВАТЬСЯ ТЕМПЛЕЙТОМ
  ███████████████████████████████████████████████████████████████

  1. Скопируйте этот файл в новое место
  2. Замените заглушки (TODO) своим содержимым
  3. Настройте footer в CSS-блоке (строки с "content:")
  4. Добавляйте/удаляйте слайды по образцам ниже

  ███████████████████████████████████████████████████████████████
       ТИПЫ СЛАЙДОВ
  ███████████████████████████████████████████████████████████████

  ┌──────────────────────────────────────────────────────────┐
  │  class="title-slide"    → синий фон, крупный заголовок   │
  │  class="section"        → синий разделитель, 1 строка    │
  │  class="closing-slide"  → синий финальный слайд          │
  │  class="content-box"    → белый слайд с контентом        │
  └──────────────────────────────────────────────────────────┘

  ███████████████████████████████████████████████████████████████
       CSS-КОМПОНЕНТЫ (см. в блоке <style>)
  ███████████████████████████████████████████████████████████████

  .content-box  → контейнер для белых слайдов
  .section-label → серая метка над заголовком
  .columns       → 2 колонки (grid, gap 32px)
  .columns-3     → 3 колонки (grid, gap 20px)
  .callout       → блок с синей левой рамкой (инсайт/вывод)
  .tag           → цветной чип/тег
  .img-card      → рамка вокруг изображения
  .img-dual      → два изображения рядом
  .fit-image     → изображение на весь слайд (для диаграмм)
-->

<style>
/* ═══════════════════════════════════════════
   CORE THEME
   Настройте цвета и footer под свой проект
   ═══════════════════════════════════════════ */
:root {
  --blue:         #1e22a9;
  --blue-dark:    #16188a;
  --white:        #FFFFFF;
  --text-dark:    #1a1a2e;
  --text-body:    #333340;
  --text-muted:   #6b6b7a;
  --gray-light:   #e8e8ed;
  --gray-border:  #d4d4db;
  --gray-footer:  #9A9A9A;
  --blue-accent:  #3b44d4;
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

/* ── Footer — ЗАМЕНИТЕ подпись на свою ── */
section::after {
  content: 'Ваше Имя  •  Ваша Организация  •  2026 \00a0\00a0\00a0\00a0 ' attr(data-marpit-pagination);
  position: absolute;
  bottom: 18px;
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

/* ── Typography ── */
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

/* ── Content area (для белых слайдов) ── */
.content-box {
  padding: 56px 64px 90px 64px;
  min-height: 100%;
  box-sizing: border-box;
  overflow: visible;
}

/* ── Section label (серая метка над заголовком) ── */
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
section.section::after {
  color: rgba(255,255,255,0.55);
  border-top: 1px solid rgba(255,255,255,0.18);
  bottom: 18px;
  font-size: 10px;
  padding: 10px 56px 0 56px;
  /* ЗАМЕНИТЕ подпись */
  content: 'Ваше Имя  •  Ваша Организация  •  2026 \00a0\00a0\00a0\00a0 ' attr(data-marpit-pagination);
}

/* ═══════════════════════════════════════════
   TITLE SLIDE — вступительный слайд
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
  margin: 0;
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
section.title-slide::after {
  color: rgba(255,255,255,0.50);
  border-top: 1px solid rgba(255,255,255,0.18);
  bottom: 18px;
  font-size: 10px;
  padding: 10px 56px 0 56px;
  content: 'Ваше Имя  •  Ваша Организация \00a0\00a0 2026 \00a0\00a0\00a0\00a0 ' attr(data-marpit-pagination);
}

/* ═══════════════════════════════════════════
   CLOSING SLIDE — финальный слайд
   ═══════════════════════════════════════════ */
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
  content: 'Ваше Имя  •  Ваша Организация \00a0\00a0 2026 \00a0\00a0\00a0\00a0 ' attr(data-marpit-pagination);
}

/* ═══════════════════════════════════════════
   TABLES
   ═══════════════════════════════════════════ */
table {
  width: fit-content;
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

/* ── Blockquote (цитата) ── */
blockquote {
  border-left: 3px solid var(--blue);
  margin: 10px 0;
  padding: 8px 16px;
  background: #f8f8fb;
  border-radius: 0 4px 4px 0;
  font-size: 15px;
  color: var(--text-body);
}

/* ═══════════════════════════════════════════
   LAYOUT COMPONENTS
   ═══════════════════════════════════════════ */

/* Две колонки */
.columns {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 32px;
}

/* Три колонки */
.columns-3 {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  gap: 20px;
}

/* Тег/чип */
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

/* Карточка изображения с рамкой */
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
  max-height: 52vh;
  object-fit: contain;
  border-radius: 4px;
}

/* Два изображения рядом */
.img-dual {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 14px;
}
.img-dual .img-card img {
  max-height: 44vh;
}

/* Изображение на весь слайд (для диаграмм) */
.fit-image {
  display: block;
  margin: 0 auto;
  max-width: 100%;
  max-height: 48vh;
  object-fit: contain;
}

/* Callout-блок (вывод/инсайт) */
.callout {
  background: #f4f5fd;
  border-left: 4px solid var(--blue);
  border-radius: 0 6px 6px 0;
  padding: 14px 18px;
  margin: 12px 0;
  font-size: 15px;
  color: var(--text-body);
}
</style>


<!--
╔══════════════════════════════════════════════════════════════════╗
║          П Р И М Е Р Ы   С Л А Й Д О В                         ║
║  Ниже — готовые шаблоны. Замените текст на свой.               ║
╚══════════════════════════════════════════════════════════════════╝
-->


<!-- ============================================================
     СЛАЙД 1 — Титульный
     Используйте class="title-slide"
     ============================================================ -->
<!-- _class: title-slide -->

<div class="title-label">Название проекта / курса</div>

<h1>Название<br>вашего продукта</h1>

<div class="thin-line"></div>

<div class="subtitle">Краткое описание — одна-две строки<br>о чём эта презентация</div>

<div class="meta">
Ваше Имя<br>
Группа / Должность<br>
2026
</div>

---

<!-- ============================================================
     СЛАЙД 2 — Контентный (белый)
     Используйте class="content-box"
     ============================================================ -->
<div class="content-box">

<div class="section-label">Название раздела</div>
<h1>Заголовок слайда</h1>

<p>Обычный текст. Используйте <strong>strong</strong> для выделения важного и <em>em</em> для второстепенного.</p>

<ul>
  <li>Первый пункт списка</li>
  <li>Второй пункт списка</li>
  <li>Третий пункт</li>
</ul>

</div>

---

<!-- ============================================================
     СЛАЙД 3 — Разделитель (синий)
     Используйте class="section"
     ============================================================ -->
<!-- _class: section -->

<h1>Название раздела</h1>

<div class="white-line"></div>

---

<!-- ============================================================
     СЛАЙД 4 — Таблица
     ============================================================ -->
<div class="content-box">

<div class="section-label">Данные</div>
<h1>Заголовок с таблицей</h1>

<br>

| Параметр | Колонка A | Колонка B | Δ |
|----------|-----------|-----------|----|
| **Показатель 1** | 123 | 456 | +270% |
| **Показатель 2** | 789 | 12 | −98% |
| **Показатель 3** | 0.05 | 0.80 | ×16 |

<div class="callout">
<strong>Вывод:</strong> здесь можно написать ключевой инсайт из таблицы выше.
</div>

</div>

---

<!-- ============================================================
     СЛАЙД 5 — Две колонки
     Используйте class="columns"
     ============================================================ -->
<div class="content-box">

<div class="section-label">Сравнение</div>
<h1>Две колонки текста</h1>

<div class="columns">
<div>

<h3>Левая колонка</h3>

- Пункт A
- Пункт B
- Пункт C

</div>
<div>

<h3>Правая колонка</h3>

- Пункт X
- Пункт Y
- Пункт Z

</div>
</div>

</div>

---

<!-- ============================================================
     СЛАЙД 6 — Три колонки
     Используйте class="columns-3"
     ============================================================ -->
<div class="content-box">

<div class="section-label">Метрики</div>
<h1>Три колонки</h1>

<div class="columns-3">
<div>

<h3>Блок 1</h3>

- Элемент
- Элемент
- Элемент

</div>
<div>

<h3>Блок 2</h3>

- Элемент
- Элемент
- Элемент

</div>
<div>

<h3>Блок 3</h3>

- Элемент
- Элемент
- Элемент

</div>
</div>

</div>

---

<!-- ============================================================
     СЛАЙД 7 — Изображение (одно)
     ============================================================ -->
<div class="content-box" style="text-align:center;">

<div class="section-label">Визуализация</div>
<h1 style="text-align:center;">Заголовок над изображением</h1>

<div style="margin-top:10px;">
<img class="fit-image" src="./path/to/your/image.png" alt="Описание" style="max-height:50vh; border-radius:6px;">
</div>

<p style="font-size:12px; color:var(--text-muted); margin-top:8px;">Подпись к изображению (опционально)</p>

</div>

---

<!-- ============================================================
     СЛАЙД 8 — Два изображения рядом
     Используйте class="img-dual"
     ============================================================ -->
<div class="content-box" style="display:flex; align-items:center; justify-content:center;">

<div style="width:100%; text-align:center;">
<div class="section-label">Визуализация</div>
<h1 style="text-align:center;">Два изображения рядом</h1>

<div class="img-dual" style="margin-top:24px;">
<div class="img-card">
<img src="./path/to/image1.png" alt="Image 1">
</div>
<div class="img-card">
<img src="./path/to/image2.png" alt="Image 2">
</div>
</div>
</div>

</div>

---

<!-- ============================================================
     СЛАЙД 9 — Callout + Tag
     ============================================================ -->
<div class="content-box">

<div class="section-label">Компоненты</div>
<h1>Callout и теги</h1>

<p>Используйте <span class="tag">теги</span> <span class="tag">для</span> <span class="tag">ключевых слов</span></p>

<div class="callout">
<strong>Callout-блок:</strong> используйте для выделения важных выводов, инсайтов и предупреждений. Синяя рамка слева привлекает внимание.
</div>

<blockquote>
Блок-цитата (blockquote) — альтернативный вариант для выделения.
</blockquote>

<pre><code>// Блок кода — для листингов
fn example() -> &amp;str {
    "Hello, World!"
}
</code></pre>

</div>

---

<!-- ============================================================
     СЛАЙД 10 — Полноэкранная диаграмма
     ============================================================ -->
<div class="content-box" style="display:flex; align-items:center; justify-content:center;">

<div style="width:100%; text-align:center;">
<div class="section-label">Диаграммы</div>
<h1 style="text-align:center;">Название диаграммы</h1>

<div style="margin-top:18px;">
<img class="fit-image" src="./path/to/diagram.png" alt="Diagram" style="max-height:62vh; border-radius:8px;">
</div>
</div>

</div>

---

<!-- ============================================================
     СЛАЙД 11 — Закрытие
     Используйте class="closing-slide"
     ============================================================ -->
<!-- _class: closing-slide -->

<h1>Название проекта</h1>

<div class="closing-authors">
Ваше Имя &nbsp;•&nbsp; Контакт<br>
Организация &nbsp;•&nbsp; 2026
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
