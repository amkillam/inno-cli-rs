//import dll GenerateExec.dll
//import function GenerateExec from GenerateExec.dll
//Load innosetup installer .exe path from commandline arg
//Load installer to memory
//Call extract::tmp::tmp to extract installer tmp files to current directory
//Call extract::embedded::install_script to get pascalscript bytecode as Pascal ANSI string
//call GenerateExec with arg install_script as Pascal ANSI string
//call exec::installer::run_installer with arg returned from GenerateExec
//

mod exec;
mod extract;
mod handle_progress;
mod parse;
pub mod pascal_types;

use clap::Parser;
use exec::run_installer;
use mimalloc::MiMalloc;
use std::path::PathBuf;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Path in which to extract temporary setup files. Defaults to ./tmp if not provided.
    #[arg(long, default_value = "./tmp")]
    tmp_dir: PathBuf,

    // Path in which to install the setup output
    #[arg(long)]
    install_dir: PathBuf,

    // Path to the Inno Setup installer
    #[arg(long)]
    installer_path: PathBuf,

    // Inno Setup components to install. Defaults to "*" if not provided.
    #[arg(long, default_value = "*")]
    components: Vec<String>,

    // Inno Setup tasks to perform. Defaults to "*" if not provided.
    #[arg(long, default_value = "*")]
    tasks: Vec<String>,

    // Inno Setup install type to complete. Defaults to "full" if not provided.
    #[arg(long, default_value = "full")]
    install_type: String,

    // Path to extracted Inno Setup pascalscript bytecode. This will be extracted from the installer if not provided.
    #[arg(long)]
    install_script: String,
}

fn main() {
    let args: Args = Args::parse();
    let tmp_dir = args.tmp_dir;
    let install_dir = args.install_dir;
    let installer_path = args.installer_path;
    let _components = args.components;
    let _tasks = args.tasks;
    let _install_type = args.install_type;
    let install_script = args.install_script;

    if !tmp_dir.exists() {
        std::fs::create_dir_all(&tmp_dir).unwrap();
    }

    if !install_dir.exists() {
        std::fs::create_dir_all(&install_dir).unwrap();
    }

    let _installer = std::fs::read(installer_path).unwrap();

    // let tmp_files = extract::tmp::tmp(&tmp_dir, &installer);

    //cd into tmp_dir
    std::env::set_current_dir(&tmp_dir).unwrap();

    let install_script =
    // if install_script.is_empty() {
    //     extract::embedded::install_script(&installer)
    // } else {
        std::fs::read(install_script).unwrap()
    // }
    ;

    run_installer(install_script);
}
