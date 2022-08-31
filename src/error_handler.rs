use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize, Debug)]
struct ErrorResponse {
	message: String,
}

pub fn actix_handle_err<T: std::error::Error + 'static>(err: T) -> actix_web::error::Error {
  let original_message = err.to_string().replace("Query deserialize error: ", "");
	let response = HttpResponse::BadRequest().json(ErrorResponse {
		message: original_message[0..1].to_uppercase() + &original_message[1..]
	});

	actix_web::error::InternalError::from_response(err, response).into()
}