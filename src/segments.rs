use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};

#[get("/<file..>")]
pub async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[get("/foo/<_>/bar")]
pub fn foo_bar() -> &'static str {
    "Foo _____ bar!"
}

#[get("/<_..>")]
pub fn everything() -> &'static str {
    "Hey, you've reached the end of the line!"
}
