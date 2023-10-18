CREATE TABLE IF NOT EXISTS mints (
    id SERIAL PRIMARY KEY,
    token_id TEXT NOT NULL,
    to_address TEXT NOT NULL,
    trx_hash TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS deltas (
    id SERIAL PRIMARY KEY,
    contract TEXT NOT NULL,
    holder TEXT NOT NULL,
    balance INTEGER NOT NULL,
    block_number INTEGER NOT NULL
);

create table if not exists "public"."cursors"
(
	id         text not null constraint cursor_pk primary key,
	cursor     text,
	block_num  bigint,
	block_id   text
);
