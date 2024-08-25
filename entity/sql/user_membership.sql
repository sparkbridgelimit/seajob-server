CREATE TABLE IF NOT EXISTS public.user_membership (
    id BIGINT PRIMARY KEY,
    user_id BIGINT NOT NULL,
    biz_code VARCHAR NOT NULL,
    expires_at timestamp with time zone NOT NULL,
    create_time timestamp with time zone DEFAULT now(),
    update_time timestamp with time zone DEFAULT now()
    PRIMARY KEY (id)
);

COMMENT ON COLUMN public.user_membership.id IS 'Primary key';
COMMENT ON COLUMN public.user_membership.user_id IS '用户ID';
COMMENT ON COLUMN public.user_membership.biz_code IS '业务标识';
COMMENT ON COLUMN public.user_membership.expires_at IS '失效时间';
COMMENT ON COLUMN public.user_membership.create_time IS '创建时间';
COMMENT ON COLUMN public.user_membership.update_time IS '更新时间';

CREATE INDEX IF NOT EXISTS idx_user_biz_code ON user_membership(user_id, biz_code);
