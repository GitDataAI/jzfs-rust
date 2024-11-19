use actix_web::{body::MessageBody, dev::ServiceResponse};
use actix_web::{http::header, Result};

pub async fn cros(mut res: ServiceResponse<impl MessageBody>) -> Result<ServiceResponse<impl MessageBody>> {
    let headers = res.headers_mut();

    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, header::HeaderValue::from_static("*"));
    headers.insert(header::ACCESS_CONTROL_ALLOW_METHODS, header::HeaderValue::from_static("*"));
    headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, header::HeaderValue::from_static("*"));
    Ok(res)
}