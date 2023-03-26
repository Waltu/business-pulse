mod model;
mod utils;

use worker::*;
use reqwest;
use model::ApiResponse;


fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}


async fn get_business_by_business_id_from_kv_or_fetch(business_id: &str, kv: &kv::KvStore) -> Result<String> {
    let api_response = match kv.get(business_id).text().await? {
        Some(cache_value) => cache_value,
        None => {
            // Does it make more sense to get reqwest client as a parameter?
            let client = reqwest::Client::new();

            let response = client
                .get(format!("{}{}", "https://avoindata.prh.fi/bis/v1/", business_id))
                .send()
                .await;
            
            // How to handle errors here?
            let api_response = match response {
                Ok(res) => res.text().await.unwrap(),
                Err(_) => "No company found with business id".to_string(),
            };

            const ONE_DAY_SECONDS: u64 = 86400;

            kv.put(business_id, api_response.clone())?.expiration_ttl(ONE_DAY_SECONDS).execute().await?;
            
            api_response
        }
    };

    Ok(api_response)
}

fn map_body_to_response(body: String) -> Result<Response> {
    let response = Response::ok(body)?;
    let mut headers = Headers::new();

    headers.set("Content-Type", "application/json")?;

    Ok(response.with_headers(headers))
}


#[event(fetch, respond_with_errors)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);
    utils::set_panic_hook();

    let router = Router::new();

    router
        .get("/", |_, _| Response::ok("Hello business pulse user!"))
        .get_async("/business/:business_id", |_, ctx| async move {
            let business_id = match ctx.param("business_id") {
                Some(business_id) => business_id,
                None => return Response::ok("No business id provided"),
            };

            let kv = ctx.kv("business_pulse")?;

            let business_api_response = get_business_by_business_id_from_kv_or_fetch(business_id, &kv).await?;

            let api_response: ApiResponse = serde_json::from_str(&business_api_response)?;

            let results = serde_json::to_string(&api_response.results)?;

            map_body_to_response(results)
        })
        .run(req, env)
        .await
}
