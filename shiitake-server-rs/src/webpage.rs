
use axum::{
    http::{
        header::{CONTENT_TYPE, CONTENT_ENCODING},
        HeaderValue,
    },
    body::{Bytes, Full, boxed},
    response::{IntoResponse, Response}
};

const WEBPAGE_BIN: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/index.html.gz"));

pub struct Webpage;

impl IntoResponse for Webpage {
    fn into_response(self) -> Response {
        let boxed_full = boxed(Full::new(Bytes::from_static(WEBPAGE_BIN)));

        Response::builder()
            .status(200)
            .header(CONTENT_TYPE, HeaderValue::from_static("text/html"))
            .header(CONTENT_ENCODING, HeaderValue::from_static("gzip"))
            .body(boxed_full)
            .unwrap()
    }
}