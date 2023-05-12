use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use std::borrow::Cow;
use std::ffi::OsStr;
use std::fmt::{Debug, Display};
use std::io::{self, BufRead, BufReader};

use std::process::{exit, Command, Stdio};
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[clap(short, long, alias = "t")]
    teacher: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Install packages for the profile, if no repository provided, it defaults to nixpkgs
    #[command(arg_required_else_help = true, alias = "i", )]
    Install {
        /// Name of packages, optionally preceeded by the repository#. Examples: `htop`, `nixpkgs#htop`
        packages: Vec<Package>,
    },

    /// List installed packages
    #[command(alias = "ls")]
    List,

    /// Update all or specific packages
    #[command(alias = "u")]
    Upgrade,
    // {
    // /// Name of packages, optionally preceeded by the repository#. Examples: `htop`, `nixpkgs#htop`
    // packages: Option<Vec<Package>>
    // },
    /// Find a package in the registry, if no repository provided, it defaults to nixpkgs
    #[command(arg_required_else_help = true, alias = "s")]
    Search {
        /// Regex used to find the package. Examples: `nixpkgs#gnome3` or `gnome3`
        package: Package,
    },

    /// Remove one or more packages
    #[command(arg_required_else_help = true, alias = "rm")]
    Remove { packages: Vec<Package> },
}

/// Package can follow the format repo#name or just the name
#[derive(Debug, Clone, PartialEq)]
struct Package {
    repo: String,
    name: String,
    fullname: String,
}

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fullname)
    }
}

impl<T: AsRef<str>> From<T> for Package {
    fn from(name: T) -> Self {
        let items: Vec<&str> = name.as_ref().split("#").collect();
        if items.len() == 1 {
            let name = items
                .get(0)
                .cloned()
                .expect("Error while processing the package name");

            return Self {
                repo: "nixpkgs".into(),
                name: name.into(),
                fullname: format!("nixpkgs#{name}"),
            };
        }

        let repo = items
            .get(0)
            .cloned()
            .expect("Error while processing repository");

        let name = items
            .get(1)
            .cloned()
            .expect("Error while processing the package name");

        return Self {
            repo: repo.into(),
            name: name.into(),
            fullname: format!("{repo}#{name}"),
        };
    }
}
impl Into<String> for Package {
    fn into(self) -> String {
        return format!("{}#{}", self.repo, self.name);
    }
}

impl AsRef<OsStr> for Package {
    fn as_ref(&self) -> &OsStr {
        &OsStr::new(&self.fullname)
    }
}
#[derive(Debug, Clone, PartialEq)]
struct ListedPackage {
    position: String,
    store_path: String,
}

impl ListedPackage {
    fn new(position: String, store_path: String) -> Self {
        Self {
            position,
            store_path,
        }
    }
}

