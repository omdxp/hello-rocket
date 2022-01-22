use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};

#[get("/<file..>")]
pub async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}
