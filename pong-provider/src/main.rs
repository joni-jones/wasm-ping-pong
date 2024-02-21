//! pong-provider capability provider
//!
//!
use wasmbus_rpc::provider::prelude::*;
use wasmcloud_examples_pong::*;

// main (via provider_main) initializes the threaded tokio executor,
// listens to lattice rpcs, handles actor links,
// and returns only when it receives a shutdown message
//
fn main() -> Result<(), Box<dyn std::error::Error>> {
    provider_main(PongProviderProvider::default(), Some("PongProvider".to_string()))?;

    eprintln!("pong-provider provider exiting");
    Ok(())
}

/// pong-provider capability provider implementation
#[derive(Default, Clone, Provider)]
#[services(Pong)]
struct PongProviderProvider {}

/// use default implementations of provider message handlers
impl ProviderDispatch for PongProviderProvider {}
impl ProviderHandler for PongProviderProvider {}

/// Handle Factorial methods
#[async_trait]
impl Pong for PongProviderProvider {
    /// accepts a number and calculates its factorial
    async fn echo(&self, _ctx: &Context, _arg: &Request) -> RpcResult<Response> {
        Ok(Response { result: Option::from(String::from("pong!")) })
    }
}
