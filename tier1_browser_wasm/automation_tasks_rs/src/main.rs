//! automation_tasks_rs for member tier1_browser_wasm of authentication_database_web_ui
//! it creates the pkg files tier1_browser_wasm_bg.wasm and tier1_browser_wasm.js

use cargo_auto_lib::*;

// ANSI colors for Linux terminal
// https://github.com/shiena/ansicolor/blob/master/README.md
#[allow(dead_code)]
pub const RED: &str = "\x1b[31m";
#[allow(dead_code)]
pub const YELLOW: &str = "\x1b[33m";
#[allow(dead_code)]
pub const GREEN: &str = "\x1b[32m";
#[allow(dead_code)]
pub const RESET: &str = "\x1b[0m";


fn main() {
    exit_if_not_run_in_rust_project_root_directory();

    // get CLI arguments
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();
    match_arguments_and_call_tasks(args);
}

// region: match, help and completion

/// match arguments and call tasks functions
fn match_arguments_and_call_tasks(mut args: std::env::Args) {
    // the first argument is the user defined task: (no argument for help), build, release,...
    let arg_1 = args.next();
    match arg_1 {
        None => print_help(),
        Some(task) => {
            if &task == "completion" {
                completion();
            } else {
                println!("{YELLOW}Running automation task: {task}{RESET}");
                if &task == "build" {
                    task_build();
                } else if &task == "release" {
                    task_release();
                } else if &task == "doc" {
                    task_doc();
                } else if &task == "test" {
                    task_test();
                } else {
                    println!("{RED}Error: Task {task} is unknown.{RESET}");
                    print_help();
                }
            }
        }
    }
}

/// write a comprehensible help for user defined tasks
fn print_help() {
    println!(
        r#"
    {YELLOW}Welcome to cargo-auto !
    This program automates your custom tasks when developing a Rust project.{RESET}

    User defined tasks in automation_tasks_rs:{RESET}{GREEN}
    cargo auto build{RESET}{YELLOW} - builds the crate in debug mode, fmt, increment version{RESET}{GREEN}
    cargo auto release{RESET}{YELLOW} - builds the crate in release mode, fmt, increment version{RESET}{GREEN}
    cargo auto doc{RESET}{YELLOW} - builds the docs, copy to docs directory{RESET}{GREEN}
    cargo auto test{RESET}{YELLOW} - runs all the tests{RESET}
    © 2022 bestia.dev  MIT License github.com/bestia-dev/cargo-auto
"#
/*
cargo auto publish_to_crates_io - publish to crates.io, git tag
    (You need credentials for publishing. On crates.io get the 'access token'. Then save it locally once and forever with the command 
    ` cargo login TOKEN` use a space before the command to avoid saving the secret token in bash history.)
*/
    );
    print_examples_cmd();
}

/// all example commands in one place
fn print_examples_cmd(){
/*
    println!(r#"{YELLOW}run examples:{RESET}{GREEN}
cargo run --example example1{RESET}
"#);
*/
}

/// sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`
fn completion() {
    let args: Vec<String> = std::env::args().collect();
    let word_being_completed = args[2].as_str();
    let last_word = args[3].as_str();

    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec!["build", "release", "doc", "test"];
        // , "publish_to_crates_io"
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    /*
    // the second level if needed
    else if last_word == "new" {
        let sub_commands = vec!["x"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    */
}

// endregion: match, help and completion

// region: tasks

/// cargo build
fn task_build() {
    // let cargo_toml = CargoToml::read();
    auto_version_increment_semver_or_date();
    run_shell_command("cargo fmt");
    // wasm-pack changes something in the folder target, so the next build unnecessarily rebuilds dependencies
    // I will try to use --release to force wasm-pack to use a different folder
    run_shell_command("wasm-pack build --target web --release");
    println!(
        r#"{YELLOW}
    After `cargo auto build`, run the compiled binary, examples and/or tests
    if ok, then,{RESET}{GREEN}
cargo auto release{RESET}{YELLOW}
{RESET}"#
    );
    print_examples_cmd();
}

/// cargo build --release
fn task_release() {
    // let cargo_toml = CargoToml::read();
    auto_version_increment_semver_or_date();
    auto_cargo_toml_to_md();
    auto_lines_of_code("");

    run_shell_command("cargo fmt");
    run_shell_command("wasm-pack build --target web --release");
    println!(
        r#"{YELLOW}
    After `cargo auto release`, run the compiled binary, examples and/or tests
    if ok, then,{RESET}{GREEN}
cargo auto doc{RESET}{YELLOW}
{RESET}"#
    );
    print_examples_cmd();
}

/// cargo doc, then copies to /docs/ folder, because this is a github standard folder
fn task_doc() {
    let cargo_toml = CargoToml::read();
    auto_cargo_toml_to_md();
    auto_lines_of_code("");
    auto_plantuml(&cargo_toml.package_repository().unwrap());
    auto_md_to_doc_comments();

    run_shell_command("cargo doc --no-deps --document-private-items");
    // copy target/doc into docs/ because it is github standard
    run_shell_command("rsync -a --info=progress2 --delete-after target/doc/ docs/");
    // Create simple index.html file in docs directory
    run_shell_command(&format!(
        "echo \"<meta http-equiv=\\\"refresh\\\" content=\\\"0; url={}/index.html\\\" />\" > docs/index.html",
        cargo_toml.package_name().replace("-","_")
    ));
    run_shell_command("cargo fmt");
    // message to help user with next move
    println!(
        r#"{YELLOW}
    After `cargo auto doc`, check `docs/index.html`. If ok, then test the documentation code examples{RESET}{GREEN}
cargo auto test{RESET}{YELLOW}
{RESET}"#
    );
}

/// cargo test
fn task_test() {
    run_shell_command("cargo test");
    println!(
        r#"{YELLOW}
    After `cargo auto test`. If ok, then 
    Repeat for other workspace members.
{RESET}"#
    );
}
