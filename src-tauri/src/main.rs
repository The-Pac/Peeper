#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]


extern crate core;

use std::borrow::Borrow;
use std::result::Result;
use sqlx::{sqlite::SqliteQueryResult, Sqlite, SqlitePool, migrate::MigrateDatabase, Pool, Error, Row};
use sqlx::sqlite::SqliteRow;

#[async_std::main]
async fn main() {
    check_data_base().await;
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_cards,add_card,move_card,get_categories])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

static DATA_BASE_URL: &str = "sqlite://database.db";

async fn check_data_base() {
    if !Sqlite::database_exists(&DATA_BASE_URL).await.unwrap_or(false) {
        Sqlite::create_database(&DATA_BASE_URL).await.unwrap();
        match schema_database().await {
            Ok(_) => print!("database created"),
            Err(e) => panic!("error create database : {:?}", e)
        }
    }
}

async fn schema_database() -> Result<SqliteQueryResult, Error> {
    let pool: Pool<Sqlite> = SqlitePool::connect(&DATA_BASE_URL).await.unwrap();
    let query: &str =
        "CREATE TABLE cards (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT,
            category_id TEXT
        );
        CREATE TABLE categories (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT,
            category_id TEXT
        );";

    let result = sqlx::query(&query).execute(&pool).await;
    pool.close().await;

    return result;
}

struct Card {
    id: i32,
    title: String,
    category_id: String,
}

struct Category {
    id: i32,
    category_id: String,
    title: String,
}

#[tauri::command]
async fn get_cards() {
    let instance: Pool<Sqlite> = SqlitePool::connect(&DATA_BASE_URL).await.unwrap();
    let query = "SELECT * FROM cards;";
    let result = sqlx::query(&query).execute(&instance).await;
    instance.close().await;
    print!("{:?}", result);
}

#[tauri::command]
async fn move_card(id: i32) {
    println!("{}", id);
}

#[tauri::command]
async fn add_card(category_id: String) {
    print!("{}", category_id);
    let pool: Pool<Sqlite> = SqlitePool::connect(&DATA_BASE_URL).await.unwrap();
    let mut query: &str = "INSERT INTO cards (title,category_id) VALUES('' ,$1)";

    sqlx::query(&query)
        .bind(category_id)
        .execute(&pool)
        .await
        .expect("TODO: panic message");

    query = "SELECT * FROM cards";

    let cards = sqlx::query(&query)
        .fetch_all(&pool)
        .await
        .expect("TODO: panic message");

    pool.close().await;

    cards;

    /*Ok(Card {
        id: result.id,
        title: result.title,
        category_id: result.category_id,
    }).expect("TODO: panic message");*/
}

#[tauri::command]
async fn get_categories() {
    let instance: Pool<Sqlite> = SqlitePool::connect(&DATA_BASE_URL).await.unwrap();
    let query = "SELECT * FROM categories;";
    let result = sqlx::query(&query).execute(&instance).await;
    instance.close().await;

    print!("{:?}", result);
}

