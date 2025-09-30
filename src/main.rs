//! A tool for updating and running `musiq`

#[cfg(target_os = "windows")]
compile_error!("This program was not designed to be run on Windows");

use std::path::PathBuf;
macro_rules! or_return {
    ($e:expr, $r:expr) => {
        match $e {
            Some(x) => x,
            None => return $r
        }
    };
    ($e:expr) => {
        match $e {
            Some(x) => x,
            None => return
        }
    }
}

macro_rules! return_unless {
    ($e:expr, $r:expr) => {
        if !$e {
            return $r;
        }
    };
}

fn main() {
    let mut args = std::env::args().skip(1);

    let musiq_source_path = PathBuf::from(or_return!(
        std::env::var("MUSIQ_SOURCE_PATH").ok().or_else(|| args.next()),
        println!("MUSIQ_SOURCE_PATH environment variable or a command-line argument is required")
    ));

    let musiq_path = PathBuf::from(or_return!(
        std::env::var("MUSIQ_PATH").ok().or_else(|| args.next()),
        println!("MUSIQ_PATH environment variable or a command-line argument is required")
    ));

    // git fetch --all
    return_unless!(
        or_return!(
            or_return!(
                std::process::Command::new("git")
                    .args(["fetch", "--all"])
                    .current_dir(&musiq_source_path)
                    .spawn()
                    .ok(),
                println!("\"git fetch\" cannot be ran")
            ).wait().ok(),
            println!("\"git push\" cannot be ran")
        ).success(),
        println!("\"git fetch\" exited unsuccessfully")
    );

    // git branch origin/master
    let _ = or_return!(
        or_return!(
            std::process::Command::new("git")
                .args(["branch", "origin/master"])
                .current_dir(&musiq_source_path)
                .spawn()
                .ok(),
            println!("\"git branch\" cannot be ran")
        ).wait().ok(),
        println!("\"git branch\" cannot be ran")
    ).success();

    // git reset --hard origin/master
    return_unless!(
        or_return!(
            or_return!(
                std::process::Command::new("git")
                    .args(["reset", "--hard", "origin/master"])
                    .current_dir(&musiq_source_path)
                    .spawn()
                    .ok(),
                println!("\"git reset\" cannot be ran")
            ).wait().ok(),
            println!("\"git reset\" cannot be ran")
        ).success(),
        println!("\"git reset\" exited unsuccessfully")
    );

    // cargo build -rq
    return_unless!(
        or_return!(
            or_return!(
                std::process::Command::new("cargo")
                    .args(["build", "-rq"])
                    //.envs([("", ""); 0]) // TODO
                    .current_dir(&musiq_source_path)
                    .spawn()
                    .ok(),
                println!("\"cargo build\" cannot be ran")
            ).wait().ok(),
            println!("\"cargo build\" cannot be ran")
        ).success(),
        println!("\"cargo build\" exited unsuccessfully")
    );

    // cp %MUSIQ_SOURCE_PATH%/target/release/musiq %MUSIQ_PATH%/musiq
    or_return!(
        std::fs::copy(
            musiq_source_path.join(r"./target/release/musiq"),
            musiq_path.join(r"./musiq")
        ).ok(),
        println!("Copying musiq executable failed")
    );

    // tmux new -ds musiq %MUSIQ_PATH%/musiq 0.0.0.0:7878
    return_unless!(
        or_return!(
            or_return!(
                std::process::Command::new("tmux")
                    .args(["new", "-ds", "musiq", "./musiq", "0.0.0.0:7878"])
                    //.envs([("", ""); 0]) // TODO
                    .current_dir(&musiq_path)
                    .spawn()
                    .ok(),
                println!("\"tmux new\" cannot be ran")
            ).wait().ok(),
            println!("\"tmux new\" cannot be ran")
        ).success(),
        println!("\"tmux new\" exited unsuccessfully")
    );

    println!("Process created. Attach to it via \"tmux a -dt musiq\"");
}
