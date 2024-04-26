use owo_colors::OwoColorize;

pub fn format((pass): (String)) -> String {
    format!("
{}

Password: {}

",          "[ Password ]".green().bold(),
            pass.blue())
}