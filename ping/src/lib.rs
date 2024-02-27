use async_trait::async_trait;
use wasmbus_rpc::actor::prelude::*;
use serde_json::json;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_examples_pong::{Pong, PongSender, Request};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct PingActor;

#[async_trait]
impl HttpServer for PingActor {
    async fn handle_request(&self, _ctx: &Context, _value: &HttpRequest) -> RpcResult<HttpResponse> {
        let provider = PongSender::new();
        let request = Request{param: Some("ping".to_string())};
        let response = provider.echo(_ctx, &request).await?;
        let body = json!({
            "body": response.result.unwrap_or("".to_string()),
        });

        let response = HttpResponse {
            status_code: 200,
            body: serde_json::to_vec(&body)
                .map_err(|e| RpcError::ActorHandler(format!("serializing response: {}", e)))?,
            ..Default::default()
        };
        Ok(response)
    }
}
