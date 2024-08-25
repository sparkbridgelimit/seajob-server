CREATE TABLE IF NOT EXISTS public.user_activation_log (
    id BIGINT PRIMARY KEY,
    user_id BIGINT NOT NULL,
    activation_code_id BIGINT NOT NULL,
    biz_code VARCHAR NOT NULL,
    activated_at timestamp with time zone NOT NULL,
    expires_at timestamp with time zone NOT NULL,
    create_time timestamp with time zone DEFAULT now(),
    update_time timestamp with time zone DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_ ON user_activation_log(user_id, biz_code);