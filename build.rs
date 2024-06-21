use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

pub fn main() {
    if Some("1".to_string()) == env::var("DEBUG_SECRETS").ok() {
        println!("cargo:rerun-if-changed=.env");
        println!("cargo:rerun-if-env-changed=DEBUG_SECRETS");
        println!("cargo:rustc-cfg=debug_secrets");

        let spotify_creds =
            rspotify::Credentials::from_env().expect("Failed to get credentials from environment");
        let spotify_expr = format!(
            "rspotify::Credentials::new(\"{}\", \"{}\")",
            spotify_creds.id,
            spotify_creds
                .secret
                .expect("Expected RSPOTIFY_CLIENT_SECRET")
        );
        create_secret_file("spotify", &[&spotify_expr]);

        let cors_secret = env::var("CORS_SECRET").unwrap();
        let cors_expr = format!("\"{}\"", cors_secret);
        create_secret_file("cors", &[&cors_expr]);
    }
}

fn create_secret_file(name: &str, expr: &[&str]) {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join(&format!("{name}_secret.rs"));
    let mut file = BufWriter::new(File::create(path).unwrap());
    for expr in expr {
        write!(&mut file, "{}", expr).unwrap();
    }
}
