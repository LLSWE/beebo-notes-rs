use tokio::{fs::OpenOptions, io::AsyncWriteExt};

use reqwest::StatusCode;

pub async fn write_beebo_notes(
    note_path: &String,
    note_paragraph: String,
) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&note_path)
        .await?;

    file.write_all(note_paragraph.as_bytes()).await?;

    Ok(())
}
