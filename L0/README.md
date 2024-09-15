## Демонстрационный сервис с простейшим интерфейсом, возвращающий данные о заказе

#### Содержание проекта
- `/src/main.rs` - весь код
- `/data/*.json` - JSON-схема и тестировочные данные получены через https://json-schema-faker.js.org/ 
- `/migrations` - SQL-скрипты для проведения миграции БД с помощью https://crates.io/crates/refinery

#### Полезные команды
- проверка: `cargo clippy -- -W clippy::pedantic`
- запуск: `cargo watch -c -d 0 -x run`
- запуск с PG: `cargo watch -c -d 0 -x "run -- --pg-enable --pg-user myuser --pg-pass mypassword --pg-host localhost --pg-port 5432 --pg-name postgres"`

#### Параметры запуска
```
$ .\target\debug\L0.exe -h
Usage: L0.exe [OPTIONS]

Options:
  -a, --address <ADDRESS>        [default: localhost]
  -p, --port <PORT>              [default: 3000]
      --pg-enable
      --pg-user <PG_USER>
      --pg-pass <PG_PASS>
      --pg-host <PG_HOST>
      --pg-port <PG_PORT>
      --pg-name <PG_NAME>
      --pg-migrate <PG_MIGRATE>
  -h, --help                     Print help
```

#### Пример неверных параметров запуска
```
$ .\target\debug\L0.exe --pg-enable
error: the following required arguments were not provided:
  <--pg-user <PG_USER>|--pg-pass <PG_PASS>|--pg-host <PG_HOST>|--pg-port <PG_PORT>|--pg-name <PG_NAME>>

Usage: L0.exe --pg-enable <--pg-user <PG_USER>|--pg-pass <PG_PASS>|--pg-host <PG_HOST>|--pg-port <PG_PORT>|--pg-name <PG_NAME>>  

For more information, try '--help'.
```

#### Пример лога

```
2024-09-12T03:36:22.304278Z  INFO ThreadId(01) L0: Listening on http://localhost:3000
2024-09-12T03:36:27.471893Z  INFO ThreadId(02) L0::controllers: GET / -> index() -> 200 OK
2024-09-12T03:36:28.687600Z  INFO ThreadId(08) L0::controllers: GET / -> index() -> 200 OK
2024-09-12T03:36:29.018279Z  INFO ThreadId(08) L0::controllers: GET / -> index() -> 200 OK
2024-09-12T03:36:29.372861Z  INFO ThreadId(08) L0::controllers: GET / -> index() -> 200 OK
2024-09-12T03:36:35.365846Z TRACE ThreadId(08) L0::controllers: GET /order -> get_orders()
2024-09-12T03:36:35.366047Z  INFO ThreadId(08) L0::controllers: GET /order -> get_orders() -> 200 OK
2024-09-12T03:36:36.300651Z TRACE ThreadId(08) L0::controllers: GET /order -> get_orders()
2024-09-12T03:36:36.300823Z  INFO ThreadId(08) L0::controllers: GET /order -> get_orders() -> 200 OK
2024-09-12T03:36:40.437483Z TRACE ThreadId(08) L0::controllers: POST /order -> add_order(payload)
2024-09-12T03:36:40.437687Z  INFO ThreadId(08) L0::controllers: POST /order -> add_order(payload) -> 201 Created
2024-09-12T03:36:45.695669Z TRACE ThreadId(02) L0::controllers: POST /order -> add_order(payload)
2024-09-12T03:36:45.695864Z ERROR ThreadId(02) L0::controllers: POST /order -> add_order(payload) -> 409 Conflict [Order with order_uid="b563feb7b2b84b6test" already exists!]
2024-09-12T03:36:50.248939Z TRACE ThreadId(02) L0::controllers: POST /order -> add_order(payload)
2024-09-12T03:36:50.249143Z  INFO ThreadId(02) L0::controllers: POST /order -> add_order(payload) -> 201 Created
2024-09-12T03:36:59.107522Z TRACE ThreadId(02) L0::controllers: GET /order -> get_orders()
2024-09-12T03:36:59.107743Z  INFO ThreadId(02) L0::controllers: GET /order -> get_orders() -> 200 OK
2024-09-12T03:37:05.258213Z TRACE ThreadId(02) L0::controllers: GET /order/b563feb7b2b84b6test -> get_order(b563feb7b2b84b6test)
2024-09-12T03:37:05.258484Z  INFO ThreadId(02) L0::controllers: GET /order/b563feb7b2b84b6test -> get_order(b563feb7b2b84b6test) -> 200 OK
2024-09-12T03:37:18.090899Z TRACE ThreadId(02) L0::controllers: GET /order/adsczews -> get_order(adsczews)
2024-09-12T03:37:18.091087Z ERROR ThreadId(02) L0::controllers: GET /order/adsczews -> get_order(adsczews) -> 404 Not Found
```