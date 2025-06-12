# PGRX first test

This repository was created to test PGRX, a framework created to code PostgreSQL extensions in Rust (https://github.com/pgcentralfoundation/pgrx/tree/develop).   

By using this project, you will be able to call a Rust function from PostgreSQL.   

The function comes from the CDS's HEALPix repository : https://github.com/cds-astro/cds-healpix-rust/blob/ac8a3d2dd5d0e37cb1a49ea8d55ddd680e73c182/src/nested/mod.rs#L235-L237.

## Installation

- **PGRX setup**  

  + Download the system requirements listed on this page : https://github.com/pgcentralfoundation/pgrx/blob/develop/README.md#system-requirements
  
  + `cargo pgrx init` : initializes PGRX the first time you are using it
 
- **HEALPix setup**

  + Download the CDS's HEALPix GitHub repository : https://github.com/cds-astro/cds-healpix-rust.git
    
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

  + `healpix_pgrx_test=# CREATE EXTENSION healpix_pgrx_test;` : creates the extension corresponding to my repository
  + `healpix_pgrx_test=# SELECT hash(<arg1>, <arg2>, <arg3>);` : calls the Rust function from HEALPix

![Clipboard - 12 juin 2025 11_32](https://github.com/user-attachments/assets/96dd26cc-0666-49f3-8b9c-bb9a5317a6e8)

As you can see on the screenshot, the call to the function returns 19456, which is the right result.

