# PostgreSQL extension for HEALPix in Rust

This repository is based on the use of [PGRX](https://github.com/pgcentralfoundation/pgrx/tree/develop), a framework created to develop PostgreSQL extensions in Rust .   

By using this project, you will be able to call Rust functions from PostgreSQL.

The functions come from [the CDS HEALPix repository](https://github.com/cds-astro/cds-healpix-rust.git).

## Installation

- **PGRX setup**  

  + Download the [system requirements](https://github.com/pgcentralfoundation/pgrx/blob/develop/README.md#system-requirements)

  + `cargo install --locked cargo-pgrx` : downloads PGRX
  
  + `cargo pgrx init` : initializes PGRX the first time you are using it
 
- **HEALPix setup**

  + Download [the CDS HEALPix GitHub repository](https://github.com/cds-astro/cds-healpix-rust.git)
    
  + In the `healpix_pgrx_test/Cargo.toml` file, make sure you put the right path to the HEALPix library :  

  ```rust
  [dependencies]  
  pgrx = "=0.14.3"  
  cdshealpix = { path = "/your_path" }
  ```

- **Getting started**

  + `cargo pgrx run` : runs your code  
    NB : It may take a little bit of time if it is your first time running code with `pgrx`.

- **PostgreSQL queries**

  Once you entered the PostgreSQL interface, you can use your Rust code through PostgreSQL queries.

  + `healpix_pgrx_test=# DROP EXTENSION healpix_pgrx_test;` : **if you already created an extension called `healpix_pgrx_test`**, you have to manually drop and recreate it for Postgres to consider the latest updates of your code
  + `healpix_pgrx_test=# CREATE EXTENSION healpix_pgrx_test;` : creates the extension corresponding to my repository
  + `healpix_pgrx_test=# SELECT hash(<arg1>, <arg2>, <arg3>);` : as an example, let's call the Rust function `hash` from HEALPix

![Console display](https://github.com/user-attachments/assets/96dd26cc-0666-49f3-8b9c-bb9a5317a6e8)

As you can see on the screenshot, the call to the function returns 19456, which is the right result.

**Author :** Marjorie Lapointe  
**Language :** Rust   
**Last version :** July 2025
