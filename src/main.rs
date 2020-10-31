mod totp;
mod config;

use totp::TOTP;

fn main() {
    config::get_config();

    let secret = "1234567890abc123";

    let t = TOTP::new(secret.as_bytes(), 10, 120);
    println!("{}", t.generate_token());
}
