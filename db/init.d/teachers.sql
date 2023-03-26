set
    timezone to 'Asia/Shanghai';

drop table if exists teachers;

create table teachers
(
    id          serial primary key,
    name        varchar(100),
    picture_url varchar(100),
    profile     varchar(2000)
);

insert into teachers (name, picture_url, profile)
VALUES ('zhang san', 'https://phy.xyz', 'zhang san is a good teacher'),
       ('li si', 'https://phy.xyz', 'zhang san is a good teacher');
