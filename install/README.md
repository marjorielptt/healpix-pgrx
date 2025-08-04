# Tutorial to install the extension

This tutorial explains how to download this Postgres extension on your own machine **without downloading Rust and PGRX**.

Remark : For now, the extension is only runable on Debian based distributions.
For more info about cross-compiling PGRX, please consult this [link](https://github.com/pgcentralfoundation/pgrx/blob/develop/docs/src/extension/build/cross-compile.md).

## Makefile instructions

- Download the directory `healpix-pgrx/install`

- Open a terminal and go in the `/install` folder

- If you have another version than 17 for Postgres, please modify the `PG_VERSION` variable in the makefile

- Use the makefile to install the extension : `make install`

NB : You can also run `make clean` and `make show`.

## Ready for use

- **Launch `psql`** :
    + `sudo -i -u postgres`
    + `psql`

- **Create the extension** : `CREATE EXTENSION healpix_pgrx;`

- Create the different indexes/functions that are in `pg_regress/sql/setup.sql`.

Now you can start working with the extension !