set
timezone to 'Asia/Shanghai';

drop table if exists courses;

create table courses
(
    id          serial primary key,
    teacher_id  INT          not null,
    name        varchar(140) not null,
    time        TIMESTAMP default now(),
    description varchar(2000),
    format      varchar(30),
    structure   varchar(200),
    duration    varchar(30),
    price       integer,
    language    varchar(30),
    level       varchar(30)
);


insert into courses (teacher_id, name)
values (1,
        'First course');


insert into courses (teacher_id, name)
values (1,
        'Second course');
