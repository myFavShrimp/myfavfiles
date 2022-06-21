DROP TABLE public.group_role_permission;

ALTER TABLE public.group_role ADD permissions public."_group_permissions_enum" NULL;
