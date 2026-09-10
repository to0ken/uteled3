--  инициализация и создание таблиц сообщений мессанджера

CREATE TABLE IF NOT EXISTS messages(
  -- создаем ид сообщения
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    author TEXT NOT NULL,

    -- ТЕЛО СООБЩЕНИЯ
    body TEXT NOT NULL,

    -- время отправки
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP

);