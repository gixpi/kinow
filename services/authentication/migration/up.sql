DROP TABLE IF EXISTS "users";
CREATE TABLE "users"(
    user_id SERIAL NOT NULL PRIMARY KEY,
    phone_number TEXT NOT NULL,
    "status" TEXT NOT NULL,
    created_at timestamp NOT NULL
);

DROP TABLE IF EXISTS "tokens";
CREATE TABLE "tokens"(
    access_token TEXT NOT NULL PRIMARY KEY,
    refresh_token TEXT NOT NULL,
    user_id SERIAL NOT NULL,
    session_id SERIAL NOT NULL,
    "status" TEXT NOT NULL,
    ip TEXT NOT NULL,
    agent TEXT NOT NULL,
    created_at timestamp NOT NULL,
    expire_at timestamp NOT NULL
);