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





insert into users (*)
    values ('admin', 'admin', 'default');

insert into direction (*)
    values (1.0, 0.0, 0.0),
           (-1.0, 0.0, 0.0);

insert into point (*)
    values (1.0, 1.0, 0.0),
           (1.0, 3.0, 0.0),
           (1.0, 5.0, 0.0),
           (4.0, 1.0, 0.0),
           (4.0, 3.0, 0.0),
           (4.0, 5.0, 0.0),
           (2.0, 2.0, 0.0), -- sensors inside racks lv1
           (2.0, 4.0, 0.0),
           (2.0, 5.75.0, 0.0),
           (5.0, 2.0, 0.0),
           (5.0, 4.0, 0.0),
           (5.0, 5.75.0, 0.0),
           (2.0, 2.0, 0.5), -- sensors inside racks lv2
           (2.0, 4.0, 0.5),
           (2.0, 5.75.0, 0.5),
           (5.0, 2.0, 0.5),
           (5.0, 4.0, 0.5),
           (5.0, 5.75.0, 0.5),


insert into triggers (*)
    values (0, 100);

insert into room (*)
    values ('demo_room', 1, 7.0, 7.5, 4.0);

insert into rack (*)
    values ('rack1', 1, 2.0, 2.0, 3.0, 1, 5),
           ('rack2', 2, 2.0, 2.0, 3.0, 1, 5),
           ('rack3', 3, 2.0, 1.5, 3.0, 1, 5),
           ('rack1', 4, 2.0, 2.0, 3.0, 2, 5),
           ('rack2', 5, 2.0, 2.0, 3.0, 2, 5),
           ('rack3', 6, 2.0, 1.5, 3.0, 2, 5);
insert into sensor (*)
    values ('rack1_ins', 1, )

