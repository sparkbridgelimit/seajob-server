CREATE TABLE IF NOT EXISTS public.user_role
(
    id                   bigint NOT NULL,
    user_id              bigint NOT NULL,
    role_id              bigint NOT NULL,
    create_time timestamp with time zone DEFAULT now(),
    update_time timestamp with time zone DEFAULT now(),
    PRIMARY KEY (id)
);

CREATE INDEX IF NOT EXISTS idx_user_role ON role(user_id, role_id);
