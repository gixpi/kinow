DROP TABLE IF EXISTS "users";
CREATE TABLE "users"(
    user_id SERIAL NOT NULL PRIMARY KEY,
    phone_number TEXT NOT NULL,
    "status" TEXT NOT NULL,
    created_at timestamp NOT NULL
);