use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MerchantApiKey {
    pub apiPublicKey: String,
}
pub async fn get_merchant_api_key(
    shop_id: String,
) -> Result<MerchantApiKey, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;
    let url = format!("http://localhost:9999/merchants/{}/apiKey", shop_id);

    let request = client.request(reqwest::Method::GET, url);

    let response = request.send().await?;
    let body = response.text().await?;
    let json_body = serde_json::from_str::<MerchantApiKey>(&body).unwrap();

    Ok(json_body)
}
