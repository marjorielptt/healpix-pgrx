-- this setup file is run immediately after the regression database is (re)created
-- the file is optional but you likely want to create the extension

-- CONFIGURATION

DROP EXTENSION IF EXISTS mogipix CASCADE;
CREATE EXTENSION mogipix;

-- Creation of the table hip_table
CREATE TABLE hip_table(HIP bigint, Vmag double precision, RAICRS double precision, DEICRS double precision);

-- Copy of the data from the csv to the table
COPY hip_table(HIP, Vmag, RAICRS, DEICRS) FROM PROGRAM 'tail -n +1 /home/mlapointe/Documents/Ressources/csv/hip_main.csv' DELIMITER ',' CSV HEADER;

-- Creation of an index on hpx_hash(29, raicrs, deicrs) for hip_table
CREATE INDEX mgx_hash_hip_idx ON hip_table (mgx_hash(29, raicrs, deicrs));
CREATE INDEX mgx_hash_range_hip_idx ON hip_table USING GIST(mgx_hash_range(29, raicrs, deicrs));

-- Creation of the table tyc2
CREATE TABLE tyc2 (
    recno integer NOT NULL,
    tyc1 smallint NOT NULL,
    tyc2 smallint NOT NULL,
    tyc3 smallint NOT NULL,
    pflag character(1) NOT NULL,
    ramdeg double precision,
    demdeg double precision,
    pmra double precision,
    pmde double precision,
    e_ramdeg float,
    e_demdeg float,
    e_pmra float,
    e_pmde float,
    epram double precision,
    epdem double precision,
    num smallint,
    q_ramdeg float,
    q_demdeg float,
    q_pmra float,
    q_pmde float,
    btmag float,
    e_btmag float,
    vtmag float,
    e_vtmag float,
    prox smallint NOT NULL,
    tyc character(1) NOT NULL,
    hip integer,
    ccdm character(3) NOT NULL,
    ra_icrs_ double precision NOT NULL,
    de_icrs_ double precision NOT NULL,
    epra_1990 float NOT NULL,
    epde_1990 float NOT NULL,
    e_radeg float NOT NULL,
    e_dedeg float NOT NULL,
    posflg character(1) NOT NULL,
    corr float NOT NULL
);

-- Copy of the data from the csv to the table
COPY tyc2 FROM '/home/mlapointe/Documents/Ressources/csv/tycho.csv' (FORMAT csv, DELIMITER ',', NULL '', HEADER false);

-- Creation of an index on hpx_hash(29, raicrs, deicrs) for tyc2
CREATE INDEX mgx_hash_tyc2_idx ON tyc2 (mgx_hash(29, ra_icrs_, de_icrs_));

-- FUNCTIONS

-- int8range[] -> int8multirange
CREATE OR REPLACE FUNCTION mgx_to_int8multirange(r int8range[])
RETURNS int8multirange AS $$
DECLARE
    result int8multirange;
BEGIN
    EXECUTE format(
        'SELECT ''{%s}''::int8multirange',
        array_to_string(r, ',')
    )
    INTO result;

    RETURN result;
END;
$$ LANGUAGE plpgsql IMMUTABLE STRICT;

-- Equivalent of the crate::moc::moc_to_ranges() function in Rust 
-- but this function provites the int8range[] -> int8multirange conversion
-- so it works directly in postgres
CREATE OR REPLACE FUNCTION mgx_moc_to_ranges_psql(moc rangemocpsql)
RETURNS int8multirange AS $$
DECLARE
    r int8range[];
    result int8multirange;
BEGIN
    r := mgx_moc_to_ranges(moc);
    EXECUTE format(
        'SELECT ''{%s}''::int8multirange',
        array_to_string(r, ',')
    )
    INTO result;

    RETURN result;
END;
$$ LANGUAGE plpgsql IMMUTABLE STRICT;

-- Equivalent of the crate::bmoc::hpx_to_ranges() function in Rust 
-- but this function provites the int8range[] -> int8multirange conversion
-- so it works directly in postgres
CREATE OR REPLACE FUNCTION mgx_bmoc_to_ranges_psql(bmoc BMOCpsql)
RETURNS int8multirange AS $$
DECLARE 
    r int8range[];
    result int8multirange;
BEGIN
    r := mgx_bmoc_to_ranges(bmoc);
    EXECUTE format(
        'SELECT ''{%s}''::int8multirange',
        array_to_string(r, ',')
    )
    INTO result;

    RETURN result;
END;
$$ LANGUAGE plpgsql IMMUTABLE STRICT;

-- Returns true if the cell (test_lon_deg, test_lat_deg) is in the cone created with the coordinates (lon_deg, lat_deg, radius_deg)
-- Uses the function hpx_contains_bool(...) to select only the cells in the BMOC
-- UNPRECISE
CREATE OR REPLACE FUNCTION mgx_in_cone_hpx(
    lon_deg double precision,
    lat_deg double precision,
    radius_deg double precision,
    test_lon_deg double precision,
    test_lat_deg double precision)
