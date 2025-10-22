-- Add migration script here
create table subscriptions(
    id uuid not null,
    email text not null,
    name text not null,
    subscribed_at timestamptz not null default now(),
    primary key (id)
);

create index idx_subscriptions_email on subscriptions (email);