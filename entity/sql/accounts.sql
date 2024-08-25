CREATE TABLE IF NOT EXISTS public.accounts
(
    id                   bigint NOT NULL,
	user_id              bigint NOT NULL,
    provider_type        VARCHAR(255) NOT NULL,
    provider_id          VARCHAR(255) NOT NULL,
    provider_account_id  VARCHAR(255) NOT NULL,
    create_time timestamp with time zone DEFAULT now(),
    update_time timestamp with time zone DEFAULT now(),
    PRIMARY KEY (id)
);

CREATE INDEX provider_account_id ON accounts(provider_account_id);

CREATE INDEX provider_id ON accounts(provider_id);

CREATE INDEX user_id ON accounts(user_id);
