-- this setup file is run immediately after the regression database is (re)created
-- the file is optional but you likely want to create the extension

-- CONFIGURATION

DROP EXTENSION healpix_pgrx CASCADE;
CREATE EXTENSION healpix_pgrx;

-- Creation of the table hip_table
CREATE TABLE hip_table(HIP bigint, Vmag double precision, RAICRS double precision, DEICRS double precision);

-- Copy of the data from the csv to the table
COPY hip_table(HIP, Vmag, RAICRS, DEICRS) FROM PROGRAM 'tail -n +1 /home/mlapointe/Documents/Ressources/csv/hip_main.csv' DELIMITER ',' CSV HEADER;

-- Creation of an index on hpx_hash(29, raicrs, deicrs) for hip_table
CREATE INDEX hpx_hash_hip_idx ON hip_table (hpx_hash(29, raicrs, deicrs));
CREATE INDEX hpx_hash_range_hip_idx ON hip_table USING GIST(hpx_hash_range(29, raicrs, deicrs));

-- Creation of the table c1239hip_main
CREATE TABLE c1239hip_main (
    recno integer NOT NULL,
    hip integer NOT NULL,
    proxy character(1) NOT NULL,
    rahms character(11) NOT NULL,
    dedms character(11) NOT NULL,
    vmag float,
    varflag float,
    r_vmag character(1) NOT NULL,
    raicrs double precision,
    deicrs double precision,
    astroref character(1) NOT NULL,
    plx double precision,
    pmra double precision,
    pmde double precision,
    e_raicrs double precision,
    e_deicrs double precision,
    e_plx double precision,
    e_pmra double precision,
    e_pmde double precision,
    de_ra double precision,
    plx_ra double precision,
    plx_de double precision,
    pmra_ra double precision,
    pmra_de double precision,
    pmra_plx double precision,
    pmde_ra double precision,
    pmde_de double precision,
    pmde_plx double precision,
    pmde_pmra double precision,
    f1 float,
    f2 float,
    btmag double precision,
    e_btmag double precision,
    vtmag double precision,
    e_vtmag double precision,
    m_btmag character(1) NOT NULL,
    b_v double precision,
    e_b_v double precision,
    r_b_v character(1) NOT NULL,
    v_i double precision,
    e_v_i double precision,
    r_v_i character(1) NOT NULL,
    combmag character(1) NOT NULL,
    hpmag double precision,
    e_hpmag double precision,
    hpscat double precision,
    o_hpmag smallint,
    m_hpmag character(1) NOT NULL,
    hpmax float,
    hpmin float,
    period double precision,
    hvartype character(1) NOT NULL,
    morevar character(1) NOT NULL,
    morephoto character(1) NOT NULL,
    ccdm character(10) NOT NULL,
    n_ccdm character(1) NOT NULL,
    nsys smallint,
    ncomp smallint,
    multflag character(1) NOT NULL,
    source_1 character(1) NOT NULL,
    qual character(1) NOT NULL,
    m_hip character(2) NOT NULL,
    theta double precision,
    rho double precision,
    e_rho double precision,
    dhp float,
    e_dhp float,
    survey character(1) NOT NULL,
    chart character(1) NOT NULL,
    notes character(1) NOT NULL,
    hd integer,
    bd character(10) NOT NULL,
    cod character(10) NOT NULL,
    cpd character(10) NOT NULL,
    _v_i_red float NOT NULL,
    sptype character varying(12) NOT NULL,
    r_sptype character(1) NOT NULL,
    _raj double precision,
    _dej double precision
);

-- Copy of the data from the csv to the table
COPY c1239hip_main FROM '/home/mlapointe/Documents/Ressources/csv/c1239hip_main.csv' (FORMAT csv, DELIMITER ',', NULL '', HEADER false);

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
CREATE INDEX hpx_hash_tyc2_idx ON tyc2 (hpx_hash(29, ra_icrs_, de_icrs_));

-- FUNCTIONS

-- Recuperation of a MOC from a row of moc_table
CREATE FUNCTION moc_from_moc_table(idx integer) RETURNS rangemocpsql AS
    '
    SELECT create_range_moc_psql(
        (SELECT depth_max FROM moc_table WHERE id = idx),
        (SELECT array_agg(r) FROM moc_table, LATERAL unnest(ranges) AS r WHERE id = idx)
    );
    '
