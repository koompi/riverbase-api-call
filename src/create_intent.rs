use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateIntentPayload {
    pub amount: String,
    pub currency: String,
    pub orderId: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateIntentResponse {
    pub _id: String,
    pub amount: String,
    pub currency: String,
    pub org_id: String,
    pub target: String,
    pub order_id: String,
}

pub async fn create_intent(
    shop_id: String,
    payload: CreateIntentPayload,
    debug: bool,
) -> Result<CreateIntentResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    let body = serde_json::to_value(&payload).unwrap();
    let url = format!("http://localhost:9999/merchants/{}/create-intent", shop_id);
    let request = client
        .request(reqwest::Method::POST, url)
        .headers(headers)
        .json(&body);

    let response = request.send().await?;
    let body = response.text().await?;
    if debug {
        println!("{}", body);
    }

    let json_body = serde_json::from_str::<CreateIntentResponse>(&body).unwrap();

    Ok(json_body)
}
