use actix_web::{get, web::Path, HttpResponse};

use crate::response::Response;
use crate::models::PriceHistory;
use crate::prices::data_access::fetch_prices_by_id;

const CONTENT_TYPE: &str = "application/json";

#[get("/price_history/{id}")]
pub async fn get_price_history(path: Path<(String,)>) -> HttpResponse {
    let fetched_price_history = fetch_prices_by_id(&path.0); 

    match fetched_price_history {
        Ok(price_history) => HttpResponse::Ok()
            .content_type(CONTENT_TYPE) 
            .json(Response { data: Some(price_history), error: None }),
        Err(err) => HttpResponse::InternalServerError() 
            .content_type(CONTENT_TYPE) 
            .json(Response::<PriceHistory>::error(&format!("Error fetching prices: {}", err))),
    }
}