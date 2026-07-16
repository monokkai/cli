use base64::{Engine, engine::general_purpose::STANDARD};
use colored::Colorize;
use ring::rand::{SecureRandom, SystemRandom};

pub fn generate() {
    let rng = SystemRandom::new();
    let mut secret = [0u8; 64];

    if let Err(e) = rng.fill(&mut secret) {
        eprintln!("{} Failed to generate secret: {}", "❌".red(), e);
        return;
    }

    let encoded = STANDARD.encode(secret);
    println!("{}", "Generated JWT secret:".blue());
    println!("JWT_SECRET={}", encoded.green());
}

pub fn decode(input: &str) {
    match STANDARD.decode(input) {
        Ok(bytes) => match std::str::from_utf8(&bytes) {
            Ok(text) => {
                println!("{}", "Decoded:".blue());
                println!("{}", text.green());
            }
            Err(_) => {
                println!("{}", "Decoded (hex, not valid UTF-8):".yellow());
                println!("{}", hex(&bytes).green());
            }
        },
        Err(e) => eprintln!("{} Invalid base64: {}", "❌".red(), e),
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join("")
}
