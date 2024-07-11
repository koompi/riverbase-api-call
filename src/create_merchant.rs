use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateMerchantPayload {
    pub storeName: String,
    pub storeDomain: String,
    pub callbackUrl: String,
    pub storeId: String,
    pub userEmail: String,
    pub userPassword: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MerchantInfo {
    pub storeId: String,
    pub userEmail: String,
    pub userPassword: String,
    pub merchantOrgId: String,
    pub apiPublicKey: String,
    pub apiSecretKey: String,
    pub apiSecretIv: String,
    pub webhookSecretKey: String,
    pub webhookSecretIv: String,
    pub _id: String,
}

pub async fn create_merchant(
    payload: CreateMerchantPayload,
    debug: bool,
) -> Result<MerchantInfo, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    let body = serde_json::to_value(&payload).unwrap();

    let request = client
        .request(
            reqwest::Method::POST,
            "http://localhost:9999/merchants/create",
        )
        .headers(headers)
        .json(&body);

    let response = request.send().await?;
    let body = response.text().await?;

    if debug {
        println!("{}", body);
    }

    let json_body = serde_json::from_str::<MerchantInfo>(&body).unwrap();

    Ok(json_body)
}
