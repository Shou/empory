
use base64::prelude::*;
use chrono::Utc;
use ed25519_dalek::{SigningKey, Signer};
use reqwest::Client;
use sha2::{Sha256, Digest};


#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub client: Client,
    pub key: SigningKey,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateJob {
    post_id: uuid::Uuid,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateJobError {
}


pub async fn create_job(
    config: &ClientConfig,
    base_url: String,
    &post_id: &uuid::Uuid
) -> anyhow::Result<()> {
    let path = "/job";
    let job = CreateJob { post_id };

    let timestamp = Utc::now().timestamp().to_string();
    let json_body = serde_json::to_vec(&job)?;
    let body_hash = Sha256::digest(json_body);

    let message = format!(
        "{}\n{}\n{}\n{}",
        "POST",
        path,
        timestamp,
        hex::encode(body_hash),
    );

    let signature_raw = config.key.sign(message.as_bytes());
    let b64sig = BASE64_STANDARD.encode(signature_raw.to_bytes());

    config.client
        .post(format!("{}{}", base_url, path))
        .json(&job)
        .header("X-Service-Id", "webapi")
        .header("X-Service-Timestamp", timestamp)
        .header("X-Service-Signature", b64sig)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}
