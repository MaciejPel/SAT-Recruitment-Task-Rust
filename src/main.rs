use actix_web::{App, HttpServer, web};

mod error_handler;
use error_handler::actix_handle_err;

mod api;
use api::dissel_usage::calculate_dissel_usage_for_distance;
use api::unit_injector::probability_of_unit_injector_fail;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new()
      .app_data(web::QueryConfig::default().error_handler(|err, _req| actix_handle_err(err)))
			.service(calculate_dissel_usage_for_distance)
			.service(probability_of_unit_injector_fail)
	})
	.bind(("127.0.0.1", 3001))?
	.run()
	.await
}
