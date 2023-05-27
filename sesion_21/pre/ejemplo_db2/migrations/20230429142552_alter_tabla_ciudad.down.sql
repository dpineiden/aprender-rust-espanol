-- Add down migration script here

alter table ciudad drop index pais_nombre_unico;
