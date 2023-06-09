use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

use clap::Parser;
use minijinja::Environment;
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

type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;

fn main() -> Result<(), DynError> {
    let args = Args::parse();

    let template_path = &args.tmpl;
    let yaml_path = &args.var;

    let mut env = Environment::new();
    env.set_undefined_behavior(minijinja::UndefinedBehavior::Strict);

    let template_contents = read_file_to_string(template_path)?;
    env.add_template(template_path, &template_contents)?;
    let template = env.get_template(template_path)?;

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

fn read_yaml_file<P: AsRef<Path>>(file_path: P) -> Result<Value, serde_yaml::Error> {
    let contents = read_file_to_string(file_path).expect("open or read yaml file");
    serde_yaml::from_str(&contents)
}

fn read_file_to_string<P: AsRef<Path>>(file_path: P) -> Result<String, io::Error> {
    let mut file = File::open(&file_path).unwrap_or_else(|_| {
        panic!(
            "unable to open file: {}",
            file_path.as_ref().to_string_lossy()
        )
    });
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap_or_else(|_| {
        panic!(
            "unable to read file: {}",
            file_path.as_ref().to_string_lossy()
        )
    });
    Ok(contents)
}
