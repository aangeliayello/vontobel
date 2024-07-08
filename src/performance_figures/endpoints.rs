use actix_web::{get, web::{Path, Query}, HttpResponse};
use chrono::{NaiveDate, Local}; 
use serde::Deserialize;

use crate::models::{DailyReturnHistory, YtdReturn};
use crate::performance_figures::returns::{calculate_daily_returns, calculate_ytd_return};
use crate::response::Response;

#[derive(Deserialize)]
pub struct AsOfDateQuery {
    as_of_date: Option<String>,
}

#[get("/daily_returns/{id}")]
pub async fn get_daily_returns(id: Path<String>) -> HttpResponse {
    match calculate_daily_returns(&id.into_inner()).await {
        Ok(daily_returns) => HttpResponse::Ok().json(Response { data: Some(daily_returns), error: None }),
        Err(err) => HttpResponse::InternalServerError().json(Response::<DailyReturnHistory>::error(&err.to_string())), 
    }
}

#[get("/ytd_return/{id}")]
pub async fn get_ytd_return(path: Path<String>, query: Query<AsOfDateQuery>) -> HttpResponse {
    let id = path.into_inner();
    let q = query.into_inner();
    let as_of_date = match q.as_of_date {
        Some(date_str) => match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
            Ok(date) => date,
            Err(err) => {
                let error_msg = format!("Invalid date format: {}", err);
                return HttpResponse::BadRequest().json(Response::<YtdReturn>::error(&error_msg));
            }
        },
        None => Local::now().naive_local().date(),
    };

    match calculate_ytd_return(&id, as_of_date).await {
        Ok(ytd_return) => HttpResponse::Ok().json(Response { data: Some(ytd_return), error: None }),
        Err(err) => HttpResponse::InternalServerError().json(Response::<YtdReturn>::error(&err.to_string())),
    }
}