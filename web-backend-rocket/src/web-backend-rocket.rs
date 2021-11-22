mod static_files;

use rocket::{
    Request,
    Response,
    response::{self, Responder, status, Builder},
    http::{
        Header,
        Cookie,
        ContentType,
        Status,
    },
    FromFormField,
    FromForm,
};
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};

use std::{
    path::{Path,PathBuf},
    process::Command,
    io::Cursor,
};
use std::borrow::Cow;
use rocket::form::Form;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate rocket;

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
struct RawJson {
    json: String
}

struct InternalFile {
    file_name: String,
}
impl<'r> Responder<'r, 'r> for InternalFile {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'r> {
        let file_name = self.file_name;
        let (compressed, file_contents) = match static_files::STATIC_FILES.get(file_name.as_str()) {
            Some(tuple) => tuple,
            None => return Result::Err(Status{code:404}),
        };
        let file_extension = match file_name.rsplit_once(".") {
            Some((file_name_before_last_dot, file_extension)) => file_extension,
            None => "html",
        };
        let mut response_builder = Response::build();
        response_builder.header(ContentType::from_extension(file_extension).unwrap());
        if *compressed {
            // informs the client that we have everything gzipped
            response_builder.raw_header("Content-Encoding", "gzip");
        }
        response_builder
            // enforce caching on the client
            .raw_header("Cache-Control", static_files::CACHE_CONTROL)
            .raw_header("expires", static_files::EXPIRATION_DATE)
            .raw_header("last-modified", static_files::GENERATION_DATE)
            .sized_body(file_contents.len(), Cursor::new(file_contents))
            .ok()
    }
}

#[get("/rest-service/<world>")]
fn rest_service(world: &str) -> RawJson {
    RawJson { json: format!(r#"{{"msg":"Hello, world of {}!"}}"#, world) }
}

#[get("/get-service?<from_temperature>&<from_length>&<conversion>")]
fn get_service(from_temperature: f64, from_length: f64, conversion: Conversions) -> RawJson {
    let (from_temperature_unit, from_length_unit,
         to_temperature, to_length,
         to_temperature_unit, to_length_unit) = match conversion {
        Conversions::MetricToImperial => ("°C", "m",  (from_temperature * 9.0/5.0) + 32.0, from_length * 3.2808398950132, "°F", "ft"),
        Conversions::ImperialToMetric => ("°F", "ft", (from_temperature - 32.0) * 5.0/9.0, from_length / 3.2808398950132, "°C", "m")
    };
    RawJson { json: format!("{{\
                                \"from_temperature\": \"{:.2}{}\",
                                \"from_length\":      \"{:.2}{}\",
                                \"to_temperature\":   \"{:.2}{}\",
                                \"to_length\":        \"{:.2}{}\"
                            }}",
                            from_temperature, from_temperature_unit,
                            from_length,      from_length_unit,
                            to_temperature,   to_temperature_unit,
                            to_length,        to_length_unit) }
}
#[derive(Debug,PartialEq,FromFormField)]
enum Conversions {
    MetricToImperial,
    ImperialToMetric,
}

#[post("/post-service", format = "json", data = "<shipping_info_json>")]
fn post_service(shipping_info_json: Json<ShippingInfo>) -> Json<ShippingInfo> {
    let shipping_info = shipping_info_json.into_inner();
    Json(shipping_info)
}
#[derive(Debug, PartialEq, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct ShippingInfo {
    company:          String,
    first_name:       String,
    last_name:        String,
    address:          String,
    city:             String,
    state:            String,
    postal_code:      u32,
    shipping:         String,
    refuse_housemate: bool,
}

/// serves statically linked files for blazing-fast speeds (no context switches)
#[get("/<file..>")]
fn internal_files(file: PathBuf) -> InternalFile {
    let internal_file_name = format!("/{}", file.to_string_lossy().to_string());
    InternalFile {file_name: internal_file_name}
}


#[launch]
fn rocket() -> _ {

    #[cfg(debug_assertions)]
    start_ng_dev_server();

    #[cfg(not(debug_assertions))]
    include_static_production_angular_files();

    rocket::build().mount("/", routes![internal_files, rest_service, get_service, post_service])
}

/// runs the angular compile & serve scripts to serve the static angular files -- note
/// client files (4200) & backend (8000) services will be on different ports
/// (make sure the app runs from the project's root folder)
fn start_ng_dev_server() {
    let message = "Starting DEV Angular server";
    let command = format!("echo '{}'; cd angular && npm start", message);
    let shell;
    if cfg!(target_os = "windows") {
        shell = "cmd";
    } else {
        shell = "sh";
    };
    let _child = Command::new(shell)
        .args(["-c", &command])
        .spawn()
        .expect("Failed to start Angular UI Application!");
}

//noinspection RsMainFunctionNotFound
/// instructs Rocket to serve Angular's static files (as generated by build.rs) -- allowing
/// UI and backend to be served together
fn include_static_production_angular_files() {
    eprintln!("Serving Angular UI generated at {}", static_files::GENERATION_DATE);
}