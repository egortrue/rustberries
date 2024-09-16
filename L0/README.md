## Демонстрационный сервис с простейшим интерфейсом, возвращающий данные о заказе

#### Содержание проекта
- `/src/main.rs` - весь код
- `/data/model.json` - Оригинальная модель данных
- `/data/orders.bak` - Сериализованные данные - сохраненное состояние при выходе программы
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
2024-09-16T00:35:50.035374Z  INFO ThreadId(01) L0: Listening on http://localhost:3000    
2024-09-16T00:36:26.596425Z  INFO ThreadId(10) L0::controllers: GET / -> index() -> 200 OK    
2024-09-16T00:36:27.470431Z  INFO ThreadId(02) L0::controllers: GET / -> index() -> 200 OK    
2024-09-16T00:36:28.126579Z  INFO ThreadId(02) L0::controllers: GET / -> index() -> 200 OK    
2024-09-16T00:36:28.953828Z  INFO ThreadId(02) L0::controllers: GET / -> index() -> 200 OK    
2024-09-16T00:36:39.855119Z TRACE ThreadId(02) L0::controllers: GET /order/non-exists -> get_order(non-exists)    
2024-09-16T00:36:39.855229Z ERROR ThreadId(02) L0::controllers: GET /order/non-exists -> get_order(non-exists) -> 404 Not Found    
2024-09-16T00:36:56.466919Z TRACE ThreadId(02) L0::controllers: GET /order/non-exists1 -> get_order(non-exists1)    
2024-09-16T00:36:56.467023Z ERROR ThreadId(02) L0::controllers: GET /order/non-exists1 -> get_order(non-exists1) -> 404 Not Found    
2024-09-16T00:37:04.255022Z TRACE ThreadId(02) L0::controllers: GET /order/b563feb7b2b84b6test -> get_order(b563feb7b2b84b6test)    
2024-09-16T00:37:04.255154Z ERROR ThreadId(02) L0::controllers: GET /order/b563feb7b2b84b6test -> get_order(b563feb7b2b84b6test) -> 404 Not Found    
2024-09-16T00:37:13.063110Z TRACE ThreadId(02) L0::controllers: GET /order -> get_orders()    
2024-09-16T00:37:13.063218Z  INFO ThreadId(02) L0::controllers: GET /order -> get_orders() -> 200 OK    
2024-09-16T00:37:16.097316Z TRACE ThreadId(02) L0::controllers: GET /order -> get_orders()    
2024-09-16T00:37:16.097422Z  INFO ThreadId(02) L0::controllers: GET /order -> get_orders() -> 200 OK    
2024-09-16T00:37:22.772462Z TRACE ThreadId(02) axum::rejection: rejecting request status=415 body="Expected request with `Content-Type: application/json`" rejection_type="axum::extract::rejection::MissingJsonContentType"
2024-09-16T00:37:34.887185Z TRACE ThreadId(02) axum::rejection: rejecting request status=400 body="Failed to parse the request body as JSON: payment.payment_dt: expected value at line 20 column 19" rejection_type="axum::extract::rejection::JsonSyntaxError"
2024-09-16T00:37:42.670075Z TRACE ThreadId(10) L0::controllers: POST /order -> add_order(payload)    
2024-09-16T00:37:42.670200Z  INFO ThreadId(10) L0::controllers: POST /order -> add_order(payload) -> 201 Created    
2024-09-16T00:37:46.621749Z TRACE ThreadId(02) L0::controllers: POST /order -> add_order(payload)    
2024-09-16T00:37:46.621916Z ERROR ThreadId(02) L0::controllers: POST /order -> add_order(payload) -> 409 Conflict [Order with order_uid=b563feb7b2b84b6test already exists!]
2024-09-16T00:37:55.862400Z TRACE ThreadId(02) L0::controllers: GET /order -> get_orders()    
2024-09-16T00:37:55.862601Z  INFO ThreadId(02) L0::controllers: GET /order -> get_orders() -> 200 OK
2024-09-16T00:38:01.440157Z TRACE ThreadId(02) L0::controllers: GET /order/b563feb7b2b84b6test -> get_order(b563feb7b2b84b6test)  

2024-09-16T00:38:01.440410Z  INFO ThreadId(02) L0::controllers: GET /order/b563feb7b2b84b6test -> get_order(b563feb7b2b84b6test) -> 200 OK
2024-09-16T00:38:24.588779Z  INFO ThreadId(02) L0::repository::local: Creating backup file from memory -> data/orders.bak    
2024-09-16T00:38:24.589242Z  WARN ThreadId(02) L0: Exited with SIGINT (Ctrl-C)
```