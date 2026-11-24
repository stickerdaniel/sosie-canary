pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn farewell(name: &str) -> String {
    let mut out = String::new();
    out.push_str("Goodbye, ");
    out.push_str(name);
    out.push('!');
    out
}

pub fn shout(text: &str) -> String {
    text.to_uppercase()
}

pub fn whisper(text: &str) -> String {
    text.to_lowercase()
}
