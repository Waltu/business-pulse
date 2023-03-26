mod model;

use worker::*;
use reqwest;
use model::ApiResponse;

mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
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

            let business = match kv.get(business_id).text().await? {
                Some(business) => business,
                None => {
                    let client = reqwest::Client::new();

                    let response = client
                        .get(format!("{}{}", "https://avoindata.prh.fi/bis/v1/", business_id))
                        .send()
                        .await;

                    let business = match response {
                        Ok(res) => res.text().await.unwrap(),
                        Err(_) => "No company found with business id".to_string(),
                    };

                    kv.put(business_id, business.clone())?.execute().await?;

                    business
                }
            };

            let response = Response::ok(business)?;
            let mut headers = Headers::new();

            headers.set("Content-Type", "application/json")?;

            Ok(response.with_headers(headers))
        })
        .run(req, env)
        .await
}
