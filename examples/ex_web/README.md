# ASN Web Example

Этот пример демонстрирует использование ASN WGPU в веб-среде с помощью WebAssembly.

## Структура проекта

- `src/` - Исходный код Rust
- `index.html` - Главная HTML-страница
- `build.sh` - Скрипт для сборки проекта
- `run.sh` - Скрипт для запуска локального сервера
- `clean.sh` - Скрипт для очистки сборочных артефактов

## Требования

- Rust и Cargo
- wasm32-unknown target
- wasm-bindgen-cli

## Установка

1. Установите Rust через rustup:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Добавьте wasm32 target:
   ```bash
   rustup target add wasm32-unknown
   ```

3. Установите wasm-bindgen-cli:
   ```bash
   cargo install wasm-bindgen-cli
   ```

## Сборка

Для сборки проекта выполните:
```bash
./build.sh
```

Скрипт автоматически:
1. Скомпилирует проект для wasm32-unknown-unknown target
2. Сгенерирует JavaScript биндинги с помощью wasm-bindgen
3. Создаст директорию `dist` с готовым веб-приложением

Для очистки предыдущих сборок перед сборкой:
```bash
./build.sh --clean
```

## Запуск

Для запуска локального сервера выполните:
```bash
./run.sh
```

Скрипт автоматически:
1. Проверит наличие сборки, и при необходимости соберет проект
2. Запустит локальный HTTP сервер
3. Откроет браузер по адресу http://localhost:8091

## Очистка

Для очистки сборочных артефактов выполните:
```bash
./clean.sh
```

## Ручной запуск

Если вы хотите запустить сервер вручную, вы можете использовать любой HTTP сервер:

### Python 3
```bash
python3 -m http.server 8091 -d dist
```

### Python 2
```bash
python -m SimpleHTTPServer 8091 dist
```

### PHP
```bash
php -S localhost:8091 -t dist
```

После запуска сервера откройте в браузере http://localhost:8091
