# Business Pulse

This project is built using Cloudflare Workers and is designed to provide business data by querying a given business ID. The data is fetched from the "avoindata.prh.fi" API, and cached in Cloudflare's KV storage for future request.

## Features

- Business data retrieval from "avoindata.prh.fi" API
- Caching of business data using Cloudflare KV storage

## Getting Started

Before you start, make sure you have the following requirements:

- Rust installed
- Wrangler installed

Clone the repository

```sh
Copy code
git clone https://github.com/waltu/business_pulse.git
cd business_pulse
```

## Configure Wrangler

Set up your wrangler.toml configuration file with your Cloudflare account details, API token, and worker settings.

### Build

To build the project, run:

```sh
wrangler build
```

### Preview

To preview the worker, run:

```sh
wrangler preview
```

### Publish

To deploy the worker to your Cloudflare account, run:

```sh
wrangler publish
```

## Usage

The worker provides two main routes:

1. `GET /`: A simple greeting message for Business Pulse users.
2. `GET /business/:business_id`: Returns business data in JSON format for the given business ID. If the data is not available in the KV storage, it fetches the data from the "avoindata.prh.fi" API and caches it for future requests.

## WebAssembly

`workers-rs` is meant to be executed as compiled WebAssembly, and as such so **must** all the code you write and depend upon. All crates and modules used in Rust-based Workers projects have to compile to the `wasm32-unknown-unknown` triple.

Read more about this on the [`workers-rs`](https://github.com/cloudflare/workers-rs) project README.

## Contributing

If you'd like to contribute, feel free to open a pull request or report any issues. Contributions are welcome!

## License

This project is licensed under the MIT License.
