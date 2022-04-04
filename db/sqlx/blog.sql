CREATE TABLE blog(
  id int primary key not null,
  title text not null,
  tags text,
  create_time real not null,
  update_time real,
  body text not null
);
