use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct TestActor {}

#[async_trait]
impl HttpServer for TestActor {
    async fn handle_request(&self, _ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        println!("Incoming request: {} {}", req.method(), req.uri());

        if req.method() == "POST" {
            let body_bytes = req.body().await.unwrap();
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
            println!("Request body: {}", body_str);
        }

        let response_body = "event: meta\ndata: {\"content_type\": \"text/markdown\", \"linkify\": true}\n\n\
                             event: text\ndata: {\"text\": \"I\"}\n\n\
                             event: text\ndata: {\"text\": \" am\"}\n\n\
                             event: text\ndata: {\"text\": \" the prem.\"}\n\n\
                             event: done\ndata: {}\n\n";

        Ok(HttpResponse::ok()
            .header("Content-Type", "text/event-stream")
            .body(response_body))
    }
}

impl TestActor {
    pub fn new() -> Self {
        Self::default()
            .with_http_server(|_| HttpServerReceiver::new(Self::handle_request))
    }
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:6969".parse().unwrap();
    let server = HttpServer::new(TestActor::new(), addr);
    server.start().await;
}
