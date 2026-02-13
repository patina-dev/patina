// SPDX-FileCopyrightText: 2026 Steven Mosley <astrosteveo>
//
// SPDX-License-Identifier: GPL-3.0-only

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "patina", about = "Static analysis for AI-generated code patterns")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Scan files for code quality patterns
    Scan {
        /// Path to scan (file or directory)
        path: PathBuf,

        /// Output format
        #[arg(long, default_value = "terminal")]
        format: OutputFormat,

        /// Minimum severity to report (error, warn, info)
        #[arg(long, default_value = "info")]
        severity_threshold: SeverityThreshold,
    },

    /// List all available rules
    Rules {
        /// Output format
        #[arg(long, default_value = "terminal")]
        format: OutputFormat,
    },
}

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    Terminal,
    Json,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum SeverityThreshold {
    Error,
    Warn,
    Info,
}
