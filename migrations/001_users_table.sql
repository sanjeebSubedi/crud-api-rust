create table users (
    id bigserial primary key,
    email varchar(255) unique not null,
    password varchar(255) not null,
    name varchar(255) not null
)