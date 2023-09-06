DROP TABLE IF EXISTS "tickets";
CREATE TABLE "tickets"(
    ticket_id SERIAL NOT NULL PRIMARY KEY,
    user_id SERIAL NOT NULL,
    "point" TEXT NOT NULL,
    ip TEXT NOT NULL,
    agent TEXT NOT NULL,
    expire_at TIMESTAMPTZ
)