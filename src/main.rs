#![allow(non_snake_case)]
pub mod callback;
pub mod create_intent;
pub mod create_merchant;
pub mod get_merchant;
pub mod get_merchant_api_key;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
