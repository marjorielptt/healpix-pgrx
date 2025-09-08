-- TESTS

-- Creation of a MOC
SELECT mgx_create_range_moc_psql(29, ARRAY[int8range(100,200),int8range(300,400)]);

-- Function test : mgx_moc_from_ascii_ivoa
SELECT mgx_moc_to_ascii(mgx_create_range_moc_psql(29, ARRAY[int8range(100,200),int8range(300,400)]));

-- Function test : mgx_moc_from_ascii_ivoa

-- Function test : mgx_moc_and
SELECT mgx_create_range_moc_psql(29, ARRAY[int8range(100,200),int8range(300,400)]) & mgx_create_range_moc_psql(28, ARRAY[int8range(350,600)]);

-- Creation of a BMOC
SELECT mgx_create_bmoc_psql(13, ARRAY[8202, 8203, 8206, 8218]) & mgx_create_bmoc_psql(13, ARRAY[8202, 8203, 8224, 8225]) AS intersection;

-- Function test : mgx_bmoc_elliptical_cone
SELECT mgx_bmoc_elliptical_cone_coverage(3, 36.80105218, 56.78028536, 14.93, 4.93, 75.0);

-- PROBLEMATIC QUERY FOR MOCs : doesn't use the bitmap index scan
-- SELECT * FROM hip_table WHERE mgx_bmoc_hash(29, raicrs, deicrs) <@ int8multirange(int8range(100,200), int8range(300,400));

-- SOLUTION : we create a range that only contains the element we want to index the table with
CREATE INDEX mgx_bmoc_hash_hip_idx ON hip_table USING GIST(mgx_bmoc_hash_range(29,raicrs,deicrs));

-- Then this query uses the index
SELECT * FROM hip_table WHERE mgx_hash_range(29, raicrs, deicrs) <@ int8multirange('[100, 200)', '[300,400)');

-- Same query but with a function that returns the ranges of a moc
SELECT * FROM hip_table WHERE mgx_hash_range(29, raicrs, deicrs) <@ mgx_moc_to_ranges(mgx_create_range_moc_psql(29, ARRAY[int8range(100,200),int8range(300,400)]));

-- Create a MOC from a cone
SELECT moc_from_cone(13.158329, -72.80028, 5.64323, 6, 5, 'All');

-- Return the cells contained in the moc created from a cone
SELECT * FROM hip_table WHERE hpx_hash_range(29, raicrs, deicrs) <@ to_ranges_moc_psql(moc_from_cone(13.158329, -72.80028, 5.64323, 6, 5, 'All'));

-- Return the cells contained in the bmoc created from a cone
select * FROM hip_table WHERE hpx_hash_range(29, raicrs, deicrs) <@ to_ranges_bmoc_psql(hpx_cone_coverage_approx(29, 13.158329, -72.80028, 5.64323));

-- Returns the cells COMPLETELY contained in the cone (flag=1)
SELECT * FROM hip_table WHERE hpx_hash_range(29, raicrs, deicrs) <@ to_int8multirange(hpx_flag_one(create_bmoc_psql(29, ARRAY[8202, 8203, 8206, 8207, 8218, 8224, 8225])));

-- Returns the cells PARTIALLY contained in the cone (flag=0)
SELECT * FROM hip_table WHERE hpx_hash_range(29, raicrs, deicrs) <@ to_int8multirange(hpx_flag_zero(create_bmoc_psql(29, ARRAY[8202, 8203, 8206, 8207, 8218, 8224, 8225])));

-- First test for is_in_cone_psql(...)
SELECT * FROM is_in_cone_psql(
    0.01814144, 3.94648893, 5.64323,
    'hip_table', 29, 'raicrs', 'deicrs')
AS t("HIP" bigint, "Vmag" double precision, "RAICRS" double precision, "DEICRS" double precision);

-- Second test for is_in_cone_psql(...)
SELECT * FROM is_in_cone_psql(
    0.01814144, 3.94648893, 5.64323,
    'tyc2', 29, 'ra_icrs_', 'de_icrs_')
AS t("recno" integer,
    "tyc1" smallint,
    "tyc2" smallint,
    "tyc3" smallint,
    "pflag" character(1),
    "ramdeg" double precision,
    "demdeg" double precision,
    "pmra" double precision,
    "pmde" double precision,
    "e_ramdeg" float,
    "e_demdeg" float,
    "e_pmra" float,
    "e_pmde" float,
    "epram" double precision,
    "epdem" double precision,
    "num" smallint,
    "q_ramdeg" float,
    "q_demdeg" float,
    "q_pmra" float,
    "q_pmde" float,
    "btmag" float,
    "e_btmag" float,
    "vtmag" float,
    "e_vtmag" float,
    "prox" smallint,
    "tyc" character(1),
    "hip" integer,
    "ccdm" character(3),
    "ra_icrs_" double precision,
    "de_icrs_" double precision,
    "epra_1990" float,
    "epde_1990" float,
    "e_radeg" float,
    "e_dedeg" float,
    "posflg" character(1),
    "corr" float);

-- Test function in which we declare a bmoc that is not in the parameters
create or replace function test_declare() returns text as $$
declare
    bmoc bmocpsql;
BEGIN
    bmoc:=create_bmoc_psql(13, ARRAY[8202, 8203, 8206, 8218]);
    execute(SELECT * FROM hip_table h
    WHERE
        (hpx_hash_range(29, h.raicrs, h.deicrs) <@ to_int8multirange(hpx_flag_one(hpx_cone_coverage_approx(hpx_best_starting_depth(radius)+4, lon, lat, radius))))
    OR
        ((hpx_hash_range(29, h.raicrs, h.deicrs) <@ to_int8multirange(hpx_flag_zero(hpx_cone_coverage_approx(hpx_best_starting_depth(radius)+4, lon, lat, radius))))
        AND
        (hpx_contains_bool(hpx_cone_coverage_approx(hpx_best_starting_depth(radius)+4, lon, lat, radius), h.raicrs, h.deicrs)))
    ) as res;
    return res;
end;
$$ language plpgsql immutable
;

-- Cone contains
SELECT * FROM hip_table WHERE in_cone(0.01814144, 3.94648893, 5.64323, raicrs, deicrs);

-- Elliptical cone contains
SELECT * FROM hip_table WHERE in_elliptical_cone(0.01814144, 3.94648893, 1.6433, 4, 1.6, raicrs, deicrs);

-- Zone contains
SELECT * FROM hip_table WHERE in_zone(0.01814144, 3.94648893, 2.57489321, 5.38601839, raicrs, deicrs);

-- Polygon contains
SELECT * FROM hip_table WHERE in_polygon(
    ARRAY[create_vertexpsql(0.01814144, 3.94648893), create_vertexpsql(2.57489321, 5.38601839), create_vertexpsql(7.57489321, 3.38601839)],
    false,  -- exact_solution
    false,  -- complement 
    raicrs,
    deicrs);

-- Box contains
SELECT * FROM hip_table WHERE in_box(0.01814144, 3.94648893, 4, 1.6433, 1.6, raicrs, deicrs);

-- Ring contains
SELECT * FROM hip_table WHERE in_ring(0.01814144, 3.94648893, 2.57489321, 5.38601839, raicrs, deicrs);