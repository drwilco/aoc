use jiff::Zoned;
use std::{
    env::args,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

#[derive(serde::Deserialize)]
struct Metadata {
    workspace_root: PathBuf,
}

fn copy_template(
    workspace_root: impl AsRef<Path>,
    day_dir: impl AsRef<Path>,
    year: i16,
    day: i8,
) -> Result<()> {
    let workspace_root = workspace_root.as_ref();
    let day_dir = day_dir.as_ref();

    let src_bin_dir = day_dir.join("src/bin");
    fs::create_dir_all(src_bin_dir)?;
    fs::copy(
        workspace_root.join("template/src/bin/first.rs"),
        day_dir.join("src/bin/first.rs"),
    )?;
    let cargo_toml = fs::read_to_string(workspace_root.join("template/Cargo.toml"))?;
    fs::write(
        day_dir.join("Cargo.toml"),
        cargo_toml.replace("template", &format!("aoc{year}day{day}")),
    )?;
    Ok(())
}

fn download_input(input_path: impl AsRef<Path>, year: i16, day: i8) -> Result<()> {
    let client = reqwest::blocking::Client::new();
    let session_cookie = std::env::var("SESSION_COOKIE").expect("SESSION_COOKIE not set");
    let cookie_header = format!("session={session_cookie}");
    let input = client
        .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
        .header(reqwest::header::COOKIE, cookie_header)
        .send()?
        .error_for_status()?
        .bytes()?;
    fs::write(input_path.as_ref(), input)?;
    Ok(())
}

fn main() -> Result<()> {
    dotenvy::dotenv().expect("failed to load .env file");

    // Run `cargo metadata --format-version 1` to get the workspace root
    let workspace_root = serde_json::from_slice::<Metadata>(
        &Command::new("cargo")
            .args(["metadata", "--format-version", "1"])
            .output()
            .expect("failed to execute process")
            .stdout,
    )?
    .workspace_root;
    // Get year and day from current date
    let now = Zoned::now();
    let year = now.year();
    let mut args = args().peekable();
    args.next();
    if let Some("aoc") = args.peek().map(String::as_str) {
        args.next();
    }
    let day = args
        .next()
        .map_or(now.day(), |s| s.parse().expect("argument needs to be an integer"));
    let day_dir = workspace_root.join(format!("{year}/day{day}"));
    println!("{}", day_dir.display());
    if !day_dir.exists() {
        copy_template(&workspace_root, &day_dir, year, day)?;
    }

    let input_path = day_dir.join("input.txt");
    if !input_path.exists() {
        download_input(&input_path, year, day)?;
    }
    Ok(())
}
