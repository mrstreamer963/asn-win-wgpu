# ASN Web Bus Example

Этот пример демонстрирует использование ASN Bus в веб-среде с помощью WebAssembly и `trunk`.

## Структура проекта

- `src/` - Исходный код Rust
- `index.html` - Главная HTML-страница
- `Cargo.toml` - Файл конфигурации проекта

## Требования

- Rust и Cargo
- wasm32-unknown target
- trunk

## Установка

1. Установите Rust через rustup:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Добавьте wasm32 target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. Установите trunk:
   ```bash
   cargo install trunk
   ```

## Запуск

Для запуска примера в режиме разработки выполните:
```bash
trunk serve
```

Сервер автоматически:
1. Скомпилирует проект в WebAssembly
2. Запустит локальный сервер для разработки
3. Откроет браузер по адресу http://localhost:8080

### Если порт уже занят

Если порт 8080 уже занят, вы можете указать другой порт:
```bash
trunk serve --port 8081
```

## Сборка

Для сборки проекта выполните:
```bash
trunk build
```

Скрипт автоматически:
1. Скомпилирует проект в WebAssembly
2. Создаст оптимизированную сборку в директории `dist`

## Функциональность

Пример демонстрирует работу с шиной событий ASN Bus веб-среде:

- **Инициализация приложения** - инициализирует шину событий и отправляет тестовые сообщения
- **Получение версии** - отображает версию приложения
- **Отправка TaskUpdate** - отправляет сообщение типа TaskUpdate через шину событий
- **Отправка TaskNone** - отправляет сообщение типа TaskNone через шину событий
- **Получение сообщения** - получает сообщение из шины событий

Все функции доступны через кнопки на веб-странице после инициализации WASM-модуля.


Можно расширить как 

```JavaScript
export default function myInitializer () {
  return {
    onStart: () => {
      // called when the loading starts
    },
    onProgress: ({current, total}) => {
      // the progress while loading, will be called periodically.
      // "current" will contain the number of bytes of the WASM already loaded
      // "total" will either contain the total number of bytes expected for the WASM, or if the server did not provide
      //   the content-length header it will contain 0.
    },
    onComplete: () => {
      // called when the initialization is complete (successfully or failed)
    },
    onSuccess: (wasm) => {
      // called when the initialization is completed successfully, receives the `wasm` instance
    },
    onFailure: (error) => {
      // called when the initialization is completed with an error, receives the `error`
    }
  }
};
```

Для корректной сборки либы getrandom приходится не только прописывать у нее features = ["wasm_js"], 
но и явно прописывать в файле ./cargo/config.toml

```sh
[build]
rustflags = ["--cfg", "getrandom_backend=\"wasm_js\""]
```

Типа указывая фичу мы включаем эту возможность в коде, а указывая флаг - говорить явно использовать ее *facepalm*
Типа для "более тонкой настройки и 