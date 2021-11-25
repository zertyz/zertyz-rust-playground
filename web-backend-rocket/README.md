# Production-ready & standalone Rocket HTTP server with Rust + Angular (Universal)

Demonstrates the state-of-the-art on loading speeds for Angular sites (100ms to show content) and how to obtain it with
Rust & Angular Universal embedded into a Rocket server.

  * Angular Universal is used to prerender static routes (the ones not receiving URL parameters) -- so the page is visible
    before loading any JavaScripts, which is optimal for performance and SEO

  * All static files are ahead-of-time compressed in 'gzip -9' (default) or 'brotli -q 11 -w 24'

  * The Angular client in this demo has a backend explorer frontend ready to use with Material Design and Bootstrap themes;

  * The backend automatically starts the angular dev serving scripts, making a good development pipeline -- even from the IDE

  * When compiled for release with ```RUSTFLAGS="-C target-cpu=native" cargo build --release```, ```build.rs``` runs
    ```ng build --aot --build-optimizer --optimization --prod --progress``` or ```npm run prerender```, then the generated files
    are compressed and incorporated into the Rocket server, along with all the backend services -- so the stand-alone executable
    also serves the UI and no external web servers are needed

  * This project may be considered a Rust + Angular Universal + Rocket seed.


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

To build the Kubernetes-friendly standalone server with default features:
```
    $ cd ..
    $ RUSTFLAGS="-C target-cpu=native" cargo build --release
```

Inspect ```build.rs``` for embedding options.


# Screenshots

## Bootstrap Theme
![rust+angular+bootstrap 1.png](screenshots/rust+angular+bootstrap%201.png)
![rust+angular+bootstrap 2.png](screenshots/rust+angular+bootstrap%202.png)
![rust+angular+bootstrap 3.png](screenshots/rust+angular+bootstrap%203.png)
(186ms to load & render what is needed to present -- bootstrap themes still requires loading & executing bootstrap.js)

## Google Material Theme
![rust+angular+material 1.png](screenshots/rust+angular+material%201.png)
![rust+angular+material 2.png](screenshots/rust+angular+material%202.png)
(only 44ms needed to show the content -- 13ms to load index.html + 31ms to render it. After being presented, Angular is loaded and after 664ms we have a fully working website)

# Pingdom measurements
![rust+angular+bootstrap+Pingdom+results.png](screenshots/rust+angular+bootstrap+Pingdom+results.png)
(measurements for Angular + Bootstrap)
