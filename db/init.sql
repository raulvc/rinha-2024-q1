CREATE TABLE clients
(
    id             INTEGER PRIMARY KEY NOT NULL,
    name           VARCHAR(50)         NOT NULL,
    negative_limit INTEGER             NOT NULL,
    balance        INTEGER             NOT NULL
);

-- obs: a ROW ID is implicitly declared for every table bellow (we have no choice in the matter)

CREATE TABLE transactions
(
    client_id   INTEGER                             NOT NULL,
    amount      INTEGER                             NOT NULL,
    operation   CHAR(1)                             NOT NULL,
    description VARCHAR(10)                         NOT NULL,
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT fk_client_transaction_id
        FOREIGN KEY (client_id) REFERENCES clients (id)
);

CREATE INDEX idx_transaction_date ON transactions (created_at DESC);

INSERT INTO clients (id, name, negative_limit, balance)
VALUES (1, 'o barato sai caro', 1000 * 100, 0),
       (2, 'zan corp ltda', 800 * 100, 0),
       (3, 'les cruders', 10000 * 100, 0),
       (4, 'padaria joia de cocaia', 100000 * 100, 0),
       (5, 'kid mais', 5000 * 100, 0);
