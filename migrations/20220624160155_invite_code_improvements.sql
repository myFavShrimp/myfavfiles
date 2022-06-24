ALTER TABLE public.invite_code ALTER COLUMN id SET DEFAULT gen_random_uuid();
ALTER TABLE public.invite_code ADD CONSTRAINT invite_code_pkey PRIMARY KEY (id);
ALTER TABLE public.invite_code ALTER COLUMN expiration DROP NOT NULL;
