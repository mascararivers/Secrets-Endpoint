use std::sync::LazyLock;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use regex::Regex;
use serde::Deserialize;


#[derive(Deserialize)]
struct SecretQuery {
    uuid: String,
    api_key: String,
}


static SECRET_CACHE: LazyLock<Cache<String, u32>> = LazyLock::new(|| { // initialized only when needed
    Cache::builder()
        .time_to_live(Duration::from_secs(45))
        .build()
});

#[get("/secrets")]
async fn get_secrets(query: web::Query<SecretQuery>) -> impl Responder {
    let uuid = query.uuid.clone();

    // self-explanatory
    if let Some(cached) = SECRET_CACHE.get(&uuid) {
        return HttpResponse::Ok().body(format!("{} (cached)", cached));
    }

    // or else do the web request
    let client = reqwest::Client::new();
    let request = client
        .get(format!(
            "https://api.hypixel.net/v2/skyblock/profiles?uuid={}",
            uuid
        ))
        .header("API-Key", query.api_key.clone())
        .send()
        .await;

    if let Ok(response) = request {
        if let Ok(text_response) = response.text().await {
            let re = Regex::new(r#""secrets":(\d+)"#).unwrap();
            let mut secrets_found = 0;

            if let Some(caps) = re.captures(&text_response) {
                if let Some(matched) = caps.get(1) {
                    if let Ok(number) = matched.as_str().parse::<u32>() {
                        secrets_found = number;

                        // store the value in the cache
                        SECRET_CACHE.insert(uuid, secrets_found).await;

                        return HttpResponse::Ok().body(format!("{}", secrets_found));
                    }
                }
            }
        }
    }

    HttpResponse::BadRequest().body("Not Found")
}

// This has to be the worst fucking API i've ever had to work with - Every person that's ever scraped the Hypixel API

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_secrets)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
