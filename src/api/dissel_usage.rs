use actix_web::{get, Responder, Result, web};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct DisselUsageParams {
	distance: u64,
  #[serde(rename = "yearOfProduction")]
	year_of_production: u64,
  #[serde(rename = "fuelUsagePer100KM")]
	fuel_usage_per100_km: u64,
}

#[derive(Serialize)]
struct DisselUsageResponse {
  #[serde(rename = "fuelUsage")]
	fuel_usage: f64,
}

#[get("/calculateDisselUsageForDistance")]
pub async fn calculate_dissel_usage_for_distance(info: web::Query<DisselUsageParams>) -> Result<impl Responder> {
  let (distance, fuel_usage_per100_km): (f64, f64) = (info.distance as f64, info.fuel_usage_per100_km as f64);
	let fuel_usage: f64 = fuel_usage_per100_km * (distance / 100.);
	
	Ok(web::Json(DisselUsageResponse{fuel_usage}))
}