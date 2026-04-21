-- mai — Initial database schema
-- Creates the core tables for users, categories, and products.

-- Enable UUID generation
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ============================================================
-- Users
-- ============================================================
CREATE TABLE users (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email       VARCHAR(255) NOT NULL UNIQUE,
    username    VARCHAR(100) NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role        VARCHAR(20)  NOT NULL DEFAULT 'customer',  -- 'customer' | 'admin'
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT now()
);

CREATE INDEX idx_users_email ON users (email);

-- ============================================================
-- Categories
-- ============================================================
CREATE TABLE categories (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name        VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    parent_id   UUID REFERENCES categories(id) ON DELETE SET NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- ============================================================
-- Products
-- ============================================================
CREATE TABLE products (
    id           UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name         VARCHAR(255)   NOT NULL,
    description  TEXT,
    price_cents  BIGINT         NOT NULL DEFAULT 0,
    currency     VARCHAR(3)     NOT NULL DEFAULT 'USD',
    sku          VARCHAR(100)   UNIQUE,
    stock_qty    INTEGER        NOT NULL DEFAULT 0,
    category_id  UUID           REFERENCES categories(id) ON DELETE SET NULL,
    is_active    BOOLEAN        NOT NULL DEFAULT true,
    image_url    TEXT,
    created_at   TIMESTAMPTZ    NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ    NOT NULL DEFAULT now()
);

CREATE INDEX idx_products_category ON products (category_id);
CREATE INDEX idx_products_sku      ON products (sku);
