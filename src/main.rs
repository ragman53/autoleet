use clap::Parser;
use std::fs;
use std::path::PathBuf;
use tera::{Context, Tera};
use std::error::Error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    index: u32,

    #[arg(short, long)]
    title: String,

    #[arg(long)]
    dry_run: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut tera = Tera::new("templates/**/*")?;
    let args = Args::parse();

    if args.dry_run {
        println!("---Dry Run Mode !!!---");
    }

    println!("No.{:?} - {:?}", args.index, args.title);
    println!("{:?}", args);

    let mut context = Context::new();
    context.insert("index", &args.index);
    context.insert("title", &args.title);

    let slugified_title: String = tera
        .render_str("{{ title | slugify }}", &context)?;

    let dir_name = format!("p{:04}-{}", args.index, slugified_title);
    println!("Directory name: {}", dir_name);

    for temp_path_str in tera.get_template_names() {
        println!("Processing template: {}", temp_path_str);

        let output_path = PathBuf::from(&dir_name).join(temp_path_str.strip_suffix(".tera").unwrap_or(temp_path_str));
        println!("[Dry Run] would create directory: {:?}", output_path);

        if !args.dry_run {
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let contents = tera.render(temp_path_str, &context)?;

            fs::write(&output_path, contents)?;

            println!(" -> Created file: {:?}", output_path);
        } else {
            println!("File Operation was skipped: {:?}", output_path);
        }
    }
    println!("\nProject '{}' created successfully!", dir_name);

    Ok(())
}
