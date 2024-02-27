### Structure

## Ping

Is the WASM actor component which functionality is extended by `pong` capability provider.

## Pong

An interface for a capability provider.

## PongProvider

An implementation of `pong` interface.

## How to Run
### start wasmcloud

> WASMCLOUD_OCI_ALLOWED_INSECURE allows to use local registry

```bash
wash up --allowed-insecure=localhost:5000 --nats-websocket-port 4001
```

Start Docker registry locally
```bash
docker run -it --rm -p 5000:5000 registry:2.8
```

Push Pong Capability Provider to the local registry

```bash
cd pong-provider && make push
```
> expected output: Successfully validated and pushed to localhost:5000/v2/pong_provider:0.1.0

Start the Capability Provider

```bash
cd pong-provider && make start
```

[optional] start WasmCloud UI

```bash
wash ui
```

Build Ping actor and replace path to the actor in `wadm.yaml`

```bash
cd ping && wash build
```

Deploy the actor

```bash
wash app deploy wadm.yaml
```
