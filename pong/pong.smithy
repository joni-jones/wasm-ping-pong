// pong.smithy
// A simple service that calculates the factorial of a whole number


// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "org.wasmcloud.examples.pong", crate: "pong" } ]

namespace org.wasmcloud.examples.pong

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U32
use org.wasmcloud.model#U64

/// The Pong service has a single method, calculate, which
/// calculates the factorial of its whole number parameter.
@wasmbus(
    contractId: "wasmcloud:examples:pong",
    actorReceive: true,
    providerReceive: true )
service Pong {
  version: "0.1",
  operations: [ Echo ]
}

/// Calculates the factorial (n!) of the input parameter
operation Echo {
  input: Request,
  output: Response
}

structure Request {
    param: String
}

structure Response {
    result: String
}