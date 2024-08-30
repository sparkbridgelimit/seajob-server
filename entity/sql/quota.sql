CREATE TABLE IF NOT EXISTS public.quota
(
    id                   bigint NOT NULL,
    user_id              bigint NOT NULL,
    biz_code             VARCHAR NOT NULL,
    quota                bigint NOT NULL,
    usage                bigint NOT NULL,
    remaining            bigint NOT NULL,
    create_time timestamp with time zone DEFAULT now(),
    update_time timestamp with time zone DEFAULT now(),
    PRIMARY KEY (id)
);

CREATE INDEX IF NOT EXISTS idx_quota ON quota(user_id, biz_code);
