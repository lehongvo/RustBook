#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
}

fn get_user_email(user: Option<User>) -> Result<String, String> {
    let Some(user) = user else {
        return Err("User not found".to_string());
    };

    Ok(user.email)
}

fn make_exciting(s: &str) -> String {
    s.replace(".", "!").replace("?", "‽")
}
