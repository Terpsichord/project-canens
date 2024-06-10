use rspotify::Credentials;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::io::Write;

pub fn main() {
    if Ok("debug".to_string()) == env::var("PROFILE") {
        println!("cargo:rerun-if-changed=.env");
        let path = Path::new(&env::var("OUT_DIR").unwrap()).join("env.rs");
        let mut file = BufWriter::new(File::create(&path).unwrap());
        let credentials =
            Credentials::from_env().expect("Couldn't get credentials from environment");

        write!(
            &mut file,
            "rspotify::Credentials::new(\"{}\", \"{}\")",
            credentials.id,
            credentials.secret.expect("Expected RSPOTIFY_CLIENT_SECRET")
        ).unwrap();
    }
}
