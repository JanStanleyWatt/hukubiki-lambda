// Copyright (C) 2024 Jan Stanley Watt

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

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
    let json = match from_utf8(event.body()) {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("Error: {}", e);
            return Ok(Response::builder()
                .status(400)
                .body("Error".into())
                .map_err(Box::new)?);
        }
    };

    // トレースログを出力
    tracing::info!(payload = %json, "JSON Payload received");

    // リクエストのボディをパース
    let req = match from_str::<JsonRequestBody>(json) {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("Error: {}", e);
            return Ok(Response::builder()
                .status(400)
                .body("Error".into())
                .map_err(Box::new)?);
        }
    };

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
