DROP TABLE IF EXISTS "devices";
CREATE TABLE "devices"(
    device_id SERIAL NOT NULL PRIMARY KEY,
    device_type TEXT NOT NULL,
    serial_code TEXT NOT NULL,
    device_status TEXT NOT NULL,
    lock_code TEXT NOT NULL,
    user_id SERIAL NOT NULL, 
    created_at TIMESTAMPTZ NOT NULL
);

DROP TABLE IF EXISTS "tokens";
CREATE TABLE "tokens"(
    access_token TEXT NOT NULL PRIMARY KEY,
    refresh_token TEXT NOT NULL,
    device_id SERIAL NOT NULL,
    device_type TEXT NOT NULL,
    token_status TEXT NOT NULL,
    ip TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    access_token_expire_at TIMESTAMPTZ NOT NULL,
    refresh_token_expire_at TIMESTAMPTZ NOT NULL
);