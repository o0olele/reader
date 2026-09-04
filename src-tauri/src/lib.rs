mod app;
#[path = "command/mod.rs"]
mod command_api;
pub mod domain;
mod error;
mod infrastructure;
mod repository;
mod service;
pub mod source_engine;

use app::AppState;
use tauri::Manager;

#[tauri::command]
fn health_check(state: tauri::State<'_, AppState>) -> Result<String, error::AppError> {
    let connected = state
        .db
        .lock()
        .map_err(|_| error::AppError::Database("state lock poisoned".into()))?
        .is_some();
    if connected {
        Ok(format!("SQLite 已连接 ({})", state.config.app_name))
    } else {
        Ok("SQLite 未初始化".into())
    }
}

pub fn run() {
    tracing_subscriber::fmt().with_env_filter("info").init();
    tauri::Builder::default()
        .manage(AppState::new())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let state = app.state::<AppState>();
            let data_dir = app.path().app_data_dir().map_err(error::AppError::io)?;
            std::fs::create_dir_all(&data_dir).map_err(error::AppError::io)?;
            let db_path = data_dir.join("app.db");
            tauri::async_runtime::block_on(app::bootstrap::initialize_database(&state, &db_path))
                .map_err(error::AppError::database)?;
            tracing::info!(target: "database", path = %db_path.display(), "database initialized");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            health_check,
            command_api::book::import_txt_book_cmd,
            command_api::book::import_epub_book_cmd,
            command_api::book::list_books_cmd,
            command_api::reader::list_chapters_cmd,
            command_api::reader::refresh_catalog_cmd,
            command_api::reader::get_reading_progress_cmd,
            command_api::reader::save_reading_progress_cmd,
            command_api::book::delete_book_cmd,
            command_api::bookshelf::list_groups_cmd,
            command_api::bookshelf::create_group_cmd,
            command_api::bookshelf::move_book_to_group_cmd,
            command_api::source::debug_source_stage_cmd,
            command_api::source::update_book_source_rules_cmd,
            command_api::source::list_book_sources_cmd,
            command_api::source::save_book_source_cmd,
            command_api::source::import_book_sources_json_cmd,
            command_api::source::import_book_sources_url_cmd,
            command_api::search::search_books_cmd,
            command_api::explore::list_explore_categories_cmd,
            command_api::explore::explore_books_cmd,
            command_api::search::test_book_source_cmd,
            command_api::book::add_online_book_cmd,
            command_api::book::fetch_book_info_cmd,
            command_api::book::switch_book_source_cmd,
            command_api::reader::fetch_online_content_cmd,
            command_api::source::login_book_source_cmd,
            command_api::source::clear_book_source_session_cmd,
            command_api::source::get_book_source_session_status_cmd,
            command_api::source::refresh_book_source_session_cmd,
            command_api::source::open_book_source_browser_cmd,
            command_api::source::save_book_source_browser_session_cmd,
            command_api::settings::get_app_settings_cmd,
            command_api::settings::save_app_settings_cmd,
            command_api::settings::report_webview_user_agent_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
