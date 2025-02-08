mod borrow;

use borrow::BorrowerInput;
use clap::{Parser, Subcommand};
use colored::Colorize;
use log::debug;
use std::time::{Duration, SystemTime};
use std::{env, fs, process::exit};
use rem_utils::compile_file;
mod error;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the repairs
    Run {
        file_name: String,
        pre_extract_file_name: String,
        mut_method_call_expr_file: String,
        caller_fn_name: String,
        callee_fn_name: String,
    },
    /// Test the borrower on inputs
    Test {},
}

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let args = Cli::parse();
    match &args.command {
        Commands::Test {} => test(),
        Commands::Run {
            file_name,
            pre_extract_file_name,
            mut_method_call_expr_file,
            caller_fn_name,
            callee_fn_name,
        } => {
            let input: BorrowerInput = BorrowerInput {
                input_code: fs::read_to_string( file_name ).unwrap(),
                unmodified_code: fs::read_to_string( pre_extract_file_name ).unwrap(),
                mut_methods_code: fs::read_to_string( mut_method_call_expr_file ).unwrap(),
                caller_fn_name: caller_fn_name.clone(),
                callee_fn_name: callee_fn_name.clone(),
            };

            let result = borrow::make_borrows( input );

            match result {
                Ok( result ) => {
                    println!("Borrower succeeded:\n{:#?}", result);
                    exit(0)
                },
                Err( error ) => {
                    println!("Borrower failed: {:#?}", error);
                    exit(1)
                }
            }
        }
    }
}

fn test() {
    let mut test_durations: Vec<Duration> = Vec::new();
    let mut successful_tests: i32 = 0;
    let mut num_tests: i32 = 0;
    let mut total_time: Duration = Duration::new(0, 0);
    let mut min_time: Duration = Duration::new(std::u64::MAX, 999_999_999);
    let mut max_time: Duration = Duration::new(0, 0);

    for file in fs::read_dir("./input").unwrap() {
        num_tests += 1;
        let test_name = file.unwrap().file_name().to_owned();
        if test_name.to_str().unwrap() == "borrow.rs" {
            continue;
        }
        let file_name = format!("./input/{}", test_name.to_str().unwrap());
        let new_file_name = format!("./output/{}", test_name.to_str().unwrap());
        let mut_method_call_expr_file: String =
            format!("./method_call_mut/{}", test_name.to_str().unwrap());
        let pre_extract_file_name: String = format!("./pre_extract/{}", test_name.to_str().unwrap());
        let callee_fn_name: &str = "bar";
        let caller_fn_name: &str = "new_foo";

        let now: SystemTime = SystemTime::now();
        let input: BorrowerInput = BorrowerInput {
            input_code: fs::read_to_string(file_name.clone()).unwrap(),
            unmodified_code: fs::read_to_string(pre_extract_file_name.clone()).unwrap(),
            mut_methods_code: fs::read_to_string(mut_method_call_expr_file.clone()).unwrap(),
            callee_fn_name: callee_fn_name.to_string(),
            caller_fn_name: caller_fn_name.to_string(),
        };
        let result = borrow::make_borrows( input );
        let time_elapsed: Duration = now.elapsed().unwrap();

        test_durations.push(time_elapsed);
        total_time += time_elapsed;
        if time_elapsed < min_time {
            min_time = time_elapsed;
        }
        if time_elapsed > max_time {
            max_time = time_elapsed;
        }

        if let Ok( ref contents ) = result {
            successful_tests += 1;
            // println!("Contents:\n {:?}", contents.clone());
            fs::write(new_file_name.clone(), contents.clone()).unwrap();
            let args: Vec<&str> = vec![];
            let mut compile_cmd = compile_file(new_file_name.as_str(), &args);
            let out = compile_cmd.output().unwrap();
            println!(
                "{}: {} in {:#?}",
                if out.status.success() && result.is_ok() {
                    format!("PASSED").green()
                } else {
                    format!("COMPILATION FAILED").magenta()
                },
                test_name.to_str().unwrap(),
                time_elapsed
            );

            // if the test path is /output/break_controller.rs, the we need to
            // delete the break_controller.exe and break_controller.pdb files
            // This is for windows
            #[cfg(windows)]
            {
                // Remove the .rs from the end of test_name
                let test_name = test_name.to_str().unwrap().split('.').collect::<Vec<&str>>()[0];
                let exe_file: String = format!("{}.exe", test_name);
                let pdb_file: String = format!("{}.pdb", test_name);
                if fs::metadata(exe_file.clone()).is_ok() {
                    fs::remove_file(exe_file.clone()).unwrap();
                }
                if fs::metadata(pdb_file.clone()).is_ok() {
                    fs::remove_file(pdb_file.clone()).unwrap();
                }
            }
            #[cfg(unix)]
            {
                // Remove the .rs from the end of test_name
                let test_name = test_name.to_str().unwrap().split('.').collect::<Vec<&str>>()[0];
                let exe_file: String = format!(".{}", test_name);
                if fs::metadata(exe_file.clone()).is_ok() {
                    fs::remove_file(exe_file.clone()).unwrap();
                }
            }
        } else {
            println!("{}: {} in {:#?}", format!("BORROWING FAILED").red(), test_name.to_str().unwrap(), time_elapsed);
            println!("------------------------------------------------------------------\n");
        }
        println!("------------------------------------------------------------------\n");
    }

    // Calculate statistics
    let average_time: Duration = if !test_durations.is_empty() {
        total_time / test_durations.len() as u32
    } else {
        Duration::new(0, 0)
    };

    println!("Test Statistics:");
    debug!("The number of successful tests ignores failures in compilation");
    println!("Number of successful tests: {} out of {}", format!("{}", successful_tests).green(), format!("{}", num_tests).blue());
    println!("Total time elapsed: {:#?}", total_time);
    println!("Average time per test: {:#?}", average_time);
    println!("Minimum time for a test: {:#?}", min_time);
    println!("Maximum time for a test: {:#?}", max_time);
}
