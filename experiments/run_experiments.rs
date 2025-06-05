#!/usr/bin/env -S cargo +nightly -Zscript
---cargo
[package]
edition = "2024"
[dependencies]
anyhow = "1.0"
bkt = "0.8"
walkdir = "2"
---
#![feature(os_str_display)]

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::time::Duration;

use bkt;
use walkdir::WalkDir;

const EXPERIMENTS: &'static str = "uniform half_range hard beta reverse_beta";

fn find_algorithm_files() -> Vec<PathBuf> {
    WalkDir::new("../src/bandits")
        .into_iter()
        .map(|e| e.expect("Error while collecting algorithm files"))
        .filter(|e| e.path().is_file())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
        .filter(|e| {
            e.path()
                .file_name()
                .map_or(false, |filename| filename != "mod.rs")
        })
        .map(|e| e.path().to_path_buf())
        .collect()
}

fn main() -> anyhow::Result<()> {
    let cache = bkt::Bkt::create("./cache".into())?;
    let algorithm_files = find_algorithm_files();

    for experiment in EXPERIMENTS.split_ascii_whitespace() {
        let result_file = format!("{experiment}.csv");
        let mut file = File::create(result_file)?;

        for algorithm_file in algorithm_files.iter() {
            let without_extension = algorithm_file.clone().with_extension("");
            let algo_name = without_extension.file_name().expect("No filename?");

            eprintln!("{experiment}: {}", algo_name.display());

            let cmd =
                bkt::CommandDesc::new(["cargo", "run", "--release", "--color", "always", "--", experiment, &*algo_name.to_string_lossy()])
                    .with_modtime(algorithm_file);

            let (result, _age) = cache.retrieve_streaming(&cmd, Duration::from_secs(3600 * 24 * 7), std::io::stdout(), std::io::stderr())?;

            if result.exit_code() != 0 {
                return Err(anyhow::Error::msg("Non-zero exit."));
            }


            file.write_all(result.stdout())?;
        }
    }

    Ok(())
}
