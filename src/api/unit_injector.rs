use actix_web::{get, Responder, Result, web};
use serde::{Deserialize, Serialize};
use rand::Rng;

fn random_number_generator() -> f64 {
  let mut rng = rand::thread_rng();
  return rng.gen_range(0.0..1.0);
}

#[derive(Deserialize)]
pub struct UnitInjectorParams {
  #[serde(rename = "VIN")]
	vin: String,
}

#[derive(Serialize)]
struct UnitInjectorResponse {
  #[serde(rename = "failProbability")]
	fail_probability: String,
}

#[get("/probabilityOfUnitInjectorFail")]
pub async fn probability_of_unit_injector_fail(_: web::Query<UnitInjectorParams>) -> Result<impl Responder> {
  // 'round' to 2 decimal places
  let random_number: f64 = (random_number_generator() * 100.0).round() / 100.0;

	Ok(web::Json(UnitInjectorResponse{fail_probability: random_number.to_string()}))
}