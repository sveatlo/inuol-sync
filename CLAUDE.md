# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`inuol-sync` is a Rust webhook receiver that synchronizes clients and invoices from Invoice Ninja to the UOL accounting system. It listens for Invoice Ninja webhook events and makes corresponding API calls to UOL, then writes back UOL-assigned IDs to Invoice Ninja.

## Commands

```bash
# Build
cargo build
cargo build --release

# Run (requires config.toml or APP_ env vars)
cargo run

# Format & Lint (clippy::pedantic is enforced — run before committing)
cargo fmt
cargo clippy -- -D warnings

# Docker
docker compose up --build
```

There are currently no automated tests (`[dev-dependencies]` is empty).

## Architecture

The service is structured as a thin Axum HTTP server with two webhook endpoints:

- `POST /webhooks/client-created` — receives an Invoice Ninja client payload
- `POST /webhooks/invoice-created` — receives an Invoice Ninja invoice payload (with embedded client info)

**Request flow:**

1. **`server.rs`** — defines routes and `AppState`; deserializes JSON bodies into models
2. **`handlers.rs`** (`SyncHandler`) — orchestrates the sync:
   - Client: transform → `UolClient::create_contact` → `InvoiceNinjaClient::update_client` (stores UOL `contact_id` in Invoice Ninja `custom_value1`)
   - Invoice: read `client.custom_value1` as `buyer_id` → transform → `UolClient::create_invoice` → optionally `InvoiceNinjaClient::update_invoice` if number differs
3. **`uol_client.rs`** — HTTP client for UOL API (Basic Auth); endpoints: `POST /contacts`, `POST /sales_invoices`
4. **`invoice_ninja_client.rs`** — HTTP client for Invoice Ninja API (token header `X-API-TOKEN`); endpoints: `PUT /clients/:id`, `PUT /invoices/:id`
5. **`models.rs`** — all request/response structs for both systems
6. **`config.rs`** — loaded via `figment` from `config.toml` then overridden by `APP_`-prefixed env vars (double underscore for nesting, e.g. `APP_SERVER__PORT`)

**Key data conventions:**
- UOL `contact_id` is stored in Invoice Ninja client `custom_value1`
- UOL `product_id` is stored in Invoice Ninja line item `custom_value1`
- Country IDs mapped: `840`→`US`, `703`→`SK`, `203`→`CZ`
- Currency IDs mapped: `3`→`EUR`, `51`→`CZK`, else `USD`
- Language IDs mapped: `1`→`en-US`, else `cs-CZ`

## Configuration

Config is loaded from `config.toml` (not committed) and/or `APP_*` environment variables. See `.env.example` for all required variables. The `APP_` prefix with `__` separators maps to nested TOML keys (e.g. `APP_INVOICE_NINJA__API_TOKEN` → `invoice_ninja.api_token`).

## Debugging

Set `RUST_LOG` to control log verbosity at runtime (e.g. `RUST_LOG=debug cargo run`). The service uses `tracing` with `EnvFilter`, so per-module filtering works (e.g. `RUST_LOG=invoice_ninja_sync=debug`).
