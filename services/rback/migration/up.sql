DROP TABLE IF EXISTS "roles";
CREATE TABLE "roles"(
    role_id TEXT NOT NULL PRIMARY KEY,
    permission_id TEXT NOT REFERENCES "permissions" ("permission_id") ON DELETE CASCADE,
);

DROP TABLE IF EXISTS "permissions";
CREATE TABLE "permissions"(
    permission_id TEXT NOT NULL PRIMARY KEY,
    "description" TEXT NOT NULL,
);

DROP TABLE IF EXISTS "user_role";
CREATE TABLE "user_role"(
    role_id TEXT NOT NULL PRIMARY KEY REFERENCES "roles" ("role_id") ON DELETE CASCADE,
    user_id TEXT NOT NULL PRIMARY KEY,
);
