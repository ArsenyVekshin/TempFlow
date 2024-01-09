drop table if exists users, direction, point, triggers, room, rack, sensor, room2rack, sens_owners;

-- Стержневые сущности
create table users (
    id serial primary key,
    username text,
    password text,
    contact text
);

create table direction (
    id serial primary key,
    x float,
    y float,
    z float
);

create table point (
   id serial primary key,
   x float,
   y float,
   z float
);

create table triggers (
   id serial primary key,
   min float,
   max float
);

-- Основные сущности

create table room (
    id serial primary key,
    name text,
    owner_id int references users(id),
    length float,
    width float,
    height float
);

create table rack (
    id serial primary key,
    name text,
    leftAngle int references point(id),
    length float,
    width float,
    height float,
    hotend int references direction(id),
    size int
);

create table sensor (
    id serial primary key,
    name text,
    position int references point(id),
    trig int references triggers(id),
    address text,
    protocol text,
    key text
);


-- Связующие таблицы

create table room2rack (
    room_id int references room(id),
    rack_id int references rack(id)
);

create table sens_owners (
    room_id int references room(id),
    rack_id int references rack(id) NULL,
    sens_id int references sensor(id)
);


