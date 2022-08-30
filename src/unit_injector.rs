use actix_web::{get, Responder, Result, web};
use serde::{Deserialize, Serialize};
use rand::Rng;

fn random_number_generator() -> f64 {
  let mut rng = rand::thread_rng();
  return rng.gen_range(0.0..1.0);
}

#[derive(Deserialize)]
pub struct UnitInjectorParams {
	VIN: String,
}

#[derive(Serialize)]
struct UnitInjectorResponse {
	failProbability: String,
}

#[get("/probabilityOfUnitInjectorFail")]
pub async fn probability_of_unit_injector_fail(info: web::Query<UnitInjectorParams>) -> Result<impl Responder> {
  // 'round to 2 decimal'
  let number: f64 = (random_number_generator() * 100.0).round() / 100.0;

	Ok(web::Json(UnitInjectorResponse{failProbability: number.to_string()}))
}