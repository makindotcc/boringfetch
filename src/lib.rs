#![feature(once_cell)]

use boringhyper::{ChromeHeadersExt, ReadBodyExt};
use hyper::{Body, Request};
use neon::prelude::*;
use std::sync::LazyLock;
use tokio::runtime::Runtime;

static TOKIO: LazyLock<Runtime> =
    LazyLock::new(|| Runtime::new().expect("Could not initialize tokio runtime!"));
static CLIENT: LazyLock<boringhyper::Client> = LazyLock::new(boringhyper::create_client);

#[derive(Debug)]
enum ReadError {
    Response(boringhyper::Error),
    Read(hyper::Error),
}

fn fetch(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let url = cx.argument::<JsString>(0)?.value(&mut cx);
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    TOKIO.spawn(async move {
        let req = Request::builder()
            .uri(url)
            .with_chrome_headers()
            .body(Body::empty())
            .unwrap();
        let response_result = CLIENT.request(req).await;
        let result = match response_result {
            Ok(mut response) => response
                .read_body()
                .await
                .map(|body| (response.status(), body))
                .map_err(ReadError::Response),
            Err(err) => Err(ReadError::Read(err)),
        };
        deferred.settle_with(&channel, move |mut cx| {
            let (status, body) =
                result.or_else(|err| cx.throw_error(format!("Could not do request: {:?}", err)))?;
            let js_response = cx.empty_object();
            let js_status = cx.number(status.as_u16());
            let js_body = cx.string(String::from_utf8_lossy(&body));
            js_response.set(&mut cx, "status", js_status)?;
            js_response.set(&mut cx, "body", js_body)?;
            Ok(js_response)
        });
    });
    Ok(promise)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("boringfetch", fetch)?;
    Ok(())
}
