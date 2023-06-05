# Nexus

Welcome to the Nexus! This README will guide you through setting up your development environment and getting started with the project.

## Prerequisites

Before you begin, make sure you have the following installed on your system:

- [Just](https://just.systems/)
- [Git](https://git-scm.com/)

## Getting Started with Cosmonic

1. Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/plurigrid/nexus.git
cd nexus
```

2. Depending if you are going to be building your own apps or just running them you can run either `play` or `build` respectivly:

```bash
just play
```

- or -

```bash
just build
```

Play will automatically check for and install OpenSSL 1.1, Nix, and Cosmonic if they are not already installed on your system. Build will install all that plus Rust and the wasm32-unknown-unknown target to compile to.

3. Once the dependenices have been installed you can start a nix shell, this will drop you into an environment with all the rest of the dependencies installed:

```bash
nix-shell
```

4. You can now run your Actors by navigating to thier directory and launching them with Cosmonic:

```bash
cd actors/<actor_name>
cosmo launch
```

Replace `<actor_name>` with the name of the acotor direcory you wish to launch from.

## Building with Cosmonic

1. Create a new actor using Cosmo CLI:

```bash
cosmo new actor <your_project_name>
```

Replace `<your_project_name>` with your desired project name.

2. Navigate to your newly created project directory:

```bash
cd <your_project_name>
```

3. Edit `src/lib.rs` file in your favorite text editor.

The default file content looks like this:

```rust
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct <your_project_name>Actor {}

/// Implementation of the HttpServer capability contract
#[async_trait]
impl HttpServer for <your_project_name>Actor {
    async fn handle_request(&self, _ctx: &Context, _req: &HttpRequest) -> RpcResult<HttpResponse> {
        Ok(HttpResponse::ok("message"))
    }
}
```

You can modify the file to accommodate more text like this:

```rust
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct <your_project_name>Actor {}

/// Implementation of the HTTP server capability
#[async_trait]
impl HttpServer for <your_project_name>Actor {
    async fn handle_request(&self, _ctx: &Context, _req: &HttpRequest) -> RpcResult<HttpResponse> {
        let message: &str = r#"message"#;

        Ok(HttpResponse::ok(message))
    }
}

```

4. Login to Cosmonic:

```bash
cosmo login
```

5. Build and sign your actor:

```bash
cosmo build
```

6. Start your wasmCloud host:

```bash
cosmo up
```

7. Launch the actor using Cosmo CLI:

```bash
cosmo launch
```

8. Navigate to [Cosmonic App](https://app.cosmonic.com) and sign in with your account.

9. In the Logic view, you should see the new actor you just launched.

10. To make your actor accessible from the web, launch a new provider for an HTTP server with the following OCI URL: `cosmonic.azurecr.io/httpserver_wormhole:0.5.3`. Give the link a name, and note that the HTTP server must be launched on a Cosmonic Manager resource.

11. Once the HTTP server is launched, link it to your actor.

12. Launch a wormhole and connect it to your actor link (the HTTP server and the actor).

13. Your actor should now be accessible at the domain of the wormhole followed by `.cosmonic.app`. For example: `https://white-morning-5041.cosmonic.app`.

Now you can access your project from any web browser using the provided URL!

You're all set! You can start building your project and explore the Nexus repository. Happy coding!

# NATS Server and Client Setup

To send peer to peer pub-sub messages NATS is an excellent tool. This will walk you through setting up a NATS server and client to publish and subscribe to messages and expose your server using ngrok.

## Server Setup

Start the NATS server by running the following command:

   ```
   nats-server
   ```

## Exposing NATS Server with ngrok

1. Sign up for an account and download ngrok from the [official website](https://ngrok.com/download).

2. Follow the [ngrok getting started guide](https://ngrok.com/docs/getting-started/) to set up ngrok on your system.

3. Go to the [ngrok dashboard](https://dashboard.ngrok.com/get-started/your-authtoken) to get your auth token.

4. Set your auth token with the following command:

   ```
   ngrok config add-authtoken <auth_token>
   ```

5. Connect ngrok to your NATS server over TCP with the following command:

   ```
   ngrok tcp 4222
   ```

6. ngrok will provide you with a URL that looks like this: `tcp://8.tcp.ngrok.io:13164`. You can use this URL to connect to your NATS server with the NATS client.

## Subscribing and Publishing with NATS CLI

1. To subscribe to messages, run the following command:

   ```
   nats sub -s <ngrok_tcp_url>
   ```

2. To publish a message, run the following command:

   ```
   nats pub <subject> <body>
   ```

That's it! You have now set up a NATS server and client, and you can publish and subscribe to messages using the NATS CLI.
