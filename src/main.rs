use actix_web::{App, HttpServer};
mod dissel_usage;
mod unit_injector;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new()
			.service(dissel_usage::calculate_dissel_usage_for_distance)
			.service(unit_injector::probability_of_unit_injector_fail)
	})
	.bind(("127.0.0.1", 3001))?
	.run()
	.await
}
