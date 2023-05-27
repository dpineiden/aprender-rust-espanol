-- Add up migration script here
create table ciudad (
	   id serial primary key,
	   pais varchar(50) not null,
	   nombre varchar(50) not null
);
