CREATE TABLE blog(
  id int primary key not null,
  title text not null,
  tags text,
  create_time text not null,
  update_time text,
  body text not null
);