LANGUAGE SQL;

-- int8range[] -> int8multirange
CREATE OR REPLACE FUNCTION to_int8multirange(r int8range[])
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
CREATE OR REPLACE FUNCTION to_ranges_moc_psql(moc rangemocpsql)
RETURNS int8multirange AS $$
DECLARE
    r int8range[];
    result int8multirange;
BEGIN
    r := moc_to_ranges(moc);
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
CREATE OR REPLACE FUNCTION to_ranges_bmoc_psql(bmoc BMOCpsql)
RETURNS int8multirange AS $$
DECLARE 
    r int8range[];
    result int8multirange;
BEGIN
    r := hpx_to_ranges(bmoc);
    EXECUTE format(
        'SELECT ''{%s}''::int8multirange',
        array_to_string(r, ',')
    )
    INTO result;

    RETURN result;
END;
$$ LANGUAGE plpgsql IMMUTABLE STRICT;

-- Equivalent of is_in_cone()
-- Doesn't use the index
CREATE OR REPLACE FUNCTION is_in_cone_bool(
    depth integer,
    lon double precision,
    lat double precision,
    radius double precision
) RETURNS boolean
AS $$
    SELECT EXISTS (
        SELECT 1
        FROM hip_table h
        WHERE
            (hpx_hash_range(29, h.raicrs, h.deicrs) <@ to_int8multirange(hpx_flag_one(hpx_cone_coverage_approx(depth, lon, lat, radius))))
        OR (
            (hpx_hash_range(29, h.raicrs, h.deicrs) <@ to_int8multirange(hpx_flag_zero(hpx_cone_coverage_approx(depth, lon, lat, radius))))
            AND hpx_contains_bool(hpx_cone_coverage_approx(depth, lon, lat, radius), h.raicrs, h.deicrs)
        )
    )
$$ LANGUAGE sql immutable;

-- Returns the set of bmocs that satisfy is_in_cone_bool()
-- Uses the index
CREATE OR REPLACE FUNCTION is_in_cone_psql(depth integer, lon double precision, lat double precision, radius double precision) RETURNS TABLE(hip bigint, vmag double precision, raicrs double precision, deicrs double precision) AS
    $$
    SELECT * FROM hip_table h
    WHERE
        (hpx_hash_range(29, h.raicrs, h.deicrs) <@ to_int8multirange(hpx_flag_one(hpx_cone_coverage_approx(depth, lon, lat, radius))))
    OR
        ((hpx_hash_range(29, h.raicrs, h.deicrs) <@ to_int8multirange(hpx_flag_zero(hpx_cone_coverage_approx(depth, lon, lat, radius))))
        AND
        (hpx_contains_bool(hpx_cone_coverage_approx(depth, lon, lat, radius), h.raicrs, h.deicrs)));
    $$
LANGUAGE sql immutable;

-- Query that replaces is_in_cone
SELECT * FROM hip_table WHERE (hpx_hash_range(29, raicrs, deicrs) <@ to_int8multirange(hpx_flag_one(hpx_cone_coverage_approx(6, 13.158329, -72.80028, 5.64323))))
OR ((hpx_hash_range(29, raicrs, deicrs) <@ to_int8multirange(hpx_flag_zero(hpx_cone_coverage_approx(6, 13.158329, -72.80028, 5.64323))))
AND (hpx_contains_bool(hpx_cone_coverage_approx(6, 13.158329, -72.80028, 5.64323),raicrs, deicrs)));

-- Idea of the query that return the bmoc post-filtered
-- SELECT * FROM hip_table WHERE hpx_hash_range(29, raicrs, deicrs) <@ to_int8multirange(hpx_flag_one(create_bmoc_psql(29, ARRAY[8202, 8203, 8206, 8207, 8218, 8224, 8225]))))
--   | (hpx_hash_range(29, raicrs, deicrs) <@ to_int8multirange(hpx_flag_zero(create_bmoc_psql(29, ARRAY[8202, 8203, 8206, 8207, 8218, 8224, 8225])))
--   & (SELECT * FROM hip_table WHERE hpx_contains_bool(hpx_cone_coverage_approx(6, 13.158329, -72.80028, 5.64323),raicrs, deicrs)));
