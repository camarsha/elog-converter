#[allow(dead_code)]
pub struct ElogDate {
    year: i32,
    month: i32,
    day: i32,
}

#[allow(dead_code)]
impl ElogDate {
    pub fn new(year: i32, month: i32, day: i32) -> Self {
        ElogDate { year, month, day }
    }

    pub fn to_elog_format(&self) -> String {
        // Right now I am assuming you do not have enough log entries in a day to
        // get past a
        format!("{}{:02}{:02}", self.year % 100, self.month, self.day)
    }
}

/// https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust
pub fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
