use clap::{Parser, Subcommand};
use regex::Regex;
use std::env::current_dir;
use std::fs::{read_dir, read_to_string, remove_file, File};
use std::io::Write;
use std::path::PathBuf;


/// Add LICENSE file to directory.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args
{
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands
{
    /// Add license to directory
    Add {
        /// License name
        name: String,
    },

    /// Select license from directory
    Select {},

    /// List licenses
    List {},
}

fn main()
{
    let args = Args::parse();

    let exe_dir = std::env::current_exe().unwrap_or("".into());

    if exe_dir.to_str().unwrap().is_empty()
    {
        eprintln!("Cannot get directory of executable.");
        return;
    }

    let licenses_dir = exe_dir.parent().unwrap().join(PathBuf::from("licenses"));

    match args.command {
        Commands::Add { name } => add(name, &licenses_dir),
        Commands::Select { .. } => select(&licenses_dir),
        Commands::List { .. } => list(&licenses_dir),
    };
}

fn select(licenses_dir: &PathBuf)
{
    let licenses_entry = read_dir(licenses_dir);
    if licenses_entry.is_err()
    {
        eprintln!("No licenses directory found at {:#?}.", licenses_dir);
        return;
    }

    println!("Enter the index of the license to add.");

    let mut dir_len = 0usize;
    let mut dir_vec = Vec::new();
    for (i, entry) in licenses_entry.unwrap().enumerate()
    {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        let mut file_name = file_name.to_str().unwrap();
        file_name = file_name.trim_end_matches(".txt");
        println!("{}: {}", i, file_name);
        dir_vec.push(file_name.to_string());
        dir_len += 1;
    }
    print!("Index: ");
    std::io::stdout().flush().unwrap();

    let mut index = String::new();
    std::io::stdin().read_line(&mut index).unwrap();

    let index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("{} is not a number.", index.trim());
            return;
        }
    };

    if index >= dir_len
    {
        eprintln!("Invalid index. (0-{})", dir_len-1);
        return;
    }

    println!();

    add(dir_vec[index].clone(), licenses_dir);
}

fn add(name: String, licenses_dir: &PathBuf)
{
    let licenses_entry = read_dir(licenses_dir);
    if licenses_entry.is_err()
    {
        eprintln!("No licenses directory found at {:#?}.", licenses_dir);
        return;
    }

    let entries = licenses_entry.unwrap()
        .map(|entry| entry.unwrap())
        .collect::<Vec<_>>();

    match entries
        .iter()
        .find(|entry| entry
            .file_name()
            .to_str()
            .unwrap()
            .trim_end_matches(".txt") == &name)
    {
        None => eprintln!("No license named \"{}\" found.", name),
        Some(entry) => {
            if let Ok(current_dir) = current_dir()
            {
                if let Ok(file) = File::options()
                    .create_new(true).create(false).write(true).read(false)
                    .open(&current_dir.join("LICENSE"))
                {
                    create_license(file, entry.path(), current_dir);
                }
                else
                {
                    eprintln!("Cannot create LICENSE file of type \"{}\" at {:#?}. It may already exist.", name, current_dir);
                }
            }
            else
            {
                eprintln!("Cannot get current directory.");
            }
        }
    }
}

fn create_license(mut file: File, path_to_source: PathBuf, current_dir: PathBuf)
{
    let mut content = read_to_string(path_to_source).unwrap();
    let mut replace_items = Vec::new();

    let re = Regex::new(r"\[.+?]").unwrap();
    for item in re.find_iter(&content).map(|m| m.as_str())
    {
        print!("Enter nothing if it shouldn't be changed.\n{} = ", item);
        std::io::stdout().flush().unwrap(); // Weird
        let mut line = String::new();
        match std::io::stdin().read_line(&mut line)
        {
            Ok(_) => {
                if !line.trim().is_empty()
                {
                    replace_items.push(String::from(line.trim()));
                }
                else
                {
                    replace_items.push(String::from(item));
                }
            }
            Err(_) => continue,
        }
        println!();
    }

    let mut slices = Vec::new();
    for replace_item in replace_items.iter()
    {
        content = re.replace(&content, replace_item.as_str()).into();
        let (slice1, slice2) = content.split_once(replace_item.as_str()).unwrap();
        slices.push(format!("{}{}", slice1, replace_item));
        content = slice2.into();
    }

    content = format!("{}{}", slices.join(""), content);

    if file.write_all(content.as_bytes()).is_err()
    {
        eprintln!("Cannot write to file at \"{}\".", &current_dir.join("LICENSE").display());
        if remove_file(&current_dir.join("LICENSE")).is_err()
        {
            eprintln!("Cannot remove file that failed to write at \"{}\".", current_dir.join("LICENSE").display());
        }
    }
}

fn list(licenses_dir: &PathBuf)
{
    let licenses_entry = read_dir(licenses_dir);
    if licenses_entry.is_err()
    {
        eprintln!("No licenses directory found at {:#?}.", licenses_dir);
        return;
    }

    for entry in licenses_entry.unwrap()
    {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        let mut file_name = file_name.to_str().unwrap();
        file_name = file_name.trim_end_matches(".txt");
        println!("{}", file_name);
    }
}
