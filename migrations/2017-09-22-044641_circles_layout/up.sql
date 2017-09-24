CREATE TABLE "users" (
    uid varchar(255) NOT NULL, 
    username varchar(255), 
    picture varchar(1023), 
    email varchar(511), 
    auth_time timestamp NOT NULL, 
    auth_until timestamp NOT NULL, 
    PRIMARY KEY (uid)
);

CREATE TABLE "position_records" (
    time timestamp NOT NULL, 
    user_uid varchar(255) NOT NULL, 
    latitude float8 NOT NULL, 
    longitude float8 NOT NULL, 
    accuracy float4, 
    altitude float8, 
    vertical_accuracy float4, 
    bearing float4, 
    speed float4, 
    speed_accuracy float4, 
    location varchar(1023), 
    PRIMARY KEY (time, user_uid)
);
