use regex::Regex;

pub fn validate_email(email: &String) -> bool {
    let pattern = Regex::new(r"^[^ ]+@[^ ]+\.[a-z]{2,3}$").unwrap();
    pattern.is_match(email)
}

pub fn validate_password(password: &String) -> bool {
    if password.len() > 7 {
        true
    } else {
        false
    }
}