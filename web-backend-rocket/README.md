# Standalone Rust + Angular + Rocket HTTP server

Demonstrates how to work with Angular & Rocket.

  * The Angular client in this demo has a backend explorer frontend ready to use with Material Design and Bootstrap themes;

  * The backend automatically starts the angular dev serving scripts, making a good development pipeline -- even from the IDE

  * When compiled for release 'RUSTFLAGS="-C target-cpu=native" cargo build --release', build.rs runs
    'ng build --aot --build-optimizer --optimization --prod --progress', then the generated files are compressed and incorporated
    into the Rocket server, along with all the backend services -- so the stand-alone executable also serves the UI and no
    external web servers are needed.

  * Static files are ahead-of-time compressed in 'gzip -9' (default) or 'brotli -q 11 -w 24'


## Setup

You'll need Angular CLI 12.2.0+ installed. Then run (only tested on Linux & Mac):

```
    $ cd angular
    $ npm install
    $ ./set_theme Material # or ./set_theme Renovat
```

Test the client setup with:
```
    $ ng serve
```

To build the standalone application:
```
    $ cd ..
    $ RUSTFLAGS="-C target-cpu=native" cargo build --release
```


# Screenshots

## Bootstrap Theme
![rust+angular+bootstrap 1.png](screenshots/rust+angular+bootstrap%201.png)
![rust+angular+bootstrap 2.png](screenshots/rust+angular+bootstrap%202.png)
![rust+angular+bootstrap 3.png](screenshots/rust+angular+bootstrap%203.png)

## Google Material Theme
![rust+angular+material 1.png](screenshots/rust+angular+material%201.png)
![rust+angular+material 2.png](screenshots/rust+angular+material%202.png)
