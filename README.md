# Конвертер температур

![Rust](https://img.shields.io/badge/lang-Rust-orange?logo=rust) ![CLI](https://img.shields.io/badge/interface-CLI-lightgrey)

Простое консольное приложение для конвертации между шкалами Цельсия и Фаренгейта, написанное на Rust.

## 🚀 Возможности

- Конвертация:
  - Цельсий → Фаренгейт
  - Фаренгейт → Цельсий
- Интерактивное меню с цветовым оформлением
- Корректная обработка ошибок ввода
- Точный вывод результатов (2 знака после запятой)
- Поддержка выхода из программы

## 📦 Установка

1. Убедитесь, что установлен [Rust](https://www.rust-lang.org/tools/install)
2. Клонируйте репозиторий:

```bash
git clone https://github.com/devstrk/temperature-converter.git
cd temperature-converter
```

3. Соберите и запустите приложение:

```bash
cargo build --release
```

## 🖥️ Использование

1. Запустите программу:

```bash
cargo run --release
```

2. Следуйте инструкциям:

```bash
Введите значение температуры для конвертации

> 25

Конвертировать в ...

1. °C → °F (Цельсия в Фаренгейты)
2. °F → °C (Фаренгейты в Цельсия)
3. Выход

> 1

──────────────────────────────

25.00°C = 77.00°F

──────────────────────────────

```

## 🛠️ Технические детали

- Архитектура:
  - Чистое разделение логики (модель-представление)
  - Использование перечислений (enum) для типов температур
  - Константные данные для вариантов конвертации
- Особенности Rust:
  - Идиоматичная обработка ошибок
  - Использование трейтов и методов
  - Безопасность типов

## 📝 Лицензия

<a href="LICENSE"><img src="https://img.shields.io/static/v1.svg?style=for-the-badge&label=License&message=MIT&logoColor=d9e0ee&colorA=363a4f&colorB=b7bdf8"/></a>

См. файл <a href="LICENSE">LICENSE</a>.

---

Разработано с ❤️ на Rust | [devstrk](https://github.com/devstrk)