RETURNS boolean AS 
$$
    SELECT 
    mgx_hash_range(29, test_lon_deg, test_lat_deg)
    <@
    mgx_to_int8multirange(
        mgx_bmoc_flag_one(
            mgx_bmoc_cone_coverage_approx(
                mgx_best_starting_depth(radius_deg)+4, lon_deg, lat_deg, radius_deg)))
    OR
    ((mgx_hash_range(29, test_lon_deg, test_lat_deg)
    <@
    mgx_to_int8multirange(
        mgx_bmoc_flag_zero(
            mgx_bmoc_cone_coverage_approx(
                mgx_best_starting_depth(radius_deg)+4, lon_deg, lat_deg, radius_deg))))
        AND
        (mgx_bmoc_contains_bool(
            mgx_bmoc_cone_coverage_approx(
                mgx_best_starting_depth(5.64323)+4, 0.01814144, 3.94648893, 5.64323),
                test_lon_deg,
                test_lat_deg)))
$$
LANGUAGE sql;

-- Returns true if the cell (test_lon_deg, test_lat_deg) is in the cone created with the coordinates (lon_deg, lat_deg, radius_deg)
-- Uses the function skyregion::cone::contains(...) to select only the cells in the BMOC
-- VERY PRECISE
CREATE OR REPLACE FUNCTION mgx_in_cone(
    lon_deg double precision,
    lat_deg double precision,
    radius_deg double precision,
    test_lon_deg double precision,
    test_lat_deg double precision)
RETURNS boolean AS 
$$
    SELECT 
    mgx_hash_range(29, test_lon_deg, test_lat_deg)
    <@
    mgx_to_int8multirange(
        mgx_bmoc_flag_one(
            mgx_bmoc_cone_coverage_approx(
                mgx_best_starting_depth(radius_deg)+4, lon_deg, lat_deg, radius_deg)))
    OR
    ((mgx_hash_range(29, test_lon_deg, test_lat_deg)
    <@
    mgx_to_int8multirange(
        mgx_bmoc_flag_zero(
            mgx_bmoc_cone_coverage_approx(
                mgx_best_starting_depth(radius_deg)+4, lon_deg, lat_deg, radius_deg))))
        AND
        (mgx_skyregion_cone_contains(
            lon_deg, lat_deg, radius_deg, test_lon_deg, test_lat_deg)))
$$
LANGUAGE sql;

-- Returns true if the cell (test_lon_deg, test_lat_deg) is in the elliptical cone given in the parameters
-- Uses the function skyregion::ellipticalcone::contains(...) to select only the cells in the BMOC
-- VERY PRECISE
CREATE OR REPLACE FUNCTION mgx_in_elliptical_cone(
    lon_deg double precision,
    lat_deg double precision,
    a_deg double precision,
    b_deg double precision,
    pa_deg double precision,
    test_lon_deg double precision,
    test_lat_deg double precision)
RETURNS boolean AS 
$$
    SELECT 
    mgx_hash_range(29, test_lon_deg, test_lat_deg)
    <@
    mgx_to_int8multirange(
        mgx_bmoc_flag_one(
            mgx_bmoc_elliptical_cone_coverage(
                mgx_best_starting_depth(a_deg)+4, lon_deg, lat_deg, a_deg, b_deg, pa_deg)))
    OR
    ((mgx_hash_range(29, test_lon_deg, test_lat_deg)
    <@
    mgx_to_int8multirange(
        mgx_bmoc_flag_zero(
            mgx_bmoc_elliptical_cone_coverage(
                mgx_best_starting_depth(a_deg)+4, lon_deg, lat_deg, a_deg, b_deg, pa_deg))))
        AND
        (mgx_skyregion_elliptical_cone_contains(lon_deg, lat_deg, a_deg, b_deg, pa_deg, test_lon_deg, test_lat_deg)))
$$
LANGUAGE sql;

-- Returns true if the cell (test_lon_deg, test_lat_deg) is in the zone created with the coordinates given in the parameters
-- Uses the function skyregion::zone::contains(...) to select only the cells in the BMOC
-- VERY PRECISE
CREATE OR REPLACE FUNCTION in_zone(
    lon_min_deg double precision,
    lat_min_deg double precision,
    lon_max_deg double precision,
    lat_max_deg double precision,
    test_lon_deg double precision,
    test_lat_deg double precision)
RETURNS boolean AS 
$$
    SELECT 
    hpx_hash_range(29, test_lon_deg, test_lat_deg)
    <@
    to_int8multirange(
        hpx_flag_one(
            hpx_zone_coverage(
                hpx_best_starting_depth(lon_min_deg)+4, lon_min_deg, lat_min_deg, lon_max_deg, lat_max_deg)))
    OR
    ((hpx_hash_range(29, test_lon_deg, test_lat_deg)
    <@
    to_int8multirange(
        hpx_flag_zero(
            hpx_zone_coverage(
                hpx_best_starting_depth(lon_min_deg)+4, lon_min_deg, lat_min_deg, lon_max_deg, lat_max_deg))))
        AND
        (skyregion_zone_contains(lon_min_deg, lat_min_deg, lon_max_deg, lat_max_deg, test_lon_deg, test_lat_deg)))