fn main() {
    let args = Cli::parse();
    let is_teacher = args.teacher;
    let cmd = Command::new("nix").arg("--version").output();
    if let Err(_) = cmd {
        println!("`nix` the package manager not found in your system.\n");
        println!("Installation:");
        println!("\thttps://nixos.org/download.html");
        exit(1);
    }

    let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim}{spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⠠⠐⠈ ");

    match args.command {
        Commands::Install { packages } => {
            println!("Installing package(s)...\n");

            let mut cmd = Command::new("nix");
            cmd.arg("profile")
                .arg("install")
                .args(&packages)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .stdin(Stdio::null());

            if is_teacher {
                let string_cmd = command_as_string(&cmd);
                println!("Learn mode on!");
                println!("Nix command created:");
                println!("\n\t{string_cmd}\n");
            }

            let mut child = cmd.spawn().expect("nix command failed to run.");
            let pkgs = packages
                .into_iter()
                .map(|p| p.into())
                .collect::<Vec<String>>()
                .join(" ");
            let pb = ProgressBar::new_spinner();
            pb.enable_steady_tick(Duration::from_millis(300));
            pb.set_style(spinner_style.clone());
            pb.set_message(pkgs.to_owned());
            for line in BufReader::new(child.stderr.take().unwrap()).lines() {
                let line = line.unwrap();
                let stripped_line = line.trim();
                if !stripped_line.is_empty() {
                    pb.set_message(stripped_line.to_owned());
                }
                pb.tick();
            }

            let output = child.wait_with_output().expect("Could not wait command");
            if output.status.success() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            } else {
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Commands::Search { package } => {
            let mut cmd = Command::new("nix");
            cmd.arg("search")
                .arg(&package.fullname)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .stdin(Stdio::null());
            if is_teacher {
                let string_cmd = command_as_string(&cmd);
                println!("Learn mode on!");
                println!("Nix command created:");
                println!("\n\t{string_cmd}\n");
            }
            println!("Searching for the package `{}`...\n", &package);
            let child = cmd.spawn().expect("nix command failed to run.");
            let output = child.wait_with_output().expect("Could not wait command");
            if output.status.success() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            } else {
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Commands::Remove { packages } => {
            let mut cmd = Command::new("nix");
            cmd.arg("profile")
                .arg("list")
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .stdin(Stdio::null());
            if is_teacher {
                let string_cmd = command_as_string(&cmd);
                println!("Learn mode on!");
                println!("Nix command created:");
                println!("\n\t{string_cmd}\n");
            }
            println!("Checking installed packages...");
            let out = cmd.output().expect("`nix profile list` failed to run.");
            let found = String::from_utf8(out.stdout).expect("Failed to parse strings");
            let found_pkgs = found
                .split("\n")
                .filter_map(|pkg| {
                    let chunks = pkg.split_whitespace().take(4).collect::<Vec<&str>>();
                    if let [position, _, _, store_path] = chunks[..] {
                        let keep = packages.iter().any(|p| store_path.contains(&p.name));
                        if !keep {
                            return None;
                        }
                        // let pos = position.trim().to_string().parse::<i32>().expect("Not a number");
                        return Some(ListedPackage::new(
                            position.trim().to_string(),
                            store_path.to_string(),
                        ));
                    }
                    return None;
                })
                .collect::<Vec<ListedPackage>>();
            if found_pkgs.len() == 0 {
                println!("Package not found");
                exit(0);
            }
            println!("Packages found\n");
            println!("Position\tStore Path");
            for pkg in found_pkgs.iter() {
                println!("{}\t\t{}", pkg.position, pkg.store_path);
            }
            println!("\nWhat do you want to do next?");
            println!("Remove [a]ll, [n]one, or space separated positions [int]: ");
            // NEXT: Add by position[int], or by keep only [!int]
            // println!("Type your answer:");
            let mut buffer = String::new();
            let stdin = io::stdin(); // We get `Stdin` here.
            stdin
                .read_line(&mut buffer)
                .expect("Could not read user input");

            let option = buffer.to_lowercase();

            match option.trim() {
                "a" => {
                    println!("Removing all found packages...");
                    let positions = found_pkgs
                        .iter()
                        .map(|p| p.position.as_ref())
                        .collect::<Vec<&str>>();
                    let mut cmd = Command::new("nix");
                    cmd.arg("profile").arg("remove").args(&positions);
                    if is_teacher {
                        let string_cmd = command_as_string(&cmd);

                        println!("Nix command created:");
                        println!("\n\t{string_cmd}\n");
                    }
                    let child = cmd.spawn().expect("nix command failed to run.");
                    let output = child.wait_with_output().expect("Could not wait command");
                    if output.status.success() {
                        println!("{}", String::from_utf8_lossy(&output.stdout));
                    } else {
                        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                    }
                }
                "n" => {
                    println!("Chosen: none");
                    exit(0);
                }
                _ => {
                    let positions: Vec<String> = option
                        .chars()
                        .filter(|c| c.is_ascii_digit())
                        .map(String::from)
                        .collect();
                    println!("Removing packages {}...", option);

                    let mut cmd = Command::new("nix");
                    cmd.arg("profile").arg("remove").args(&positions);

                    if is_teacher {
                        let string_cmd = command_as_string(&cmd);
                        println!("Nix command created:");
                        println!("\n\t{string_cmd}\n");
                    }
                    let child = cmd.spawn().expect("nix command failed to run.");
                    let output = child.wait_with_output().expect("Could not wait command");
                    if output.status.success() {
                        println!("{}", String::from_utf8_lossy(&output.stdout));
                    } else {
                        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                    }
                    exit(1);
                }
            }
        }
        Commands::Upgrade => {
            println!("Upgrading all packages...");
            //# nix profile upgrade '.*'
            // let items = packages.clone().map_or(vec![".*".to_string()], |pkgs| {
            //     pkgs.iter()
            //         .map(|pkg| {
            //             let escapedrepo = regex::escape(&pkg.repo);
            //             let escapedpkg = regex::escape(&pkg.name);
            //             let regex = format!(".*{escapedrepo}.*{escapedpkg}.*");
            //             return regex;
            //         })
            //         .collect::<Vec<String>>()
            // });
            let items = vec![".*".to_string()];
            let mut cmd = Command::new("nix");
            cmd.arg("profile")
                .arg("upgrade")
                .args(&items)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .stdin(Stdio::null());

            if is_teacher {
                let string_cmd = command_as_string(&cmd);
                println!("Learn mode on!");
                println!("Nix command created:");
                println!("\n\t{string_cmd}\n");
            }
            let mut child = cmd.spawn().expect("nix command failed to run.");
            let pb = ProgressBar::new_spinner();
            // let pkgs = packages
            //     .map_or(vec![".*".to_string()], |pkgs| {
            //         pkgs.into_iter().map(|p| p.into()).collect::<Vec<String>>()
            //     })
            //     .join(" ");
            pb.enable_steady_tick(Duration::from_millis(300));
            pb.set_style(spinner_style.clone());
            // pb.set_message(pkgs.to_owned());
            for line in BufReader::new(child.stderr.take().unwrap()).lines() {
                let line = line.unwrap();
                let stripped_line = line.trim();
                if !stripped_line.is_empty() {
                    pb.set_message(stripped_line.to_owned());
                }
                pb.tick();
            }
            let output = child.wait_with_output().expect("Could not wait command");
            if output.status.success() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            } else {
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Commands::List => {
            let mut cmd = Command::new("nix");
            cmd.arg("profile")
                .arg("list")
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .stdin(Stdio::null());
            if is_teacher {
                let string_cmd = command_as_string(&cmd);
                println!("Learn mode on!");
                println!("Nix command created:");
                println!("\n\t{string_cmd}\n");
            }
            let output = cmd.output().expect("nix failed to run.");
            if output.status.success() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            } else {
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }
    }
}

fn command_as_string(cmd: &Command) -> String {
    let program = &cmd.get_program().to_string_lossy();
    let args = &cmd
        .get_args()
        .into_iter()
        .map(OsStr::to_string_lossy)
        .collect::<Vec<Cow<str>>>()
        .join(" ");

    format!("{program} {args}")
}

#[cfg(test)]
mod test {
    use crate::Package;

    #[test]
    fn test_package_from() {
        let items = [
            (
                "nixpkgs#htop",
                Package {
                    repo: "nixpkgs".into(),
                    name: "htop".into(),
                    fullname: "nixpkgs#htop".into(),
                },
            ),
            (
                "htop",
                Package {
                    repo: "nixpkgs".into(),
                    name: "htop".into(),
                    fullname: "nixpkgs#htop".into(),
                },
            ),
            (
                "nixpkgs/release-20.09#hello",
                Package {
                    repo: "nixpkgs/release-20.09".into(),
                    name: "hello".into(),
                    fullname: "nixpkgs/release-20.09#hello".into(),
                },
            ),
            (
                "nixpkgs/d73407e8e6002646acfdef0e39ace088bacc83da#hello",
                Package {
                    repo: "nixpkgs/d73407e8e6002646acfdef0e39ace088bacc83da".into(),
                    name: "hello".into(),
                    fullname: "nixpkgs/d73407e8e6002646acfdef0e39ace088bacc83da#hello".into(),
                },
            ),
            (
                "nixpkgs#bash^man",
                Package {
                    repo: "nixpkgs".into(),
                    name: "bash^man".into(),
                    fullname: "nixpkgs#bash^man".into(),
                },
            ),
        ];
        for (input, expected) in items.into_iter() {
            let p = Package::from(input);
            assert_eq!(p, expected)
        }
    }
}
