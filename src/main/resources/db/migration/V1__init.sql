-- Klother — initial schema

CREATE TABLE users (
    id                      BIGSERIAL PRIMARY KEY,
    email                   VARCHAR(320) UNIQUE NOT NULL,
    display_name            VARCHAR(120)        NOT NULL,
    gender                  VARCHAR(20)         NOT NULL DEFAULT 'women',
    password_hash           VARCHAR(120),

    -- Body measurements (cm)
    height_cm               REAL,
    chest_cm                REAL,
    waist_cm                REAL,
    hip_cm                  REAL,
    shoulder_width_cm       REAL,
    inseam_cm               REAL,

    measurements_updated_at TIMESTAMPTZ,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE products (
    id          BIGSERIAL PRIMARY KEY,
    name        VARCHAR(255)       NOT NULL,
    brand       VARCHAR(120)       NOT NULL,
    category    VARCHAR(50)        NOT NULL,   -- TOPS, BOTTOMS, DRESSES, ...
    price_gbp   NUMERIC(10, 2)     NOT NULL,
    image_url   VARCHAR(500),
    description TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE product_sizes (
    product_id  BIGINT REFERENCES products(id) ON DELETE CASCADE,
    size_label  VARCHAR(10) NOT NULL,           -- XS, S, M, L, XL, XXL
    stock_qty   INTEGER     NOT NULL DEFAULT 0,
    PRIMARY KEY (product_id, size_label)
);

CREATE TABLE orders (
    id          BIGSERIAL PRIMARY KEY,
    user_id     BIGINT REFERENCES users(id),
    status      VARCHAR(30) NOT NULL DEFAULT 'pending',
    total_gbp   NUMERIC(10, 2),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE order_items (
    id         BIGSERIAL PRIMARY KEY,
    order_id   BIGINT REFERENCES orders(id) ON DELETE CASCADE,
    product_id BIGINT REFERENCES products(id),
    size_label VARCHAR(10) NOT NULL,
    quantity   INTEGER     NOT NULL DEFAULT 1,
    price_gbp  NUMERIC(10, 2) NOT NULL
);

-- Indexes
CREATE INDEX idx_users_email           ON users(email);
CREATE INDEX idx_products_category     ON products(category);
CREATE INDEX idx_product_sizes_size    ON product_sizes(size_label);