$$
LANGUAGE sql;

-- Returns true if the cell (test_lon_deg, test_lat_deg) is in the polygon created with the coordinates given in the parameters
-- Uses the function skyregion::polygon::contains(...) to select only the cells in the BMOC

-- The vertices aren't included in the polygon
CREATE OR REPLACE FUNCTION in_polygon(
    vertices_deg vertexpsql[],
    exact_solution boolean,      -- calculates the exact solution or not (nb : the exact solution still need some tests)
    complement boolean,          -- if complement = true, the polygon's surface is the exterior of the polygon formed by the vertices on the sphere
    test_lon_deg double precision,
    test_lat_deg double precision)
RETURNS boolean AS 
$$
    SELECT 
    (
        hpx_hash_range(29, test_lon_deg, test_lat_deg)
        <@
        to_int8multirange(
            hpx_flag_one(
                hpx_polygon_coverage(
                    hpx_polygon_characteristic_depth(vertices_deg, complement), vertices_deg, exact_solution))))
    OR
    ((hpx_hash_range(29, test_lon_deg, test_lat_deg)
    <@
    to_int8multirange(
        hpx_flag_zero(
            hpx_polygon_coverage(
                hpx_polygon_characteristic_depth(vertices_deg, complement), vertices_deg, exact_solution))))
        AND
        (skyregion_polygon_contains(vertices_deg, complement, test_lon_deg, test_lat_deg)))
$$
LANGUAGE sql;

-- Returns true if the cell (test_lon_deg, test_lat_deg) is in the box created with the coordinates given in the parameters
-- Uses the function skyregion::polygon::contains(...) to select only the cells in the BMOC
-- VERY PRECISE
CREATE OR REPLACE FUNCTION in_box(
    lon_deg double precision,
    lat_deg double precision,
    a_deg double precision,
    b_deg double precision,
    pa_deg double precision,
    test_lon_deg double precision,
    test_lat_deg double precision)
RETURNS boolean AS 
$$
    SELECT 
    hpx_hash_range(29, test_lon_deg, test_lat_deg)
    <@
    to_int8multirange(
        hpx_flag_one(
            hpx_box_coverage(
                hpx_best_starting_depth(a_deg)+4, lon_deg, lat_deg, a_deg, b_deg, pa_deg)))
    OR
    ((hpx_hash_range(29, test_lon_deg, test_lat_deg)
    <@
    to_int8multirange(
        hpx_flag_zero(
            hpx_box_coverage(
                hpx_best_starting_depth(a_deg)+4, lon_deg, lat_deg, a_deg, b_deg, pa_deg))))
        AND
        (skyregion_box_contains(lon_deg, lat_deg, a_deg, b_deg, pa_deg, test_lon_deg, test_lat_deg)))
$$
LANGUAGE sql;

-- Returns true if the cell (test_lon_deg, test_lat_deg) is in the ring created with the coordinates given in the parameters
-- Uses the function skyregion::ring::contains(...) to select only the cells in the BMOC
-- VERY PRECISE
CREATE OR REPLACE FUNCTION in_ring(
    lon_deg double precision,
    lat_deg double precision,
    r_min_deg double precision,
    r_max_deg double precision,
    test_lon_deg double precision,
    test_lat_deg double precision)
RETURNS boolean AS 
$$
    SELECT 
    hpx_hash_range(29, test_lon_deg, test_lat_deg)
    <@
    to_int8multirange(
        hpx_flag_one(
            hpx_zone_coverage(
                hpx_best_starting_depth(r_min_deg)+4, lon_deg, lat_deg, r_min_deg, r_max_deg)))
    OR
    ((hpx_hash_range(29, test_lon_deg, test_lat_deg)
    <@
    to_int8multirange(
        hpx_flag_zero(
            hpx_zone_coverage(
                hpx_best_starting_depth(r_min_deg)+4, lon_deg, lat_deg, r_min_deg, r_max_deg))))
        AND
        (skyregion_ring_contains(lon_deg, lat_deg, r_min_deg, r_max_deg, test_lon_deg, test_lat_deg)))
$$
LANGUAGE sql;

-- CREATE OR REPLACE FUNCTION hpx_hash_range2(depth integer, lon double precision, lat double precision) RETURNS int8range AS
-- $$ 
--     DECLARE 
--         res bigint;
--     BEGIN
--         res := public.hpx_hash($1, $2, $3);
--         RETURN int8range(res, res+1);
--     END
-- $$
-- LANGUAGE plpgsql immutable;