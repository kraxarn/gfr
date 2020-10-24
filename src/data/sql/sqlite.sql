create table if not exists Config
(
    version integer not null
);

create table if not exists Apartments
(
    apartment_number               text    not null,
    elevator                       integer not null,
    floor                          integer not null,
    heating_included               integer not null,
    hired                          integer not null,
    hot_water_included             integer not null,
    household_electricity_included integer not null,
    object_number                  text    not null,
    rent                           integer not null,
    size                           integer not null,
    stair_cleaning_included        integer not null
);