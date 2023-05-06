-- Add up migration script here
alter table temperatura add  constraint ciudad_fecha_unica  unique (ciudad_id, fecha);
