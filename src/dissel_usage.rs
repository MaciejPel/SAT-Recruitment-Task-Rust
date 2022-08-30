use actix_web::{get, Responder, Result, web};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct DisselUsageParams {
	distance: u64,
	yearOfProduction: u64,
	fuelUsagePer100KM: u64,
}

#[derive(Serialize)]
struct DisselUsageResponse {
	fuelUsage: f64,
}

#[get("/calculateDisselUsageForDistance")]
pub async fn calculate_dissel_usage_for_distance(info: web::Query<DisselUsageParams>) -> Result<impl Responder> {
  let (distance, fuelUsagePer100KM): (f64, f64) = (info.distance as f64, info.fuelUsagePer100KM as f64);
	let fuelUsage: f64 = fuelUsagePer100KM * (distance / 100.0);
	
	Ok(web::Json(DisselUsageResponse{fuelUsage}))
}