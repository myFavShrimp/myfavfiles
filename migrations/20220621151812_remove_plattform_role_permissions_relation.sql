DROP TABLE public.platform_role_permission;

ALTER TABLE public.platform_role ADD permissions public."_platform_permissions_enum" NULL;
