use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CallbackPayload {
    pub encrypted_order_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CallbackResponse {
    pub orderId: String,
}

pub async fn create_intent(
    shop_id: String,
    payload: CallbackPayload,
    debug: bool,
) -> Result<CallbackResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    let body = serde_json::to_value(&payload).unwrap();
    let url = format!("http://localhost:9999/merchants/{}/callback", shop_id);
    let request = client
        .request(reqwest::Method::POST, url)
        .headers(headers)
        .json(&body);

    let response = request.send().await?;
    let body = response.text().await?;
    if debug {
        println!("{}", body);
    }

    let json_body = serde_json::from_str::<CallbackResponse>(&body).unwrap();

    Ok(json_body)
}
