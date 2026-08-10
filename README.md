# 🦀 Rust Learning

Личный курс изучения Rust — от синтаксиса до HTML-сайта с бэкендом на Rust.

**Бэкграунд:** ~3-4 года на Python, пробовал HTML/CSS/JS/TS/Java/C++/C#/Lua, но синтаксис Rust зашёл больше всего.

**Цель:** уметь писать полноценный веб-бэкенд на Rust (Axum + SQLite) и раздавать через него простую HTML-страницу.

---

## 📂 Структура

```
rust-learning/
├── stage-1-syntax/              # переменные, функции, if/match как expressions
├── stage-2-ownership/           # ownership, borrowing, lifetimes
├── stage-3-structs-traits/      # структуры, enum, трейты, итераторы
├── stage-4-async-ecosystem/     # cargo-экосистема, async/tokio, serde, reqwest
├── stage-5-web-backend/         # Axum, sqlx/SQLite, REST API
└── README.md
```

Каждая задача — отдельный `cargo` проект внутри своего этапа:
```
stage-N-name/
└── task-M-short-description/
    ├── src/main.rs
    └── Cargo.toml
```

---

## 🗺️ Roadmap

| Этап | Тема | Статус |
|---|---|---|
| 0 | Установка rustup/cargo/rustlings | ✅ |
| 1 | Синтаксис: переменные, функции, match | ✅ |
| 2 | Ownership, borrowing, lifetimes | 🔄 в процессе |
| 3 | Структуры, enum, трейты, итераторы | ⬜ |
| 4 | Async, tokio, serde, reqwest | ⬜ |
| 5 | Веб-бэкенд: Axum + SQLite | ⬜ |
| 🏁 | Финал: HTML + Rust backend | ⬜ |

---

## 🏆 Проекты-награды по ходу курса

- 🎲 CLI игра "угадай число" (после ownership)
- 📜 Текстовый квест с enum-состояниями (после structs/traits)
- ⛅ CLI-погода через публичное API (после async/serde/reqwest)
- 📝 REST API для todo-листа (Axum + SQLite)
- 🌐 Финал: HTML-страница + Rust backend

---

## 🛠️ Инструменты

- `rustup` — управление версиями Rust
- `cargo clippy` — линтер, идиоматичность кода
- [`rustlings`](https://github.com/rust-lang/rustlings) — интерактивные упражнения
- [The Rust Book](https://doc.rust-lang.org/book/) — справочник по концепциям

---

## 📌 Правила курса

- Короткие циклы (3-5 дней на этап), не растягивать
- Одна "весёлая" бесполезная Rust-программа раз в неделю
- Если завис на задаче >40 минут — смотреть подсказку/решение, это нормально
- После каждого этапа — пауза, а не гонка дальше