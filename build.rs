use rspotify::Credentials;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

pub fn main() {
    if Some("1".to_string()) == env::var("DEBUG_SECRETS").ok() {
        println!("cargo:rerun-if-changed=.env");
        println!("cargo:rustc-cfg=debug_secrets");
        let path = Path::new(&env::var("OUT_DIR").unwrap()).join("env.rs");
        let mut file = BufWriter::new(File::create(path).unwrap());
        let credentials =
            Credentials::from_env().expect("Couldn't get credentials from environment");

        write!(
            &mut file,
            "rspotify::Credentials::new(\"{}\", \"{}\")",
            credentials.id,
            credentials.secret.expect("Expected RSPOTIFY_CLIENT_SECRET")
        )
        .unwrap();
    }
}
