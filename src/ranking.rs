#[get("/<id>")]
pub fn user(id: usize) -> String {
    format!("Hello, {} rank1!", id)
}

#[get("/<id>", rank = 2)]
pub fn user_int(id: isize) -> String {
    format!("Hello, {} rank2!", id)
}

#[get("/<id>", rank = 3)]
pub fn user_str(id: String) -> String {
    format!("Hello, {} rank3!", id)
}
