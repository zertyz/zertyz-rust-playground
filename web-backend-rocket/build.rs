// build.rs: keeps static files to be served inside the main executable, allowing for blazing-fast service (without context switches nor cache manipulations)

use std::{
    env,
    fs,
    path::{Path,PathBuf},
    io::{Write,BufWriter},
    process::Command,
    collections::HashMap,
    time::{SystemTime,Duration},
    ops::Add,
};
use walkdir::WalkDir;
use chrono::{DateTime, Utc};

// ---------------------------------- CONFIGURATION START ----------------------------------

/// the Angular app name, as created in 'dist/' when building the app
const ANGULAR_APP_NAME: &str = "DemoAngularAndBootstrapUI";

/// should we build a regular Angular site or a blazing fast pre-rendered, universal one?
const ANGULAR_BUILD_TYPE: AngularBuildTypes = AngularBuildTypes::PreRenderedUniversal;

/// which compressor to use to serve the static files
const COMPRESSOR: Compressors = Compressors::GZip;

// ----------------------------------- CONFIGURATION END -----------------------------------

/// how smaller (in bytes) the compressed file must be, in comparison to the plain version, for us to serve it in the compressed form
const COMPRESSION_THRESHOLD: usize = 100;

/// builds a production-ready website using regular Angular scripts
const REGULAR_ANGULAR_BUILD_COMMAND: &str = "ng build --aot --build-optimizer --optimization --prod --progress";

/// builds a production-ready website + dynamic server (which is not used by us, for we only care for URL parameter-less routes)
/// -- look at 'angular.json' for which routes will participate on the pre-rendering
const PRE_RENDERED_UNIVERSAL_BUILD_COMMAND: &str = "npm run prerender";

/// selects between Vanilla or Angular Universal builds
#[derive(Debug)]
enum AngularBuildTypes {
    /// Universal build with pre-rendered static pages -- HTML already contains the final DOM, so
    /// the page is presented even before any JavaScripts are loaded
    PreRenderedUniversal,
    /// regular vanilla Angular build -- DOM rendered after JavaScripts are loaded
    Regular,
}

/// selects between compressors to use for static files
#[derive(Debug)]
enum Compressors {
    /// must be supported by all browsers
    GZip,
    /// offers ~15% better compression ratios for text, when compared to gzip -- not accepted by Firefox 94.0.1 (2021, nov, 24) when accessing via HTTP
    Brotli,
}

fn main() {

    eprintln!("Running custom build.rs:");

    #[cfg(debug_assertions)]
    on_non_release();

    #[cfg(not(debug_assertions))]
    on_release();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=angular/src");
}

fn on_non_release() {
    eprintln!("\t(nothing to run, since we're not compiling for Release)");
    save_static_files(
        HashMap::from([
            ("/index.html".to_string(), Vec::from("RUNNING IN NON-RELEASE MODE (redirects to localhost:4200/)".as_bytes())),
        ]),
        HashMap::from([
            ("/".to_string(), "/index.html".to_string())
        ])
    );
}

