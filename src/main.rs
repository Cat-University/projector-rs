use build::Build;
use std::env::args;
use std::fs::{create_dir, write};

mod build;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let build_hash = args().nth(1).unwrap();

    println!("Downloading build {}", build_hash);

    let client = reqwest::Client::new();
    let build = client
        .get(&format!("https://api.discord.sale/builds/{}", build_hash))
        .send()
        .await?
        .json::<Build>()
        .await?;

    write(
        "config.json",
        serde_json::to_string_pretty(&build)?.as_bytes(),
    )?;

    create_dir("./modules")?;

    for root_script in &build.files.root_scripts {
        let root_script = format!("{}.js", root_script);
        println!("Dowloading file \x1b[36m{}\x1b[0m", root_script);
        let file = client
            .get(format!("https://canary.discord.com/assets/{}", root_script))
            .send()
            .await?
            .text()
            .await?;
        write(root_script, file.as_bytes())?;
    }

    for module in &build.files.modules {
        let module = format!("{}.js", module);
        println!("Dowloading module \x1b[36m{}\x1b[0m", module);
        let file_path = format!("./modules/{}", module);
        let file = client
            .get(format!("https://canary.discord.com/assets/{}", module))
            .send()
            .await?
            .text()
            .await?;
        write(file_path, file.as_bytes())?;
    }

    for css in &build.files.css {
        let css = format!("{}.css", css);
        println!("Dowloading stylesheet \x1b[36m{}\x1b[0m", css);
        let file = client
            .get(format!("https://canary.discord.com/assets/{}", css))
            .send()
            .await?
            .text()
            .await?;
        write(css, file.as_bytes())?;
    }

    Ok(())
}
