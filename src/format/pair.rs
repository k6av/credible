use owo_colors::OwoColorize;

pub fn format((user, pass): (String, String)) -> String {
    format!("
{}

Username: {}
Password: {}

",          "[ Username-password pair ]".green().bold(),
            user.yellow(), pass.blue())
}