#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use rusqlite::{Connection, Result};

const CONNEXION: Result<Connection, rusqlite::Error> = Connection::open("database.db");

fn main() {
    let connexion: Result<Connection, rusqlite::Error> = Connection::open("database.db");
    connexion.execute(
        "CREATE TABLE cards (
                  id              INTEGER PRIMARY KEY,
                  has_date            INTEGER,
                  date_from           TEXT
                  date_to             TEXT
                  title               TEXT
                  category_id         TEXT
                  ),
          CREATE TABLE categories (
                  id              INTEGER PRIMARY KEY,
                  title               TEXT
                  category_id         TEXT
                  )
                  ",
        [],
    );
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_cards,set_card,get_categories])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

struct Card {
    id: i32,
    has_date: bool,
    date_from: String,
    date_to: String,
    title: String,
    category_id: String,
}

struct Category {
    id: i32,
    category_id: String,
    title: String,
}

/*#[tauri::command]
fn get_cards() -> Result<()> {
    let mut request = CONNEXION.prepare(
        "SELECT * FROM cards;",
    ).expect("rows failed");

    let cards = request.query_map(|row| {
        Ok(Card {
            id: row.get(0)?: i32,
            has_date: row.get(1)?: bool,
            date_from: row.get(2)?.to_string(),
            date_to: row.get(3)?.to_string(),
            title: row.get(4)?.to_string(),
            category_id: row.get(5)?.to_string(),
        })
    })?;

    for card in cards {
        println!("Found {:?}", card);
    }
    Ok(())
}

#[tauri::command]
fn set_card() {}

#[tauri::command]
fn get_categories() -> Result<()> {
    let mut request = CONNEXION.prepare(
        "SELECT * FROM categories;",
    ).expect("rows failed");

    let cards = request.query_map(|row| {
        Ok(Category {
            id: row.get(0)?: i32,
            category_id: row.get(0)?.to_string(),
            title: row.get(0)?.to_string(),
        })
    })?;

    for card in cards {
        println!("Found {:?}", card);
    }
    Ok(())
}*/

