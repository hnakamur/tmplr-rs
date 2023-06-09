use std::{
    env::current_dir,
    fs::{self, File},
    io::Write,
    path::Path,
};

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use minijinja::{Environment, Error, ErrorKind};
use serde_yaml::Value;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// variable yaml file path
    #[arg(short, long)]
    var: String,

    /// template file path
    #[arg(short, long)]
    tmpl: String,

    /// destination file path
    #[arg(short, long, default_value_t = String::from("-"))]
    dest: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let template_path = current_dir()?.join(&args.tmpl);
    let yaml_path = &args.var;

    let mut env = Environment::new();
    env.set_undefined_behavior(minijinja::UndefinedBehavior::Strict);

    let template_dir = template_path.parent().unwrap().to_owned();
    env.set_loader(move |name| {
        let path = template_dir.join(name);
        match fs::read_to_string(path) {
            Ok(result) => Ok(Some(result)),
            Err(err) => {
                let msg = if err.kind() == std::io::ErrorKind::NotFound {
                    "template not found"
                } else {
                    "failed to load template"
                };
                Err(Error::new(ErrorKind::TemplateNotFound, msg).with_source(err))
            }
        }
    });

    let template_file = template_path
        .as_path()
        .file_name()
        .ok_or(anyhow!("invalid template path"))?
        .to_str()
        .ok_or(anyhow!("not UTF-8 template path"))?;
    let template = env.get_template(template_file)?;

    let yaml_data = read_yaml_file(yaml_path)?;
    let rendered_contents = template.render(yaml_data)?;
    if &args.dest == "-" {
        println!("{}", rendered_contents);
    } else {
        let mut file = File::create(args.dest)?;
        writeln!(&mut file, "{}", rendered_contents)?;
    }
    Ok(())
}

fn read_yaml_file<P: AsRef<Path>>(file_path: P) -> Result<Value> {
    let contents = fs::read_to_string(file_path)?;
    serde_yaml::from_str(&contents).context("load variables in YAML")
}
