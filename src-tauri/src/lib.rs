
// ИМПАРТИРУЕМ ТИПЫ для megrations
use tauri_plugin_sql::{Migration, MigrationKind};

// аннотация для мобильных устройств
#[cfg_attr(mobile, tauri::modile_entry_point)]

// ГЛАВНАЯ ФУН ЗАПУСКА ПРИЛОЖЕНИЯ
pub fn run() {
    // список миграций
     let migrations = vec![
         Migration {
             version: 1,
             description: "create_message_table",
             sql: include_str!("../migrations/0000_initial.sql"),
             kind: MigrationKind::Up, // сдвиг базы вперед
         },
     ];

    // создаем сборщик приложения
    tauri::Builder::default()
    .plugin(tauri_plugin_sql::Builder::new().build())
    // подключаем sql плагин
        .plugin(
            tauri_plugin_sql::Builder::default()
            // связываем миграции с sql базой
                .add_migrations("@sqlite:messenger.db", migrations)
                .build(),

        )
    // создаем plugin opener из стантарного шаблона tauri
        .plugin(tauri_plugin_opener::init())
    // запуск приложения
        .run(tauri::generate_context!())
    // если запучк завегршиться с ощшибкой сообщаем об этом
        .expect("error while running tauri application");
}
