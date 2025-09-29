<img src="https://r2cdn.perplexity.ai/pplx-full-logo-primary-dark%402x.png" style="height:64px;margin-right:32px"/>

# зачем нужно прописывать Set RUSTFLAGS='--cfg getrandom_backend="wasm_js"' почему недостаточно просто указать в конфигурации  getrandom = { version = "0.3", features = ["wasm_js"] }

Для библиотеки getrandom с версии 0.3 при сборке для WebAssembly (wasm32-unknown-unknown) недостаточно просто указать в Cargo.toml зависимость с фичей "wasm_js". Нужно дополнительно настроить компилятор с помощью RUSTFLAGS:

```bash
Set RUSTFLAGS='--cfg getrandom_backend="wasm_js"'
```

Почему?

- Фича `"wasm_js"` лишь включает код поддержки соответствующего бэкенда библиотеки getrandom, но **не выбирает этот бэкенд автоматически**. Выбор конкретного бэкенда делается через конфигурационный флаг компилятора `getrandom_backend`.
- Чтобы код библиотеки с подходящим wasm бэкендом попадал в итоговый бинарь, компилятору нужно сообщить, что необходимо использовать именно этот бэкенд. Для этого используется условная компиляция с помощью `--cfg getrandom_backend="wasm_js"`.
- Если не указать этот флаг в RUSTFLAGS, компиляция для wasm-цели не найдёт подходящий бэкенд, и будет ошибка с рекомендацией именно о необходимости указать этот флаг.
- Попытки указать только фичу в Cargo.toml не работают, потому что библиотека getrandom в 0.3 и выше разделяет включение кода (feature) и выбор backend (cfg-флаг) для более тонкой настройки и предотвращения ошибок.
- Поэтому поместить `--cfg getrandom_backend="wasm_js"` в переменную среды RUSTFLAGS при сборке WebAssembly — это обязательный шаг для правильного выбора backend.

Резюмируя:

- `features = ["wasm_js"]` включает код поддержки wasm backend в getrandom,
- а `Set RUSTFLAGS='--cfg getrandom_backend="wasm_js"'` — выбирает этот backend для компиляции.
Без последнего сборка для wasm не пройдёт успешно.[^1][^3][^5]
<span style="display:none">[^10][^11][^12][^13][^14][^15][^16][^17][^2][^4][^6][^7][^8][^9]</span>

<div style="text-align: center">⁂</div>

[^1]: https://stackoverflow.com/questions/79479737/setting-rustflags-for-a-specific-target

[^2]: https://users.rust-lang.org/t/getrandom-0-3-2-wasm-js-feature-issue/127584

[^3]: https://github.com/rust-random/getrandom

[^4]: https://github.com/rust-random/getrandom/issues/208

[^5]: https://users.rust-lang.org/t/getrandom-version-conflict-error-building-a-wasm/132355

[^6]: https://github.com/uuid-rs/uuid/issues/792

[^7]: https://forum.dfinity.org/t/solution-to-the-wbindgen-placeholder-problem-generated-by-the-0-3-x-version-of-getrandom/43628

[^8]: https://github.com/rust-random/rand/issues/886

[^9]: https://www.reddit.com/r/rust/comments/1m8qxgy/suffering_from_rustwasm_version_conflict/

[^10]: https://searchfox.org/comm-central/source/third_party/rust/getrandom/src/backends.rs

[^11]: https://docs.diesel.rs/master/src/getrandom/backends.rs.html

[^12]: https://github.com/rust-random/rand/issues/1110

[^13]: https://lib.rs/crates/getrandom/features

[^14]: https://users.rust-lang.org/t/getrandom-0-3-2-wasm-js-feature-issue/127584/2

[^15]: https://gitlab.torproject.org/tpo/anti-censorship/lox/-/merge_requests/325/pipelines

[^16]: https://github.com/rust-random/getrandom/blob/master/src/backends/wasm_js.rs

[^17]: https://github.com/rust-random/getrandom/pull/149



---

<img src="https://r2cdn.perplexity.ai/pplx-full-logo-primary-dark%402x.png" style="height:64px;margin-right:32px"/>

# Чем отличаются флаги cfg и features в Rust

В Rust флаги `cfg` и `features` связаны, но служат разным целям:

- **cfg (configuration flags)** — это макросы и атрибуты компилятора, которые управляют условной компиляцией кода. Они говорят компилятору включать или исключать определённые части кода на основе различных условий (платформа, архитектура, наличие фичи и др.). Например, `#[cfg(target_os = "windows")]` включит код только для Windows, а `#[cfg(feature = "my-feature")]` — только если активирована фича `my-feature`. Эти условия задаются либо через предопределённые переменные (target_os, target_arch и др.), либо через пользовательские, такие как features.
- **features** — это механизм Cargo для включения или отключения функциональных возможностей (фич) библиотек и приложений при сборке. Они определяются в секции `[features]` в Cargo.toml и могут быть активированы при вызове cargo командой `--features`. Features упрощают конфигурирование зависимостей и позволяют выбирать, какой функционал включать в билд, тем самым уменьшая размер бинарника и улучшая производительность.

