use reqwest::{RequestBuilder, Response};

pub fn send_request(request: RequestBuilder) -> Response {
    match request.send() {
        Ok(response) => response,
        Err(err) => {
            // this is not recoverable
            println!("Unrecoverable: Cannot make network request, {:?}", err);
            std::process::exit(2);
        }
    }
}
