# Tutorial to install the extension

This tutorial explains how to download this Postgres extension on your own machine **without downloading Rust and PGRX**.

Remark : For now, the extension is only runable on Linux distributions.
For more info about cross-compiling PGRX, please consult this [link](https://github.com/pgcentralfoundation/pgrx/blob/develop/docs/src/extension/build/cross-compile.md).

## Get started

- **Download PostgreSQL v17.5** : `sudo apt install postgresql-17`

- **Download the zip and extract the 3 following files** :
    + in `/lib` : `healpix_pgrx.so`
    + in `/share` : 
        + `healpix_pgrx.control`
        + `healpix_pgrx--0.0.1.sql`
    
- **Copy the 3 files in your PostgreSQL directories to add the extension** :
    + `sudo cp Downloads/cds_healpix_pgrx/lib/postgresql/17/lib/healpix_pgrx.so /usr/lib/postgresql/17/lib/healpix_pgrx.so`
    + `sudo cp Downloads/cds_healpix_pgrx/share/postgresql/17/extension/healpix_pgrx.control /usr/share/postgresql/17/extension/healpix_pgrx.control`
    + `sudo cp Downloads/cds_healpix_pgrx/share/postgresql/17/extension/healpix_pgrx--0.0.1.sql /usr/share/postgresql/17/extension/healpix_pgrx--0.0.1.sql`

- You may need to grant the permissions to the files for psql to use them

## Ready for use

- **Launch `psql`** :
    + `sudo -i -u postgres`
    + `psql`

- **Create the extension** : `CREATE EXTENSION healpix_pgrx;`

Now you can start working with the extension !
    
