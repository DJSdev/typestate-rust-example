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

// There are two states during build
//   1. With a Body
//   2. Without a Body
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

fn main() -> Result<()> {
    let req = RequestBuilder::new()
        .url("https://www.google.com")
        .post()
        .header("Token", "zxcvasdv")
        .header("user-agent", "chrome/4.20.69")
        .body("asdf")
        .build();

    println!("{req:?}");

    let req = RequestBuilder::new()
        .get()
        .url("https://www.google.com")
        .header("Token", "zxcvasdv")
        .header("user-agent", "chrome/4.20.69")
        // .body("asdf") // throws a compiler error since `.get()` returns a RequestBuilder with a NoBody
        .build();

    println!("{req:?}");

    Ok(())
}
