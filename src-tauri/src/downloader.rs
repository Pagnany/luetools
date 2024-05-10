use std::fs::File;
use std::io::{self, Write};
use std::time::Duration;

fn read_urls(path: &str) -> io::Result<Vec<String>> {
    let file_string = std::fs::read_to_string(path)?;
    let mut urls = Vec::new();

    for line in file_string.lines() {
        urls.push(line.trim().to_string());
    }

    urls.sort();
    urls.dedup();

    Ok(urls)
}

pub fn download_files_from_csv(input: &str, output: &str) -> String {
    let mut faults = String::new();
    let mut fault_count = 0;
    if let Ok(urls) = read_urls(input) {
        for url in urls {
            if let Some(file_name) = url.split('/').last() {
                let path = format!("{}{}", output, file_name);
                if let Ok(_) = download_file(&url, path.as_str()) {
                    // alles läuft
                } else {
                    // Fehler beim Herunterladen oder speichern
                    fault_count += 1;
                    faults = format!("{}<br>{}: {}", faults, fault_count, url);
                }
            }
        }
        if faults.is_empty() {
            return "Fertig".to_string();
        } else {
            return format!("Fertig<br>Fehler bei folgenden Links:<br>{}", faults);
        }
    } else {
        return "Fehler beim lesen der CSV".to_string();
    }
}

fn download_file(url: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if std::path::Path::new(&path).exists() {
        return Ok(());
    }
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;
    let response = client.get(url).send()?;
    if response.status() != 200 {
        return Err(Box::from("HTML Fehler Code"));
    }
    let bytes = response.bytes()?;
    let mut file = File::create(path)?;
    file.write_all(&bytes)?;

    Ok(())
}
