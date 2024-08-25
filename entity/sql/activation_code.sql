CREATE TABLE IF NOT EXISTS public.activation_code (
    id BIGINT PRIMARY KEY,
    code VARCHAR NOT NULL,
    expire_at timestamp with time zone NOT NULL,
    biz_code VARCHAR NOT NULL,
    duration BIGINT NOT NULL,
    is_used BOOLEAN NOT NULL,
    create_time timestamp with time zone DEFAULT now(),
    update_time timestamp with time zone DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_unused ON activation_code(biz_code, is_used);

COMMENT ON COLUMN public.activation_code.id IS 'Primary key';
COMMENT ON COLUMN public.activation_code.code IS '激活码';
COMMENT ON COLUMN public.activation_code.expire_at IS '过期时间';
COMMENT ON COLUMN public.activation_code.biz_code IS '业务代码';
COMMENT ON COLUMN public.activation_code.duration IS '有效时长';
COMMENT ON COLUMN public.activation_code.is_used IS '是否已使用';
COMMENT ON COLUMN public.activation_code.create_time IS '创建时间';
COMMENT ON COLUMN public.activation_code.update_time IS '更新时间';

