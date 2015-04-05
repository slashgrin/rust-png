use std::process::Command;
use std::process::Stdio;
use std::path::Path;
use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    let is_android = target.find("android").is_some();

    if is_android {
        let cc = format!("{}-gcc", target);
        let ar = format!("{}-ar", target);
        env::set_var("CC", &cc);
        env::set_var("AR", &ar);
    }

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cfg = Path::new(&manifest_dir).join("libpng-1.6.16/configure");
    let out_dir = env::var("OUT_DIR").unwrap();
    let dst = Path::new(&out_dir);

    env::set_var("CFLAGS", "-fPIC -O3");

    let mut cmd = Command::new(cfg);
    cmd.arg("--with-libpng-prefix=RUST_");
    if is_android {
        cmd.arg("--host=arm-linux-gnueabi");
    }
    cmd.current_dir(&dst);
    run(&mut cmd);

    let mut cmd = Command::new("make");
    cmd.arg("-j4");
    cmd.current_dir(&dst);
    run(&mut cmd);

    println!("cargo:root={}", dst.display());
    println!("cargo:rustc-flags=-l png16:static -L {}/.libs", dst.display());
}

fn run(cmd: &mut Command) {
    println!("running: {:?}", cmd);
    assert!(cmd.stdout(Stdio::inherit())
               .stderr(Stdio::inherit())
               .status()
               .unwrap()
               .success());
}
