-- Add up migration script here
create table temperatura (
	   id serial primary key,
	   ciudad_id serial not null,
	   fecha date not null,
	   temperatura double precision not null,
	   constraint fk_ciudad foreign key (ciudad_id) references ciudad (id)
);
