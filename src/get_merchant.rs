use super::create_merchant::MerchantInfo;

pub async fn get_merchant(shop_id: String) -> Result<MerchantInfo, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;
    let url = format!("http://localhost:9999/merchants/{}", shop_id);

    let request = client.request(reqwest::Method::GET, url);

    let response = request.send().await?;
    let body = response.text().await?;
    let json_body = serde_json::from_str::<MerchantInfo>(&body).unwrap();

    Ok(json_body)
}
