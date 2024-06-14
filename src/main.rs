use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};
use serde::Deserialize;
use serde_json::from_str;
use std::str::from_utf8;

#[derive(Deserialize)]
struct JsonRequestBody {
    message: String,
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // リクエストから情報を抽出
    let json = from_utf8(event.body()).expect("illegal body");
    // トレースログを出力
    tracing::info!(payload = %json, "JSON Payload received");

    // リクエストのボディをパース
    let req = from_str::<JsonRequestBody>(json).expect("parse error");

    // レスポンスを返す
    let resp = Response::builder()
        .status(200)
        // .header("content-type", "application/json; charset=utf-8")
        .header("content-type", "text/plain; charset=utf-8")
        .body(req.message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // ロガーを初期化
    tracing::init_default_subscriber();

    // ラムダハンドラを起動
    run(service_fn(function_handler)).await?;
    Ok(())
}
