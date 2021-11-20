Demonstrates how to work with Angular & Rocket.

  * The Angular client in this demo has a backend explorer frontend ready to use with Material Design and Bootstrap themes;

  * The backend automatically starts the angular dev serving scripts, making a good development pipeline -- even from the IDE

  * When compiled for release 'RUSTFLAGS="-C target-cpu=native" cargo build --release', build.rs runs
    'ng build --aot --build-optimizer --optimization --prod --progress', then the generated files are -9 gzipped and incorporated
    into the Rocket server, along with all the backend services -- so the stand-alone executable also serves the UI and no
    external web servers are needed.