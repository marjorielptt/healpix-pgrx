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

-- Create a MOC from a row of moc_table
-- Make sure you created the function moc_from_moc_table() whose code is in ../setup.sql
SELECT moc_from_moc_table(2);

-- Function test : moc_to_ascii
SELECT moc_to_ascii(create_range_moc_psql(29, ARRAY[int8range(100,200),int8range(300,400)]));

-- Function test : moc_and
SELECT create_range_moc_psql(29, ARRAY[int8range(100,200),int8range(300,400)]) & create_range_moc_psql(28, ARRAY[int8range(500,600)]);

-- Creation of a BMOC
SELECT create_bmoc_psql(29, ARRAY[8202, 8203, 8206, 8207, 8218, 8224, 8225]);

-- Function test : hpx_elliptical_cone
SELECT hpx_elliptical_cone_coverage(3, 36.80105218, 56.78028536, 14.93, 4.93, 75.0);

-- PROBLEMATIC QUERY FOR MOCs : doesn't use the bitmap index scan
-- SELECT * FROM hip_table WHERE hpx_hash(29, raicrs, deicrs) <@ int8multirange(int8range(100,200), int8range(300,400));

-- SOLUTION : we create a range that only contains the element we want to index the table with
CREATE INDEX hpx_hash_hip_idx ON hip_table USING GIST(hpx_hash_range(29,raicrs,deicrs));

-- Then this query uses the index
SELECT * FROM hip_table WHERE hpx_hash_range(29, raicrs, deicrs) <@ int8multirange('[100, 200)', '[300,400)');

-- Same query but with a function that returns the ranges of a moc
SELECT * FROM hip_table WHERE hpx_hash_range(29, raicrs, deicrs) <@ to_ranges(create_range_moc_psql(29, ARRAY[int8range(100,200),int8range(300,400)]));

-- Create a MOC from a cone
SELECT moc_from_cone(13.158329, -72.80028, 5.64323, 6, 5, 'All');

-- Return the cells contained in the moc created from a cone
SELECT * FROM hip_table WHERE hpx_hash_range(29, raicrs, deicrs) <@ to_ranges_moc_psql(moc_from_cone(13.158329, -72.80028, 5.64323, 6, 5, 'All'));

-- Return the cells contained in the bmoc created from a cone
explain select * from hip_table where hpx_hash_range(29, raicrs, deicrs) <@ to_ranges_bmoc_psql(hpx_cone_coverage_approx(29, 13.158329, -72.80028, 5.64323));

-- Returns the cells COMPLETELY contained in the cone (flag=1)
SELECT * FROM hip_table WHERE hpx_hash_range(29, raicrs, deicrs) <@ to_int8multirange(hpx_flag_one(create_bmoc_psql(29, ARRAY[8202, 8203, 8206, 8207, 8218, 8224, 8225])));

-- Returns the cells PARTIALLY contained in the cone (flag=0)
SELECT * FROM hip_table WHERE hpx_hash_range(29, raicrs, deicrs) <@ to_int8multirange(hpx_flag_zero(create_bmoc_psql(29, ARRAY[8202, 8203, 8206, 8207, 8218, 8224, 8225])));

-- Only uses the index in the InitPlan1 but not everywhere
SELECT * FROM hip_table h
WHERE EXISTS (
    SELECT 1 FROM hip_table h
    WHERE
        hpx_hash_range(29, h.raicrs, h.deicrs) <@ to_int8multirange(hpx_flag_one(hpx_cone_coverage_approx(6, 13.158329, -72.80028, 5.64323))))
    OR 
        ((hpx_hash_range(29, h.raicrs, h.deicrs) <@ to_int8multirange(hpx_flag_zero(hpx_cone_coverage_approx(6, 13.158329, -72.80028, 5.64323))))
        AND (hpx_contains_bool(hpx_cone_coverage_approx(6, 13.158329, -72.80028, 5.64323),h.raicrs, h.deicrs))
);
