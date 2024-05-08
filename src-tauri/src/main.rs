// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;

mod bmecat;
mod downloader;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            greet,
            bmecat_to_picture_csv,
            download_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn bmecat_to_picture_csv(inputpath: &str, outputpath: &str, preset: i32) -> String {
    if let Ok(file_string) = std::fs::read_to_string(inputpath) {
        let bmecat = bmecat::read_bmecat(file_string);
        match preset {
            1 => {
                if let Ok(()) = article_picture_to_csv_one_line(bmecat.article, outputpath) {
                    return "Fertig".to_string();
                } else {
                    return "Fehler beim Schreiben der CSV".to_string();
                }
            }
            2 => {
                if let Ok(()) = article_pictures_to_csv(bmecat.article, outputpath) {
                    return "Fertig".to_string();
                } else {
                    return "Fehler beim Schreiben der CSV".to_string();
                }
            }
            _ => return "Keine valide Option".to_string(),
        }
    } else {
        return "Fehler beim Lesen der BMEcat".to_string();
    }
}

fn article_pictures_to_csv(articles: Vec<bmecat::Article>, outputpath: &str) -> Result<()> {
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b';')
        .from_path(outputpath)?;

    for article in articles {
        let pictures = article.get_pictures();

        for picture in pictures {
            wtr.write_record(&[article.id.clone(), picture])?;
        }
    }
    wtr.flush()?;
    Ok(())
}

fn article_picture_to_csv_one_line(articles: Vec<bmecat::Article>, outputpath: &str) -> Result<()> {
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b';')
        .quote_style(csv::QuoteStyle::Never)
        .from_path(outputpath)?;

    for article in articles {
        let pictures = article.get_pictures();

        wtr.write_record(&[article.id.clone(), pictures.join(";")])?;
    }
    wtr.flush()?;
    Ok(())
}

#[tauri::command]
fn download_files(inputfile: &str, outputpath: &str) -> String {
    downloader::download_files_from_csv(inputfile, outputpath)
}
