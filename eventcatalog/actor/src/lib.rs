use std::collections::HashMap;

use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};

mod ui;
use ui::Asset;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct BankAccountCatalog {}

#[async_trait]
impl HttpServer for BankAccountCatalog {
    async fn handle_request(&self, _ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        Ok(match req.path.trim_start_matches('/') {
            path => {
                // Request for UI asset
                Asset::get(path)
                    .map(|asset| response(Vec::from(asset.data), path))
                    // Simple fallback to grab index.html pages when the path is the root of a page or subpage
                    .or_else(|| {
                        Asset::get(
                            &format!("{}/index.html", path.trim_end_matches('/').to_owned())
                                .trim_start_matches('/'),
                        )
                        .map(|asset| response(Vec::from(asset.data), path))
                    })
                    .unwrap_or_else(|| HttpResponse::not_found())
            }
        })
    }
}

fn response(body: Vec<u8>, path: &str) -> HttpResponse {
    let mut header = HashMap::new();
    if let Some(content_type) = mime_guess::from_path(path).first() {
        header.insert("Content-Type".to_string(), vec![content_type.to_string()]);
    }
    HttpResponse {
        status_code: 200,
        header,
        body,
    }
}