/// builds the angular site for production, then loads (and compresses) the resulting static files, storing them in a hash map for use by the application.
fn on_release() {
    let angular_relative_path = "./angular";
    let angular_dist_path = format!("{}/dist/{}/browser", angular_relative_path, ANGULAR_APP_NAME);
    let angular_build_command = match ANGULAR_BUILD_TYPE {
        AngularBuildTypes::PreRenderedUniversal => PRE_RENDERED_UNIVERSAL_BUILD_COMMAND,
        AngularBuildTypes::Regular =>              REGULAR_ANGULAR_BUILD_COMMAND,
    };
    let full_build_command = format!("cd '{}' && {}", angular_relative_path, angular_build_command);
    let get_angular_routes_command = format!(r#"grep "{{ path: '" {}/src/app/app-routing.module.ts | sed "s|.* path: '\([^']*\)'.*|\1|""#, angular_relative_path);
    let shell = if cfg!(target_os = "windows") {"cmd"} else {"sh"};

    eprintln!("\tGetting Angular routes...");
    let output = Command::new(shell)
        .args(["-c", &get_angular_routes_command])
        .output().expect("Failed to start Angular UI Application!")
        .stdout;
    let angular_routes_output = String::from_utf8(output).expect("command output is not in UTF-8");
    let angular_routes = angular_routes_output.split("\n");

    eprintln!("\tRunning Angular's production build: {:?} ==> '{}'", ANGULAR_BUILD_TYPE, full_build_command);
    let _exit_status = Command::new(shell)
        .args(["-c", &full_build_command])
        .spawn().expect(&format!("Failed to start Angular UI build command '{}'", full_build_command))
        .wait().unwrap();

    // reads all static files, recursively
    let mut files_contents = HashMap::<String, Vec<u8>>::new();
    let mut current_dir = env::current_dir().unwrap();
    current_dir = current_dir.join(angular_dist_path);
    let root_dir = PathBuf::from(&current_dir);
    eprintln!("Incorporating all files from '{:?}' into the executable -- and compressing them with {:?}", root_dir, COMPRESSOR);
    WalkDir::new(current_dir)
        .into_iter()
        .filter_entry(|entry| entry
            .file_name()
            .to_str()
            .map(|entry_name| entry_name != "." && entry_name != "..")
            .unwrap_or(false))
        .filter_map(|v| v.ok())
        .for_each(|entry| {
            let metadata = fs::metadata(&entry.path()).unwrap();
            if metadata.is_file() {
                let file_contents = fs::read(&entry.path()).expect(&format!("Cannot read file contents: '{:?}'", entry));
                let relative_file_name = entry.path().to_string_lossy().to_string().replace(root_dir.to_str().unwrap(), "");
                files_contents.insert(relative_file_name, file_contents);
            }
        });

    // includes all angular routes as links to index.html
    // -- for universal builds, they'll be linked to 'index.original.html' and the pre-rendered
    //    routes will be overwritten by the corresponding pre-rendered file
    let dynamic_routes_index_name = match ANGULAR_BUILD_TYPE {
        AngularBuildTypes::PreRenderedUniversal => "index.original.html",
        AngularBuildTypes::Regular => "index.html",
    };
    eprintln!("\tLinking '/{}' to all dynamic Angular routes", dynamic_routes_index_name);
    let mut file_links: HashMap<String, String> = angular_routes.into_iter()
        .map(|route| (format!("/{}", route), format!("/{}", dynamic_routes_index_name)) )
        .collect();

    // allows automatic dir -> dir/index.html access -- pre-rendered routes uses this mechanism
    eprintln!("\tLinking 'index.html's to their parent directory name -- so '/dir/index.html' may be accessed by just '/dir'...");
    for (file_name, _file_contents) in &files_contents {
        if file_name.ends_with("index.html") {
            let mut directory_name = file_name.replace("/index.html", "");
            if directory_name.len() == 0 {
                // instead of linking "", links "/" to index.html
                directory_name = "/".to_string();
            }
            file_links.insert(directory_name.to_string(), file_name.to_string());
        }
    }

    save_static_files(files_contents, file_links);
}

/// saves (possibly compressing) 'static_files' into a const hash map for use by the web server & application when
/// clients request them. Additionally, defines some constants related to compression & optimizing the browser's cache.\
/// 'file_links' refers to 'static_files' in the form {link_name = real_file_name, ...}\
fn save_static_files(static_files: HashMap<String, Vec<u8>>, file_links: HashMap<String, String>) {
    const CACHE_MAX_AGE_SECONDS: u64 = 3600 * 24 * 365;
    let out_dir = env::var_os("OUT_DIR").expect("Environment var 'OUT_DIR' is not present");
    let dest_path = Path::new(&out_dir).join("static_files.rs");
    let mut writer = BufWriter::with_capacity(4*1024*1024, fs::File::create(dest_path).unwrap());

    // file names to Rust identifiers (file contents are stored in consts)
    fn file_name_as_token(file_name: &str) -> String {
        let to_underscore = ["/", "-", "."];
        let mut file_name_as_token = file_name.to_string();
        file_name_as_token.insert_str(0, "FILE_");
        for replacement in to_underscore {
            file_name_as_token = file_name_as_token.replace(replacement, "_");
        }
        file_name_as_token
    }

    let file_header = r#"
// Auto-generated by build.rs. See there for docs.

use std::collections::HashMap;

"#;
    let hash_map_header = r#"
lazy_static! {
    pub static ref STATIC_FILES: HashMap<&'static str, (/*compressed*/bool, /*contents*/&'static [u8])> = {
        let mut m = HashMap::new();
"#;
    let function_and_file_footers = r#"
        m
    };
}"#;

    // header
    writer.write(file_header.as_bytes()).unwrap();

    // file constants
    for (file_name, file_contents) in &static_files {
        let compressed_bytes = compress(&file_name, &file_contents);
        if compressed_bytes.len() + COMPRESSION_THRESHOLD < file_contents.len() {
            // serve it compressed (text)
            writer.write(format!("// \"{}\": {} compressed / {} plain ==> compressed to {:.2}% of the original\n\
                                       const {}: (bool, &[u8]) = (true, &{:?});\n",
                                 file_name, compressed_bytes.len(), file_contents.len(), (compressed_bytes.len() as f64 / file_contents.len() as f64) * 100.0,
                                 file_name_as_token(file_name), compressed_bytes.as_slice()).as_bytes() ).unwrap();
        } else {
            // serve it plain (images, videos, ...)
            writer.write(format!("\n// \"{}\": {} compressed / {} plain ==> would be {:.2}% of the original\n\
                                         const {}: (bool, &[u8]) = (false, &{:?});\n",
                                 file_name, compressed_bytes.len(), file_contents.len(), (compressed_bytes.len() as f64 / file_contents.len() as f64) * 100.0,
                                 file_name_as_token(file_name), file_contents.as_slice()).as_bytes() ).unwrap();
        }
    }

    // Content-Encoding (compressor) constant
    writer.write(format!("\npub const CONTENT_ENCODING: &str = \"{}\";\n", compressor_http_header()).as_bytes()).unwrap();

    // date constants
    let now_time: DateTime<Utc> = Utc::now();
    let expiration_time = DateTime::<Utc>::from(SystemTime::from(now_time).add(Duration::from_secs(CACHE_MAX_AGE_SECONDS)));
    let generation_date_str = now_time.to_rfc2822();
    let expiration_date_str = expiration_time.to_rfc2822();
    let cache_control_str = format!("public, max-age: {}", CACHE_MAX_AGE_SECONDS);
    writer.write(format!("pub const GENERATION_DATE:  &str = \"{}\";\n", generation_date_str).as_bytes() ).unwrap();
    writer.write(format!("pub const EXPIRATION_DATE:  &str = \"{}\";\n", expiration_date_str).as_bytes() ).unwrap();
    writer.write(format!("pub const CACHE_CONTROL:    &str = \"{}\";\n\n", cache_control_str).as_bytes() ).unwrap();

    // hash map header
    writer.write(hash_map_header.as_bytes() ).unwrap();

    // contents (hash map)
    writer.write("        // links\n".as_bytes() ).unwrap();
    for (link_name, real_file_name) in &file_links {
        writer.write(format!("        m.insert(\"{}\", {});\n", link_name, file_name_as_token(real_file_name)).as_bytes() ).unwrap();
    }
    writer.write("        // files\n".as_bytes() ).unwrap();
    for (file_name, _file_contents) in &static_files {
        writer.write(format!("        m.insert(\"{}\", {});\n", file_name, file_name_as_token(file_name)).as_bytes() ).unwrap();
    }

    // footer
    writer.write(function_and_file_footers.as_bytes() ).unwrap();
}

