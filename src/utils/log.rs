use colorful::Color;
use colorful::Colorful;

pub fn error(input: String) {
    println!("{}", input.color(Color::Red));
}

pub fn info(input: String) {
    println!("{}", input.color(Color::DarkGray));
}

pub fn yellow(input: &str) -> String {
    input.color(Color::Yellow).bold().to_string()
}

pub fn cyan(input: &str) -> String {
    input.color(Color::Cyan).bold().to_string()
}

pub fn blue(input: &str) -> String {
    input.color(Color::Blue).bold().to_string()
}

pub fn red(input: &str) -> String {
    input.color(Color::Red).bold().to_string()
}

pub fn bold(input: &str) -> String {
    input.bold().to_string()
}
