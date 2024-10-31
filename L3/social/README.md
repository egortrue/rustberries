# L3.1 Мини-социальная сеть для обмена сообщениями

## Особенности реализации
- Чистая архитектура
- [Скрипт](./docker-compose.yml) Docker Compose для запуска БД PostgreSQL
- [Скрипт](./migrations/init.sql) инициализации таблиц БД (встроен в запуск docker compose)

## Описание таблиц БД
```sql
CREATE TABLE users (
    username VARCHAR PRIMARY KEY,
    password_hash VARCHAR
);
CREATE TABLE posts (
    id UUID PRIMARY KEY,
    author VARCHAR REFERENCES users(username),
    content VARCHAR,
    likes INT
);
CREATE TABLE likes (
    post UUID REFERENCES posts(id) ON DELETE CASCADE,
    username VARCHAR REFERENCES users(username),
    PRIMARY KEY (post, username)
);
```