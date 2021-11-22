// build.rs: keeps static files to be served inside the main executable, allowing for blasting-fast service (without context switches)

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
use flate2::{
    GzBuilder,
    Compression,
    write::GzEncoder,
};

/// how smaller (in bytes) the gzipped file must be, in comparison to the plain version, for us to serve it in the compressed form
const GZIP_THRESHOLD: usize = 100;

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

/// builds the angular site for production, then loads (and compresses) the resulting static files, storing them in a hash map for use by the application.\
///
fn on_release() {
    let angular_relative_path = "./angular";
    let angular_dist_path = format!("{}/dist/DemoAngularAndBootstrapUI", angular_relative_path);
    let angular_production_build_command = format!("cd '{}' && ng build --aot --build-optimizer --optimization --prod --progress", angular_relative_path);
    let get_angular_routes_command = format!(r#"grep "{{ path: '" {}/src/app/app-routing.module.ts | sed "s|.* path: '\([^']*\)'.*|\1|""#, angular_relative_path);

    eprintln!("\tRunning Angular's production build: '{}'", angular_production_build_command);
    let shell;
    if cfg!(target_os = "windows") {
        shell = "cmd";
    } else {
        shell = "sh";
    };
    let _exit_status = Command::new(shell)
        .args(["-c", &angular_production_build_command])
        .spawn().expect("Failed to start Angular UI Application!")
        .wait().unwrap();

    // reads all static files, recursively
    let mut files_contents = HashMap::<String, Vec<u8>>::new();
    let mut current_dir = env::current_dir().unwrap();
    current_dir = current_dir.join(angular_dist_path);
    let root_dir = PathBuf::from(&current_dir);
    eprintln!("Incorporating all files from '{:?}' into the executable", root_dir);
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
    let output = Command::new(shell)
        .args(["-c", &get_angular_routes_command])
        .output().expect("Failed to start Angular UI Application!")
        .stdout;
    let output_string = String::from_utf8(output).expect("command not in UTF-8");
    let routes = output_string.split("\n");
    let file_links: HashMap<String, String> = routes.into_iter()
        .map(|route| (format!("/{}", route), "/index.html".to_string()) )
        .collect();
    //file_links.insert("/".to_string(), "/index.html".to_string());    <-- already included by the above command for an empty line (path: '')

    save_static_files(files_contents, file_links);
}

/// saves (possibly compressing) 'static_files' into a const hash map for use by the web server & application
/// when clients request them. Additionally, defines some date constants useful for optimizing the browser's cache.\
/// 'file_links' refers to 'static_files' in the form {link_name = real_file_name, ...}\
fn save_static_files(static_files: HashMap<String, Vec<u8>>, file_links: HashMap<String, String>) {
    const CACHE_MAX_AGE_SECONDS: u64 = 3600 * 24 * 365;
    let out_dir = env::var_os("OUT_DIR").unwrap();
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
    pub static ref STATIC_FILES: HashMap<&'static str, (/*gzipped*/bool, /*contents*/&'static [u8])> = {
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
        let mut gzip = GzEncoder::new(Vec::new(), Compression::best());
        gzip.write_all(file_contents).unwrap();
        let gzipped_bytes = gzip.finish().expect(&format!("Could not compress file '{}'", file_name));
        if gzipped_bytes.len() + GZIP_THRESHOLD < file_contents.len() {
            // serve it gzipped (text)
            writer.write(format!("// \"{}\": {} compressed / {} plain ==> compressed to {:.2}% of the original\n\
                                       const {}: (bool, &[u8]) = (true, &{:?});\n",
                                     file_name, gzipped_bytes.len(), file_contents.len(), (gzipped_bytes.len() as f64 / file_contents.len() as f64) * 100.0,
                                     file_name_as_token(file_name), gzipped_bytes.as_slice()).as_bytes() ).unwrap();
        } else {
            // serve it plain (images, videos, ...)
            writer.write(format!("// \"{}\": {} compressed / {} plain ==> would be {:.2}% of the original\n\
                                       const {}: (bool, &[u8]) = (false, &{:?});\n\n",
                                      file_name, gzipped_bytes.len(), file_contents.len(), (gzipped_bytes.len() as f64 / file_contents.len() as f64) * 100.0,
                                      file_name_as_token(file_name), file_contents.as_slice()).as_bytes() ).unwrap();
        }
    }

    // date constants
    let now_time: DateTime<Utc> = Utc::now();
    let expiration_time = DateTime::<Utc>::from(SystemTime::from(now_time).add(Duration::from_secs(CACHE_MAX_AGE_SECONDS)));
    let generation_date_str = now_time.to_rfc2822();
    let expiration_date_str = expiration_time.to_rfc2822();
    let cache_control_str = format!("public, max-age: {}", CACHE_MAX_AGE_SECONDS);
    writer.write(format!("pub const GENERATION_DATE: &str = \"{}\";\n", generation_date_str).as_bytes() ).unwrap();
    writer.write(format!("pub const EXPIRATION_DATE: &str = \"{}\";\n", expiration_date_str).as_bytes() ).unwrap();
    writer.write(format!("pub const CACHE_CONTROL:   &str = \"{}\";\n\n", cache_control_str).as_bytes() ).unwrap();

    // hash map header
    writer.write(hash_map_header.as_bytes() ).unwrap();

    // contents (hash map)
    writer.write("        // links\n".as_bytes() ).unwrap();
    for (link_name, real_file_name) in &file_links {
        writer.write(format!("        m.insert(\"{}\", {});\n", link_name, file_name_as_token(real_file_name)).as_bytes() ).unwrap();
    }
    writer.write("        // files\n".as_bytes() ).unwrap();
    for (file_name, file_contents) in &static_files {
        writer.write(format!("        m.insert(\"{}\", {});\n", file_name, file_name_as_token(file_name)).as_bytes() ).unwrap();
    }

    // footer
    writer.write(function_and_file_footers.as_bytes() ).unwrap();
}
