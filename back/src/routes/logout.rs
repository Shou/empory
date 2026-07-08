
use http::{StatusCode};


pub async fn logout() -> Result<http::HeaderMap, (StatusCode, &'static str)> {
    let mut hm = http::HeaderMap::new();
    let cookie = format!("Authorization=deleted; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=0")
        .parse()
        .map_err(|err| {
            println!("{:?}", err);
            (StatusCode::UNAUTHORIZED, "rabbit gagged asf")
        })?;
    hm.insert(
        axum::http::header::SET_COOKIE,
        cookie,
    );
    Ok(hm)
}
