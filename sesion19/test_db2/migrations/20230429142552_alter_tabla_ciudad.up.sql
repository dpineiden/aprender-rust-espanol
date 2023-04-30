-- Add up migration script here
alter table ciudad  add constraint pais_nombre_unico unique (pais, nombre);
