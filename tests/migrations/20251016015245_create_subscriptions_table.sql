-- Add migration script here

create table subscriptions( 
    id uuid not null,  
    primary key (id), 
    email text not null unique, -- unique约束唯一性
    name text not null,  -- text作为amial和name的类型
    subscriptions_at timestamptz not null -- 追踪订阅时间(timestamptz时区感知)
)