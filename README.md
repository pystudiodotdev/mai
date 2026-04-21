# mai

> An AI-integrated e-commerce system built with cutting-edge technology.

---

## Introduction

**mai** is a modern, high-performance e-commerce platform that seamlessly blends artificial intelligence with the full e-commerce lifecycle. Designed for reliability and scale, mai empowers businesses to manage their operations across multiple channels from a single, unified system.

---

## Features

- **AI Integration** — Artificial intelligence is woven into the core of mai, powering product recommendations, demand forecasting, customer insights, and automated decision-making across every layer of the platform.

- **Omnichannel Support** — Deliver a consistent and unified experience across all sales channels — online storefronts, marketplaces, social commerce, and beyond — from one central platform.

- **Comprehensive Tools** — A full suite of tools covering the entire e-commerce lifecycle: inventory management, order processing, customer management, analytics, and more.

- **Zero-Cost Connectivity** — Connect to leading marketplaces and third-party platforms without additional integration costs, removing barriers to expanding your sales reach.

---

## Tech Stack

| Layer       | Technology                                      |
|-------------|-------------------------------------------------|
| Language    | [Rust](https://www.rust-lang.org/)              |
| Framework   | [Leptos 0.8](https://leptos.dev/) (SSR + Axum)  |
| Styling     | [Tailwind CSS](https://tailwindcss.com/)        |
| Database    | [PostgreSQL](https://www.postgresql.org/) + sqlx |
| Config      | dotenvy (`.env` files)                          |

Rust provides memory safety and near-zero runtime overhead, while Leptos enables reactive, full-stack web applications — together forming a foundation built for performance and reliability.

---

## Project Structure

```
mai/
├── Cargo.toml                 # Dependencies & cargo-leptos metadata
├── tailwind.config.js         # Tailwind CSS configuration
├── .env / .env.example        # Environment variables
├── migrations/                # SQL migrations (sqlx)
│   └── 0001_initial_schema.sql
├── public/                    # Static assets
├── style/
│   └── tailwind.css           # Tailwind entry point
├── src/
│   ├── main.rs                # Axum server entry point (SSR)
│   ├── lib.rs                 # WASM hydration entry + module tree
│   ├── app/                   # Leptos components & routing
│   │   ├── components/        # Reusable UI (nav, footer, layout)
│   │   └── pages/             # Page components
│   │       ├── home.rs        #   Landing page
│   │       ├── auth/          #   Login & registration
│   │       ├── storefront/    #   Catalog & product detail
│   │       └── admin/         #   Dashboard, user & product mgmt
│   ├── server/                # Leptos server functions
│   │   ├── auth.rs            #   Authentication endpoints
│   │   ├── products.rs        #   Product CRUD
│   │   └── admin.rs           #   Admin-only endpoints
│   ├── models/                # Domain types & DB queries
│   │   ├── user.rs
│   │   ├── product.rs
│   │   └── category.rs
│   └── ai/                    # AI module (placeholder)
│       ├── recommendations.rs #   Product recommendation engine
│       └── forecasting.rs     #   Demand forecasting engine
└── tests/                     # Integration & unit tests
    ├── user_tests.rs
    └── product_tests.rs
```

---

## Getting Started

### Prerequisites

- **Rust nightly** — `rustup toolchain install nightly --allow-downgrade`
- **WASM target** — `rustup target add wasm32-unknown-unknown`
- **cargo-leptos** — `cargo install cargo-leptos --locked`
- **PostgreSQL** — a running instance (local or Docker)
- **Tailwind CSS** — installed via npm (`npm install -g tailwindcss`)

### Setup

```bash
# Clone the repository
git clone https://github.com/pystudiodotdev/mai.git && cd mai

# Copy the environment template and configure your database
cp .env.example .env
# Edit .env with your DATABASE_URL

# Run database migrations
cargo sqlx migrate run

# Start the development server
cargo leptos watch
```

The application will be available at **http://127.0.0.1:3000**.

### Running Tests

```bash
# Unit tests (no database required)
cargo test

# Integration tests (requires PostgreSQL)
cargo test -- --ignored
```

---

## License

This project is licensed under the [MIT License](LICENSE).
