CREATE TABLE "users" (
    uid varchar(255) NOT NULL, 
    username varchar(255), 
    picture varchar(1023), 
    email varchar(511), 
    auth_time timestamp NOT NULL, 
    auth_until timestamp NOT NULL, 
    PRIMARY KEY (uid)
);
