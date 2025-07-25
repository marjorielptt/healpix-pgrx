# PostgreSQL extension for HEALPix in Rust

By using this project, you will be able to call [HEALPix](https://github.com/cds-astro/cds-healpix-rust.git) and [MOC](https://github.com/cds-astro/cds-moc-rust/tree/31e69f6e85c02043576740839f16f1d1f7e1ac77) Rust functions from PostgreSQL.

This repository is based on the use of [PGRX](https://github.com/pgcentralfoundation/pgrx/tree/develop), a framework created to develop PostgreSQL extensions in Rust .   

## Installation without Rust and PGRX

If you want to use the extension without installing Rust and PGRX, please consult the [healpix-pgrx-test/install](https://gitlab.cds.unistra.fr/mlapointe/healpix-pgrx-test/-/tree/main/install) folder of this repository.

## Installation with Rust and PGRX

If you want to use the extension and install Rust and PGRX (to modify it for example), please read the following instructions.

- **PGRX setup**  

  + Download the [system requirements](https://github.com/pgcentralfoundation/pgrx/blob/develop/README.md#system-requirements)

  + `cargo install --locked cargo-pgrx` : downloads PGRX
  
  + `cargo pgrx init` : initializes PGRX the first time you are using it
 
- **HEALPix setup**

  + Download [the CDS HEALPix GitHub repository](https://github.com/cds-astro/cds-healpix-rust.git)
    
  + In the `healpix-pgrx/Cargo.toml` file, make sure the `[dependencies]` and `[features]` sections are configurated as below :  

  ```rust
  [dependencies]  
  pgrx = "=0.15.0" 
  cdshealpix = { git = "https://github.com/cds-astro/cds-healpix-rust.git" }
  ```
  ```rust
  [features]
  default = ["pg17"]
  ```
- **Getting started**

  + `cargo pgrx run` : runs your code  
    NB : It may take a little bit of time if it is your first time running code with `pgrx`.

- **PostgreSQL queries**

  Once you entered the PostgreSQL interface, you can use your Rust code through PostgreSQL queries.

  + `healpix_pgrx=# DROP EXTENSION healpix_pgrx;` : **if you already created an extension called `healpix_pgrx`**, you have to manually drop and recreate it for Postgres to consider the latest updates of your code
  + `healpix_pgrx=# CREATE EXTENSION healpix_pgrx;` : creates the extension corresponding to my repository
  + `healpix_pgrx=# SELECT hpx_hash(<arg1>, <arg2>, <arg3>);` : as an example, let's call the Rust function `hash` from HEALPix

![Console display](images/minimal_demo_of_the_extension.png)

As you can see on the screenshot, the call to the function returns 19456, which is the right result.

**Author :** Marjorie Lapointe  
**Language :** Rust   
**Last version :** July 2025
