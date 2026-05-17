use tokio::process::Command;

use reqwest::StatusCode;

pub async fn send_git(notes_path: &str) -> Result<String, StatusCode> {
    if let Err(e) = git_beebos_notes(&notes_path).await {
        eprint!("[ERROR] Failed to send beebos notes {e}");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok("Beebos messages was sent to github !".to_string())
}

pub async fn git_beebos_notes(notes_path: &str) -> Result<(), std::io::Error> {
    Command::new("git")
        .arg("add")
        .arg(&notes_path)
        .output()
        .await?;
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("beebo analysis")
        .output()
        .await?;
    let output = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("main")
        .output()
        .await?;

    if !output.status.success() {
        eprintln!(
            "[ERROR] Git push failed {}",
            String::from_utf8_lossy(&output.stderr)
        );

        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "git push failed",
        ));
    };

    Ok(())
}
