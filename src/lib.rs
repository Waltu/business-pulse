mod model;
mod utils;

use worker::*;
use reqwest;
use model::{ApiResponse, Business};


fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

async fn fetch_business_by_id(business_id: &str) -> Result<String> {
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

    return Ok(api_response);
}

fn map_api_response_to_business(api_response: String) -> Business {
    let api_response: ApiResponse = serde_json::from_str(&api_response).unwrap();
    let business = api_response.results[0].clone();

    return business;
}


async fn get_business_by_business_id_from_kv_or_fetch(business_id: &str, kv: &kv::KvStore) -> Result<Business> {
    let api_response = match kv.get(business_id).text().await? {
        Some(cache_value) => cache_value,
        None => {
            let api_response = fetch_business_by_id(business_id).await?;

            const ONE_DAY_SECONDS: u64 = 86400;
            kv.put(business_id, api_response.clone())?.expiration_ttl(ONE_DAY_SECONDS).execute().await?;
            
            api_response
        }
    };

    let business = map_api_response_to_business(api_response);

    Ok(business)
}

async fn get_business_by_business_id(business_id: &str) -> Result<Business> {
    let api_response = fetch_business_by_id(business_id).await?;
    let business = map_api_response_to_business(api_response);

    Ok(business)
}

fn map_body_to_response(body: String) -> Result<Response> {
    let mut headers = Headers::new();
    headers.set("Content-Type", "application/json")?;
    headers.set("Cache-Control", "s-maxage=60")?;

    let response = Response::ok(body)?;

    Ok(response.with_headers(headers))
}


#[event(fetch, respond_with_errors)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);
    utils::set_panic_hook();

    let router = Router::new();

    router
        .get("/", |_, _| Response::ok("Hello business pulse user!"))
        .get_async("/v1/business/:business_id", |_, ctx| async move {
            let business_id = match ctx.param("business_id") {
                Some(business_id) => business_id,
                None => return Response::ok("No business id provided"),
            };

            let kv = ctx.kv("business_pulse")?;

            let business = get_business_by_business_id_from_kv_or_fetch(business_id, &kv).await?;

            map_body_to_response(serde_json::to_string(&business)?)
        })
        .get_async("/v2/business/:business_id", |req, ctx| async move {
            let business_id = match ctx.param("business_id") {
                Some(business_id) => business_id,
                None => return Response::ok("No business id provided"),
            };

            let cache = Cache::default();
            let cache_key = req.url()?.to_string();

            if let Some(resp) = cache.get(&cache_key, true).await? {
                console_log!("Cache HIT!");
                Ok(resp)
            } else {
                console_log!("Cache MISS!");
                let business = get_business_by_business_id(business_id).await?;
                let mut response = map_body_to_response(serde_json::to_string(&business)?)?;

                cache.put(&cache_key, response.cloned()?).await?;
                
                Ok(response)
            }

        })
        .run(req, env)
        .await
}
