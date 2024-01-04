use mtiny_core::Request;

#[derive(Debug)]
pub struct NotFound {
    request: Request,
}

impl NotFound {
    pub fn new(request: Request) -> Self {
        Self { request }
    }

    pub fn request_ref(&self) -> &Request {
        &self.request
    }

    pub fn request_mut(&mut self) -> &mut Request {
        &mut self.request
    }

    pub fn into_request(self) -> Request {
        self.request
    }
}

impl std::error::Error for NotFound {
    
}
impl std::fmt::Display for NotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("router not found")
    }
}


#[derive(Debug)]
pub struct MethodNotAllowed {
    request: Request,
}

impl MethodNotAllowed {
    pub fn new(request: Request) -> Self {
        Self { request }
    }

    pub fn request_ref(&self) -> &Request {
        &self.request
    }

    pub fn request_mut(&mut self) -> &mut Request {
        &mut self.request
    }

    pub fn into_request(self) -> Request {
        self.request
    }
}

impl std::error::Error for MethodNotAllowed {
    
}
impl std::fmt::Display for MethodNotAllowed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("router not found")
    }
}
