ALTER TABLE public.group_file_share ALTER COLUMN id SET DEFAULT gen_random_uuid();
ALTER TABLE public.user_file_share ALTER COLUMN id SET DEFAULT gen_random_uuid();
