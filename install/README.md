# Tutorial to install mogipix

This tutorial explains how to download this Postgres extension on your own machine **without downloading Rust and PGRX**.

Remark : For now, the extension is only runable on Debian based distributions.
For more info about cross-compiling PGRX, please consult this [link](https://github.com/pgcentralfoundation/pgrx/blob/develop/docs/src/extension/build/cross-compile.md).

## Makefile instructions

- Download the directory [mogipix/install](https://gitlab.com/marjorielptt/mogipix/-/tree/main/install?ref_type=heads)

- Open a terminal and go in the /install folder

- If you have another version than 17 for Postgres, please modify the `PG_VERSION` variable in the makefile

- Use the makefile to install the extension : `make install`

NB : You can also run `make clean` and `make show`.

## Ready for use

- **Launch `psql`** :
    + `sudo -i -u postgres`
    + `psql`

- **Create the extension** : **If you already created an extension called `mogipix`**, you have to manually drop and recreate it for Postgres to consider the latest updates of the extension : `mogipix=# DROP EXTENSION mogipix;` and then `mogipix=# CREATE EXTENSION mogipix;`

- Execute the whole [setup.sql](https://gitlab.com/marjorielptt/mogipix/-/blob/main/pg_regress/sql/setup.sql?ref_type=heads) file to have the needed indexes/functions.

Now you can start working with the extension !

## Query examples

  Once you entered the PostgreSQL interface, you can use your Rust code through PostgreSQL queries.<br/>
  As an example, let's try some queries :

  - `mogipix=# SELECT mgx_hash(<arg1>, <arg2>, <arg3>);`<br/>
  Returns the hash corresponding to the bmoc parameters you entered.

  - `mogipix=# CREATE INDEX mgx_hash_hip_idx ON hip_table (mgx_hash(29, raicrs, deicrs));`<br/>
  Creates an index on the hash of the BMOCs
  - `mogipix=# SELECT mgx_create_range_moc_psql(29, ARRAY[int8range(100,200),int8range(300,400)]);`<br/>
  Creates a RangeMOC at depth 29 which contains the cells contained between 100 and 200 and between 300 and 400.
  - `mogipix=# SELECT mgx_moc_to_ascii(mgx_create_range_moc_psql(29, ARRAY[int8range(100,200),int8range(300,400)]));`<br/>
  Returns the ASCII representation corresponding to the RangeMOC given in parameter.
  - `mogipix=# SELECT mgx_create_bmoc_psql(13, ARRAY[8202, 8203, 8206, 8218]) & mgx_create_bmoc_psql(13, ARRAY[8202, 8203, 8224, 8225]) AS intersection;`<br/>
  Creates a BMOC out of the intersection between 2 BMOCs
  - `mogipix=# SELECT * FROM hip_table WHERE mgx_hash_range(29, h.raicrs, h.deicrs) <@ mgx_to_int8multirange(mgx_bmoc_flag_zero(mgx_bmoc_cone_coverage_approx(mgx_best_starting_depth(5.64323)+4, 0.01814144, 3.94648893, 5.64323)));`<br/>
  Returns the cells on the border of the cone created with the parameters lon=0.01814144, lat=3.94648893 and radius=5.64323
  - `mogipix=# SELECT * FROM hip_table WHERE mgx_in_cone(0.01814144, 3.94648893, 5.64323, raicrs, deicrs);`<br/>
  Returns the cells forming a precise cone 
  - For more examples, please consult [tests.sql](https://gitlab.com/marjorielptt/mogipix/-/blob/main/pg_regress/sql/tests.sql?ref_type=heads)  


**Author :** Marjorie Lapointe  
**Language :** Rust   
**Last version :** July 2025