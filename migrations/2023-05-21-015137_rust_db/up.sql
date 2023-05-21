-- Your SQL goes here
CREATE TABLE rust_db (
	id INT auto_increment primary key NOT NULL,
	name varchar(100) NOT NULL,
	last_name varchar(100) NOT NULL,
	email varchar(100) NULL
)