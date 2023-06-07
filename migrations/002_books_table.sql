create table books (
    id bigserial primary key,
    title varchar(255) not null,
    author varchar(255) not null,
    owner_id bigint not null,
    constraint fk_book_user foreign key (owner_id) references users(id) on delete cascade
);