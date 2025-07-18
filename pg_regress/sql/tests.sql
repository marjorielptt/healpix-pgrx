-- TESTS

-- Creation of a MOC table
CREATE TABLE moc_table(id bigint PRIMARY KEY, depth_max int4, ranges int8multirange);

-- Indexation of the ranges column
CREATE INDEX ranges_idx ON moc_table USING GIST (ranges);

-- Creation of a MOC
SELECT create_range_moc_psql(29, ARRAY[int8range(100,200),int8range(300,400)]);

-- Insertion of MOCs in moc_table
INSERT INTO moc_table (id, depth_max, ranges) VALUES (1, 29, int8multirange(int8range(100,200), int8range(300,400)));
INSERT INTO moc_table (id, depth_max, ranges) VALUES (2, 28, int8multirange(int8range(500,600)));
INSERT INTO moc_table (id, depth_max, ranges) VALUES (3, 27, int8multirange(int8range(150,250), int8range(350,450), int8range(550,650)));

-- Function test : moc_to_ascii
SELECT moc_to_ascii(create_range_moc_psql(29, ARRAY[int8range(100,200),int8range(300,400)]));

-- Function test : moc_and
SELECT create_range_moc_psql(29, ARRAY[int8range(100,200),int8range(300,400)]) & create_range_moc_psql(28, ARRAY[int8range(500,600)]);
