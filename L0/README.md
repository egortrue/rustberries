## Демонстрационный сервис с простейшим интерфейсом, возвращающий данные о заказе

#### Содержание проекта
- `/src/main.rs` - весь код
- `/data/*.json` - JSON-схема и тестировочные данные получены через https://json-schema-faker.js.org/ 
- `/migrations` - SQL-скрипты для проведения миграции БД с помощью https://crates.io/crates/refinery

#### Полезные команды
- чекер кода: `cargo clippy -- -W clippy::pedantic`
- hot-reload: `cargo watch -c -d 0 -x run`

#### Параметры запуска
```
Usage: L0.exe [ADDRESS] [PORT] [ENABLE_POSTGRES]

Arguments:
  [ADDRESS]
  [PORT]
  [ENABLE_POSTGRES]  [possible values: true, false]

Options:
  -h, --help  Print help
```

#### Пример лога

```
2024-09-12T01:15:09.794650Z  INFO ThreadId(01) L0: Listening on http://localhost:3000
2024-09-12T01:19:28.459183Z TRACE ThreadId(02) L0::controllers: GET /order -> get_orders()
2024-09-12T01:19:28.459298Z  INFO ThreadId(02) L0::controllers: GET /order -> get_orders() -> 200 OK
2024-09-12T01:19:33.909354Z TRACE ThreadId(10) L0::controllers: POST /order -> add_order(payload)
2024-09-12T01:19:33.909553Z  INFO ThreadId(10) L0::controllers: POST /order -> add_order(payload) -> 201 Created
2024-09-12T01:19:39.457857Z TRACE ThreadId(10) L0::controllers: GET /order/ASDasd -> get_order(ASDasd)
2024-09-12T01:19:39.458045Z ERROR ThreadId(10) L0::controllers: GET /order/ASDasd -> get_order(ASDasd) -> 404 Not Found
2024-09-12T01:19:50.381117Z TRACE ThreadId(10) L0::controllers: GET /order -> get_orders()
2024-09-12T01:19:50.381306Z  INFO ThreadId(10) L0::controllers: GET /order -> get_orders() -> 200 OK
2024-09-12T01:19:55.302765Z  INFO ThreadId(10) L0::controllers: GET / -> index() -> 200 OK
2024-09-12T01:20:08.370583Z TRACE ThreadId(10) L0::controllers: GET /order/b563feb7b2b84b6test -> get_order(b563feb7b2b84b6test)
2024-09-12T01:20:08.370826Z  INFO ThreadId(10) L0::controllers: GET /order/b563feb7b2b84b6test -> get_order(b563feb7b2b84b6test) -> 200 OK
```