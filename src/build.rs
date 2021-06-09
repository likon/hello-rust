use std::env;

fn main() {
    let staging_dir = env::var("STAGING_DIR").unwrap();
    println!(
        r"cargo:rustc-link-search={}/target-mipsel_24kec+dsp_uClibc-0.9.33.2/usr/lib",
        staging_dir
    );
}
