use mtiny_core::{Request, body::{BodyStream, BoxBody, BodyExt}};

pub fn stream(request: &mut Request)->BodyStream<BoxBody>{
    std::mem::take(request.body_mut()).stream()
}