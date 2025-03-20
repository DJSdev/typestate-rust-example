# Summary

This is a basic example of using a TypeState Builder pattern in Rust

## Details

This video was the starting point: https://www.youtube.com/watch?v=pwmIQzLuYl0

In addition to the video, I removed the `method()` function.

In place I added individual `get()` and `post()` functions. 

When `get()` is called, a `RequestBuilder` with a `NoBody` type is returned, this restricts a call to `body()` so the dev can't set a `body` on a GET request.

When `post()` is called, a `RequestBuilder` with a `Body` type is returned, which allows the developer to set the body using `body()`.