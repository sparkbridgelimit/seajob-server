CREATE TABLE IF NOT EXISTS public.role
(
    id                   bigint NOT NULL,
    name                 VARCHAR(255) NOT NULL,
    code                 VARCHAR(255) NOT NULL,
    desc                 VARCHAR(255) NOT NULL,
    create_time timestamp with time zone DEFAULT now(),
    update_time timestamp with time zone DEFAULT now(),
    PRIMARY KEY (id)
);

# 为code字段添加唯一索引
CREATE UNIQUE INDEX IF NOT EXISTS idx_role_code ON public.role (code);