pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

pub fn farewell(name: &str) -> String {
    let mut out = String::with_capacity(name.len() + 10);
    out.push_str("Goodbye, ");
    out.push_str(name);
    out.push_str("!");
    out
}

pub fn shout(text: &str) -> String {
    text.to_uppercase()
}

pub fn excited(text: &str) -> String {
    let mut s = text.to_string();
      s.push_str("!!");
    s
}

pub fn whisper(text: &str) -> String {
    text.to_lowercase()
}