Взаимосвязь: чтобы условно скомпилировать код, зависящий от фичи, в коде пишут `#[cfg(feature = "my-feature")]`, а сама фича управляется через Cargo features.

Кратко:


| Флаг | Назначение | Использование | Определяется где? |
| :-- | :-- | :-- | :-- |
| cfg | Условная компиляция | В коде через `#[cfg(...)]`, `#[cfg_attr(...)]` | Компилятором, с помощью ключей компиляции или features |
| features | Включение/отключение функционала | В Cargo.toml секция `[features]`, активируются через `cargo build --features` | В Cargo.toml пакета |

Таким образом, `features` управляют настройкой конфигурации проекта на уровне сборки (включить/выключить функционал), а `cfg` — говорит компилятору, какой код именно включать в итоговый бинарник в зависимости от этих и других условий. Для условной компиляции по фичам всегда пишут `#[cfg(feature = "feature-name")]`.

Это два взаимодополняющих инструмента для создания гибко настраиваемого и оптимизированного кода в Rust.[^1][^2][^9]
<span style="display:none">[^10][^3][^4][^5][^6][^7][^8]</span>

<div style="text-align: center">⁂</div>

[^1]: https://nuancesprog.ru/p/17375/

[^2]: https://nweb42.com/books/rust/upravlenie-konfiguratsiej-i-kompilyatsiej-s-atributami/

[^3]: https://stackoverflow.com/questions/72908829/what-is-the-difference-between-cfgtest-and-cfgfeature-test

[^4]: https://habr.com/ru/companies/ruvds/articles/697772/

[^5]: https://www.worthe-it.co.za/blog/2018-11-18-compile-time-feature-flags-in-rust.html

[^6]: https://www.reddit.com/r/rust/comments/df1aj6/different_dependency_crates_features_depending_on/

[^7]: https://microsin.net/programming/arm/rust-conditional-compilation-vs-c-preprocessor.html

[^8]: https://www.reddit.com/r/rust/comments/hv2mn6/how_to_check_a_feature_flag_for_an_item_defined/

[^9]: https://doc.rust-lang.org/cargo/reference/features.html

[^10]: https://blog.rng0.io/compile-time-feature-flags-in-rust-why-how-and-when/


---

<img src="https://r2cdn.perplexity.ai/pplx-full-logo-primary-dark%402x.png" style="height:64px;margin-right:32px"/>

# разделяет включение кода (feature) и выбор backend (cfg-флаг) для более тонкой настройки и предотвращения ошибок - уточни что именно ?

Разъяснение по разделению включения кода (feature) и выбору backend (cfg-флаг) в getrandom:

- **Feature** в Cargo.toml (`features = ["wasm_js"]`) просто включает в проект исходники поддержки этого backend, то есть сам код, связанный с wasm_js, становится доступен для компиляции. Это значит, что компилятор имеет в дереве зависимостей нужный код, но где именно его использовать — не определяется на уровне feature.
- **cfg-флаг** (например, `--cfg getrandom_backend="wasm_js"`) говорит компилятору **выбрать определённый backend именно для активации**, то есть сгенерировать исполняемый код именно с реализацией для wasm_js. Он управляет условной компиляцией конкретного фрагмента кода, который реализует выбранный backend. То есть, из всех включённых feature-компонентов только один backend реально используется.

Такое разделение даёт:

- гибкость — можно в одном проекте включить поддержку нескольких backend-ов (через features), но выбрать, какой реально использовать при сборке (через cfg);
- предотвращение конфликтов — один момент, когда проект может ошибочно включить или использовать неправильный backend, если выбирать backend напрямую через features без отдельного cfg;
- возможность точной настройки без усложнения механизма управления фичами в Cargo.

В getrandom таким образом фича лишь "включает" код, а cfg-флаг выбирает, какой backend компилировать и использовать, что обеспечивает чистую и безопасную настройку сборки для разных целей (например, wasm_js vs wasm_rust) без конфликтов и двусмысленностей.[^2][^3]
<span style="display:none">[^1]</span>

<div style="text-align: center">⁂</div>

[^1]: http://www.bog.pp.ru/work/linux-distribution.html

[^2]: https://stackoverflow.com/questions/79479737/setting-rustflags-for-a-specific-target

[^3]: https://github.com/rust-random/getrandom

