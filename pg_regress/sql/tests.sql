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

-- Creation of a BMOC
SELECT create_bmoc_psql(
    29,
    ARRAY[
      8202, 8203, 8206, 8207, 8218, 8224, 8225, 8226, 8227, 8228, 8229, 8230, 8231, 8232, 8233,
      8234, 8236, 8237, 8239, 8240, 8241, 8242, 8243, 8246, 8248, 8249, 8250, 8251, 8252, 8254,
      8320, 8333, 8335, 8336, 8337, 8338, 8339, 8340, 8342, 8344, 8345, 8346, 8347, 8348, 8355,
      8356, 8357, 8358, 8359, 8360, 8361, 8362, 8363, 8364, 8365, 8366, 8367, 8368, 8369, 8370,
      8376, 8704, 8705, 8706, 8707, 8708, 11280, 11281, 11283, 11284, 11285, 11286, 11287, 11292,
      11293, 11328, 11329, 11330, 11331, 11332, 11333, 11334, 11335, 11336, 11337, 11340, 11341,
      11344, 11345, 11346, 11347, 11348, 11349, 11350, 11351, 11352, 11353, 11520, 11521
    ]
);

-- Function test : hpx_elliptical_cone
SELECT hpx_elliptical_cone_coverage(3, RADIANS(36.80105218), RADIANS(56.78028536), RADIANS(14.93), RADIANS(4.93), RADIANS(75.0));

-- PROBLEMATIC QUERY (doesn't use the bitmap index scan)
-- SELECT * FROM tyc2 WHERE hpx_hash(29, ra_icrs_, de_icrs_) <@ int8multirange(int8range(0,1000), int8range(150,250));

-- SOLUTION (provides a query you have to copy and paste and this query uses the bitmap index scan)
SELECT moc_contains_element_query('tyc2', 'hpx_hash(29, ra_icrs_, de_icrs_)', create_range_moc_psql(29, ARRAY[int8range(100,200),int8range(300,400)]));
