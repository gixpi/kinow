DROP TABLE IF EXISTS "rts";
CREATE TABLE "rts"(
    id SERIAL PRIMARY KEY,
    device_id SERIAL NOT NULL,
    report_message TEXT NOT NULL,
    "description" TEXT NOT NULL,  
    announced_at TIMESTAMPTZ NOT NULL,
    received_at TIMESTAMPTZ NOT NULL
);
