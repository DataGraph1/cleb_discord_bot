CREATE TABLE coin_transactions (
    transaction_id BIGINT PRIMARY KEY,
    user_id TEXT NOT NULL,
    amount BIGINT NOT NULL,
    date BIGINT NOT NULL
)