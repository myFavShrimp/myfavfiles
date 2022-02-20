CREATE TABLE "user" (
    "id" UUID DEFAULT gen_random_uuid() PRIMARY KEY NOT NULL,
    "name" VARCHAR(255) NOT NULL UNIQUE,
    "password" TEXT NOT NULL,
    "is_admin" BOOLEAN NOT NULL
);

CREATE TABLE "group" (
    "id" UUID DEFAULT gen_random_uuid() PRIMARY KEY NOT NULL,
    "name" VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE "group_member" (
    "id" UUID DEFAULT gen_random_uuid() PRIMARY KEY NOT NULL,
    "user_id" UUID NOT NULL,
    "group_id" UUID NOT NULL,
    "is_admin" BOOLEAN NOT NULL
);

CREATE TABLE "group_role" (
    "id" UUID DEFAULT gen_random_uuid() PRIMARY KEY NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "group_id" UUID NOT NULL
);

CREATE TABLE "group_member_role" (
    "group_member_id" UUID NOT NULL,
    "group_role_id" UUID NOT NULL
);

CREATE TABLE "platform_role" (
    "id" UUID DEFAULT gen_random_uuid() PRIMARY KEY NOT NULL,
    "name" VARCHAR(255) NOT NULL
);

CREATE TABLE "user_role" (
    "user_id" UUID NOT NULL,
    "platform_role_id" UUID NOT NULL
);

CREATE TABLE "invite_code" (
    "id" UUID NOT NULL,
    "expiration" TIMESTAMP NOT NULL
);

CREATE TABLE "user_file_share" (
    "id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "expiration" TIMESTAMP NOT NULL
);

CREATE TABLE "group_file_share" (
    "id" UUID NOT NULL,
    "group_id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "expiration" TIMESTAMP NOT NULL
);

CREATE TYPE platform_permissions_enum AS ENUM (
    'create_invite_code',
    'ban_user',
    'administrator',
    'manage_groups',
    'create_groups',
    'upload_files',
    'delete_files',
    'manage_roles'
);

-- ENUM ARRAY gets turned into String by sea-orm entity generation
CREATE TABLE "platform_role_permission" (
    "platform_role_id" UUID NOT NULL,
    "permission" platform_permissions_enum NOT NULL,
    UNIQUE ("platform_role_id", "permission")
);

CREATE TYPE group_permissions_enum AS ENUM (
    'create_invite_code',
    'kick_user',
    'administrator',
    'upload_files',
    'delete_files',
    'manage_roles'
);

-- ENUM ARRAY gets turned into String by sea-orm entity generation
CREATE TABLE "group_role_permission" (
    "group_role_id" UUID NOT NULL,
    "permission" group_permissions_enum NOT NULL,
    UNIQUE ("group_role_id", "permission")
);

ALTER TABLE "group_member" ADD CONSTRAINT "group_member_user_id_user_id" FOREIGN KEY ("user_id") REFERENCES "user"("id");
ALTER TABLE "group_member" ADD CONSTRAINT "group_member_group_id_group_id" FOREIGN KEY ("group_id") REFERENCES "group"("id");
ALTER TABLE "group_role" ADD CONSTRAINT "group_role_group_id_group_id" FOREIGN KEY ("group_id") REFERENCES "group"("id");
ALTER TABLE "group_member_role" ADD CONSTRAINT "group_member_role_group_member_id_group_member_id" FOREIGN KEY ("group_member_id") REFERENCES "group_member"("id");
ALTER TABLE "group_member_role" ADD CONSTRAINT "group_member_role_group_role_id_group_role_id" FOREIGN KEY ("group_role_id") REFERENCES "group_role"("id");
ALTER TABLE "user_role" ADD CONSTRAINT "user_role_user_id_user_id" FOREIGN KEY ("user_id") REFERENCES "user"("id");
ALTER TABLE "user_role" ADD CONSTRAINT "user_role_platform_role_id_platform_role_id" FOREIGN KEY ("platform_role_id") REFERENCES "platform_role"("id");
ALTER TABLE "user_file_share" ADD CONSTRAINT "user_file_share_user_id_user_id" FOREIGN KEY ("user_id") REFERENCES "user"("id");
ALTER TABLE "group_file_share" ADD CONSTRAINT "group_file_share_group_id_group_id" FOREIGN KEY ("group_id") REFERENCES "group"("id");
ALTER TABLE "group_file_share" ADD CONSTRAINT "group_file_share_user_id_user_id" FOREIGN KEY ("user_id") REFERENCES "user"("id");
ALTER TABLE "platform_role_permission" ADD CONSTRAINT "platform_role_permission_platform_role_id_platform_role_id" FOREIGN KEY ("platform_role_id") REFERENCES "platform_role"("id");
ALTER TABLE "group_role_permission" ADD CONSTRAINT "group_role_permission_group_role_id_group_role_id" FOREIGN KEY ("group_role_id") REFERENCES "group_role"("id");
