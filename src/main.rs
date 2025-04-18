use anyhow::Result;

#[derive(Debug)]
pub struct Request {
    url: String,
    method: Method,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

// Request Builder
#[derive(Default)]
struct RequestBuilder<U, M, B> {
    url: U,
    method: M,
    headers: Vec<(String, String)>,
    body: B,
}

/// STATES
// URL States
#[derive(Default, Clone)]
pub struct MissingUrl;
#[derive(Default, Clone)]
pub struct Url(String);

// Method States
#[derive(Default, Clone)]
pub struct MissingMethod;
#[derive(Default, Debug, Clone)]
pub enum Method {
    #[default]
    GET,
    POST,
}

// Body States
#[derive(Default, Clone)]
pub struct MissingBody;
#[derive(Default, Clone)]
pub struct NoBody;
#[derive(Default, Clone)]
pub struct Body(Option<String>);

// Default state is always going to start off without a Url, Method, or Body
impl RequestBuilder<MissingUrl, MissingMethod, MissingBody> {
    pub fn new() -> Self {
        RequestBuilder::default()
    }
}

// Basic functions for building request
impl<U, M, B> RequestBuilder<U, M, B> {
    /// Returns a RequestBuilder with a URL
    pub fn url(self, url: impl Into<String>) -> RequestBuilder<Url, M, B> {
        RequestBuilder {
            url: Url(url.into()),
            method: self.method,
            headers: self.headers,
            body: self.body,
        }
    }

    /// Adds a header to a request
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((key.into(), value.into()));
        self
    }
}

impl<U, B> RequestBuilder<U, MissingMethod, B> {
    /// GET request will never have a body, so return a RequestBuilder with a Method and NoBody type
    pub fn get(self) -> RequestBuilder<U, Method, NoBody> {
        RequestBuilder {
            url: self.url,
            method: Method::GET,
            headers: self.headers,
            body: NoBody,
        }
    }
}

impl<U, B> RequestBuilder<U, MissingMethod, B> {
    /// POST requests may or may not have a body, so return a RequestBuilder with a Method, but any Body
    pub fn post(self) -> RequestBuilder<U, Method, B> {
        RequestBuilder {
            url: self.url,
            method: Method::POST,
            headers: self.headers,
            body: self.body,
        }
    }
}

impl<U, M> RequestBuilder<U, M, MissingBody> {
    /// Return a RequestBuilder with a Body
    pub fn body(self, body: impl Into<String>) -> RequestBuilder<U, M, Body> {
        RequestBuilder {
            url: self.url,
            method: self.method,
            headers: self.headers,
            body: Body(Some(body.into())),
        }
    }
}

/// There are three states during build
///   1. NoBody (GET)
///   2. Body (POST)
///   3. MissingBody (POST)
impl RequestBuilder<Url, Method, Body> {
    pub fn build(self) -> Request {
        Request {
            url: self.url.0,
            method: self.method,
            headers: self.headers,
            body: self.body.0,
        }
    }
}
impl RequestBuilder<Url, Method, NoBody> {
    pub fn build(self) -> Request {
        Request {
            url: self.url.0,
            method: self.method,
            headers: self.headers,
            body: None,
        }
    }
}
impl RequestBuilder<Url, Method, MissingBody> {
    pub fn build(self) -> Request {
        Request {
            url: self.url.0,
            method: self.method,
            headers: self.headers,
            body: None,
        }
    }
}

fn main() -> Result<()> {
    // When building a GET, `body()` cannot be called, and a RequestBuilder with NoBody is returned
    let req = RequestBuilder::new()
    .get()
    .url("https://www.google.com")
    .header("Token", "zxcvasdv")
    .header("user-agent", "chrome/4.20.69")
    // .body("asdf") // throws a compiler error since `.get()` returns a RequestBuilder with a NoBody
    .build();

    println!("** GET Request **");
    println!("{:?} {} {:?} {:?} \n", req.method, req.url, req.headers, req.body);

    // When building a POST and calling `body()` a RequestBuilder with Body is returned
    let req = RequestBuilder::new()
        .url("https://www.google.com")
        .post()
        .header("Token", "zxcvasdv")
        .header("user-agent", "chrome/4.20.69")
        .body("asdf")
        .build();

    println!("** POST Request **");
    println!("{:?} {} {:?} {:?} \n", req.method, req.url, req.headers, req.body);

    // When building a POST without calling `body()` a RequestBuilder with MissingBody is returned
    let req = RequestBuilder::new()
        .url("https://www.google.com")
        .post()
        .header("Token", "zxcvasdv")
        .header("user-agent", "chrome/4.20.69")
        // .body("asdf") // Not setting a body, means a RequestBuilder with MissingBody is returned
        .build();

    println!("** POST Request with missing body **");
    println!("{:?} {} {:?} {:?} \n", req.method, req.url, req.headers, req.body);

    Ok(())
}
