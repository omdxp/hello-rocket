use std::io;

use rocket::tokio::time::{sleep, Duration};

use rocket::tokio::task::spawn_blocking;

#[get("/delay/<second>")]
pub async fn delay(second: u64) -> String {
    sleep(Duration::from_secs(second)).await;
    format!("Delayed for {} seconds", second)
}

#[get("/blocking_task")]
pub async fn blocking_task() -> io::Result<Vec<u8>> {
    // In a real app, use rocket::fs::NamedFile or tokio::fs::File
    let vec = spawn_blocking(|| std::fs::read("data.txt"))
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}
