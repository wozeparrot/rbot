use clap::{App, AppSettings, Arg, SubCommand};
use subprocess::Exec;

use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    let matches = App::new("cargo-rbot")
        .about("Run it as cargo rbot <command>!")
        .version("0.1.0")
        .bin_name("cargo")
        .subcommand(
            SubCommand::with_name("rbot")
                .about("rbot command line tool for deploying and create rbot projects")
                .arg(
                    Arg::with_name("verbose")
                        .long("verbose")
                        .short("v")
                        .global(true)
                        .multiple(false)
                )
                .subcommand(
                    SubCommand::with_name("create")
                        .about("create a rbot project")
                        .arg(
                            Arg::with_name("NAME")
                                .required(true)
                                .index(1)
                                .help("name of project")
                        )
                        .arg(
                            Arg::with_name("TEAM")
                                .required(true)
                                .index(2)
                                .help("team number")
                        )
                )
                .subcommand(
                    SubCommand::with_name("deploy")
                        .about("deploy a rbot project")
                        .arg(
                            Arg::with_name("release")
                                .long("--release")
                                .help("build in release mode")
                        )
                )
                .setting(AppSettings::SubcommandRequiredElseHelp)
        )
        .setting(AppSettings::SubcommandRequired)
        .get_matches();

    let rbot_matches = matches.subcommand_matches("rbot").expect("Failed");

    match rbot_matches.subcommand_name() {
        Some("create") => {
            create(rbot_matches.subcommand_matches("create").unwrap().value_of("NAME").unwrap(), rbot_matches.subcommand_matches("create").unwrap().value_of("TEAM").unwrap())
        }
        Some("deploy") => {
            deploy(rbot_matches.subcommand_matches("deploy").unwrap().is_present("release"))
        }
        _ => panic!("Unknown Subcommand")
    }
}

fn create(name: &str, team: &str) {
    fs::create_dir(name).expect("Directory Creation Failed");
    fs::create_dir(format!("{}/{}", name, "src/")).expect("Directory Creation Failed");

    let mut f = File::create(format!("{}/{}", name, "Cargo.toml")).expect("Cargo.toml Creation Failed");
    f.write_all(b"[package]\n").unwrap();
    f.write_all(b"name = ").unwrap();
    f.write_all(format!("\"{}\"", name).as_bytes()).unwrap();
    f.write_all(b"\nversion = \"0.1.0\"").unwrap();
    f.write_all(b"\nedition = \"2018\"").unwrap();
    f.write_all(b"\n").unwrap();
    f.write_all(b"\n[dependencies]\n").unwrap();
    f.write_all(b"rbotlib = \"0.0.2\"").unwrap();

    f.sync_all().unwrap();

    let mut f = File::create(format!("{}/{}", name, ".rbotconfig")).expect(".rbotconfig Creation Failed");
    f.write_all(b"[deploy]\n").unwrap();
    f.write_all(b"team = ").unwrap();
    f.write_all(format!("\"{}\"", team).as_bytes()).unwrap();

    f.sync_all().unwrap();

    let mut f = File::create(format!("{}/{}/{}", name, "src", "main.rs")).expect("src/main.rs Creation Failed");
    f.write_all(b"fn main() {").unwrap();
    f.write_all(b"    println!(\"Hello rbot!\"))").unwrap();
    f.write_all(b"}").unwrap();

    f.sync_all().unwrap();
}

fn deploy(release: bool) {
    build_project(release);
}

fn build_project(release: bool) {
    let mut args = vec![
        "build",
        "--target=arm-unknown-linux-gnueabi",
    ];

    if release {
        args.push("--release");
    }

    Exec::cmd("cargo").args(&args).join().expect("cargo build Failed");
}