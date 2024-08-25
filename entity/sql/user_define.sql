CREATE TABLE IF NOT EXISTS public.user_define
(
	id bigint NOT NULL,
    PRIMARY KEY (id),
	status VARCHAR(20),
	extra VARCHAR,
    create_time timestamp with time zone DEFAULT now(),
    update_time timestamp with time zone DEFAULT now()
);

ALTER TABLE IF EXISTS public.user_define
    OWNER to seajob;

COMMENT ON COLUMN public.user_define.create_time IS '创建时间';
COMMENT ON COLUMN public.user_define.update_time IS '更新时间';
COMMENT ON COLUMN public.user_define.status IS '用户状态';