/// adapter for a compressor, respecting the global config
fn compress(file_name: &String, file_content: &Vec<u8>) -> Vec<u8> {
    match COMPRESSOR {
        Compressors::GZip => gzip_compress(&file_name, &file_content),
        Compressors::Brotli => brotli_compress(&file_name, &file_content),
    }
}

/// returns the corresponding 'Content-Encoding' HTTP header value for the chosen 'COMPRESSOR'
fn compressor_http_header() -> &'static str {
    match COMPRESSOR {
        Compressors::GZip => "gzip",
        Compressors::Brotli => "br",
    }
}

use flate2::{
    Compression,
    write::GzEncoder,
};
/// equivalent of 'gzip -9'
fn gzip_compress(file_name: &String, file_content: &Vec<u8>) -> Vec<u8> {
    let mut gzip = GzEncoder::new(Vec::new(), Compression::best());
    gzip.write_all(file_content).unwrap();
    let gzipped_bytes = gzip.finish().expect(&format!("Could not compress file '{}'", file_name));
    gzipped_bytes
}


/// equivalent of 'brotli -q 11 -w 24'
fn brotli_compress(_file_name: &String, file_content: &Vec<u8>) -> Vec<u8> {
    let mut brotlied_bytes = Vec::new();
    let mut brotli = brotli::CompressorWriter::new(&mut brotlied_bytes, 4096, 11, 24);
    brotli.write_all(file_content).unwrap();
    brotli.flush().unwrap();
    drop(brotli);
    brotlied_bytes
}