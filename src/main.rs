use std::{
    env::{args, current_dir},
    fs, io,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
};

const CPP_HELLO_WORLD: &str = include_str!("cpp_hello_world.txt");
const COMPILER_FLAG: &str = include_str!("compiler_flag.txt");
const CMAKE_LISTS: &str = include_str!("cmake_config.txt");
const USAGE_MSG: &str = include_str!("usage.txt");

fn main() -> ExitCode {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        eprintln!("{USAGE_MSG}");
        return ExitCode::FAILURE;
    }

    match args[1].as_str() {
        "new" => cmd_new(&args),
        "build" => cmd_build(),
        "run" => cmd_run(),
        _ => {
            eprintln!("{USAGE_MSG}");
            ExitCode::FAILURE
        }
    }
}

fn cmd_new(args: &[String]) -> ExitCode {
    if args.len() != 3 {
        eprintln!("Usage: castle new <project_name>");
        return ExitCode::FAILURE;
    }
    if !is_project_path_valid(&args[2]) {
        return ExitCode::FAILURE;
    }

    let project_name = PathBuf::from(&args[2]);

    if project_name.exists() {
        eprintln!("Castle: Project {} already exists.", project_name.display());
        return ExitCode::FAILURE;
    }

    match create_project_dir(&project_name) {
        Ok(()) => {
            println!("Castle: created project {}", project_name.display());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Castle: {e}");
            ExitCode::FAILURE
        }
    }
}

fn cmd_build() -> ExitCode {
    if !check_clang_version() {
        return ExitCode::FAILURE;
    }

    let Some(root_dir) = find_root_dir() else {
        eprintln!("Castle: Not inside a castle project (no CMakeLists.txt found).");
        return ExitCode::FAILURE;
    };

    let ok = run_step(
        Command::new("cmake")
            .args(["-S", ".", "-B", "build"])
            .current_dir(&root_dir),
        "cmake configure",
    )
    .is_ok()
        && run_step(
            Command::new("cmake")
                .args(["--build", "build"])
                .current_dir(&root_dir),
            "cmake build",
        )
        .is_ok();

    if ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn cmd_run() -> ExitCode {
    let Some(bin) = find_binary() else {
        eprintln!("Castle: Root not found.");
        return ExitCode::FAILURE;
    };

    let Some(bin) = bin.to_str() else {
        eprintln!("Path to {} is not UTF-8 valid.", bin.display());
        return ExitCode::FAILURE;
    };

    match run_step(&mut Command::new(bin), "program") {
        Ok(()) => ExitCode::SUCCESS,
        Err(code) => ExitCode::from(code as u8),
    }
}

fn find_binary() -> Option<PathBuf> {
    let root_dir = find_root_dir()?;
    Some(root_dir.join("build").join("bin"))
}

fn find_root_dir() -> Option<PathBuf> {
    let mut dir = current_dir().ok()?;

    loop {
        if dir.join("CMakeLists.txt").is_file() {
            return Some(dir);
        }
        if !dir.pop() {
            return None;
        }
    }
}

fn create_project_dir(project_name: &Path) -> io::Result<()> {
    let src = project_name.join("src");
    fs::create_dir_all(&src)?;
    fs::write(src.join("main.cpp"), CPP_HELLO_WORLD)?;
    fs::write(project_name.join(".clangd"), COMPILER_FLAG)?;
    fs::write(project_name.join("CMakeLists.txt"), CMAKE_LISTS)?;

    Ok(())
}

fn run_step(cmd: &mut Command, label: &str) -> Result<(), i32> {
    match cmd.status() {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => {
            eprintln!("Castle: {label} exited with status {status}");
            Err(status.code().unwrap_or(1))
        }
        Err(err) => {
            eprintln!("Castle: {err}");
            Err(1)
        }
    }
}

fn is_project_path_valid(arg: &str) -> bool {
    if arg.is_empty() {
        eprintln!("Castle: Project must have a name.");
        return false;
    }

    if !arg.is_ascii() || arg.contains(' ') {
        eprintln!("Castle: Project can't contain spaces or non-ASCII characters.");
        return false;
    }

    if arg.contains('/') || arg.contains("..") {
        eprintln!("Castle: Project name must be a single directory name.");
        return false;
    }

    true
}

fn parse_clang_version(stdout: &str) -> Option<&str> {
    stdout
        .lines()
        .next()?
        .split_whitespace()
        .skip_while(|&token| token != "version")
        .nth(1)
}

fn is_version_compatible(stdout: &str) -> Result<bool, String> {
    let Some(clang_version) = parse_clang_version(stdout) else {
        return Err(format!(
            "Apparently, the function parse_clang_version() returned None. {stdout}."
        ));
    };

    let digits: String = clang_version
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    let Ok(version_number) = digits.parse::<u32>() else {
        return Err(format!(
            "Failed to parse major version from '{clang_version}'"
        ));
    };

    Ok(version_number >= 18)
}

fn check_clang_version() -> bool {
    let Ok(output) = Command::new("clang").arg("--version").output() else {
        eprintln!("Castle: Clang compiler not found.");
        return false;
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    match is_version_compatible(&stdout) {
        Ok(true) => true,
        Ok(false) => {
            eprintln!("Castle: Clang 18 or newer is required.");
            false
        }
        Err(err) => {
            eprintln!("Castle: {err}");
            false
        }
    }
}
