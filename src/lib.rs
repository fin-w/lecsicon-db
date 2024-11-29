pub mod definitions;
mod implementations;
mod schema;

use std::boxed::Box;
use std::error::Error;
use std::path::Path;
use std::rc::Rc;
use std::result::Result;

pub fn connection(
    db_file_path: &Path,
) -> Result<Rc<diesel::SqliteConnection>, Box<(dyn Error + 'static)>> {
    implementations::establish_connection(db_file_path)
}

pub fn save_csv_as_sqlite_db(csv_file_path: &Path, db_file_path: &Path) {
    println!("loading data from csv...");
    if let Some(data) = implementations::load_text_lecsicon(csv_file_path) {
        if let Ok(mut db_connection) = implementations::establish_connection(db_file_path) {
            println!("converting loaded data to sqlite-ready format...");
            let sqlite_ready_data = implementations::convert_csv_to_sqlite_format(data);
            println!("saving data to the database...");
            if implementations::save_data_to_sqlite_db(sqlite_ready_data, db_file_path) {
                println!("attempting to search...");
                if search("meddwl", Rc::get_mut(&mut db_connection).unwrap()).is_some()
                    && search("iawn", Rc::get_mut(&mut db_connection).unwrap()).is_some()
                    && search("yr", Rc::get_mut(&mut db_connection).unwrap()).is_some()
                {
                    println!("data confirmed in db");
                } else {
                    println!("no search results, data not confirmed in db");
                }
            } else {
                println!("saving failed");
            }
        }
    } else {
        println!("no csv data");
    }
}

pub fn search(
    word: &str,
    db_connection: &mut diesel::SqliteConnection,
) -> Option<Vec<definitions::LecsiconEntry>> {
    let results = implementations::get_lecsicon_entries_by_word(word, db_connection)
        .into_iter()
        .map(implementations::convert_sqlite_to_text)
        .collect::<Vec<definitions::LecsiconEntry>>();
    if results.is_empty() {
        None
    } else {
        Some(results)
    }
}

pub fn related(
    entry: definitions::LecsiconEntry,
    db_connection: &mut diesel::SqliteConnection,
) -> Option<Vec<definitions::LecsiconEntry>> {
    implementations::get_related_entries(entry, db_connection)
}

pub fn text_prompt(db_file_path: &Path, recent_searches: Vec<String>) -> Option<inquire::Text<'_>> {
    if let Ok(db_connection) = implementations::establish_connection(db_file_path) {
        Some(
            inquire::Text::new("Chwilio: ").with_autocomplete(definitions::SqliteSearcher::new(
                db_connection,
                recent_searches.into_iter().rev().collect::<Vec<String>>(),
            )),
        )
    } else {
        None
    }
}
