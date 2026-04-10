use anyhow::Result;
use serde::Serialize;
use tabled::{Table, Tabled};
use crate::t;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Table,
}

impl OutputFormat {
    pub fn from_json_flag(json: bool) -> Self {
        if json {
            OutputFormat::Json
        } else {
            OutputFormat::Table
        }
    }
}

pub fn print_json<T: Serialize + ?Sized>(data: &T) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(data)?);
    Ok(())
}

pub fn print_table<T: Tabled>(data: &[T]) -> Result<()> {
    if data.is_empty() {
        println!("(empty)");
    } else {
        println!("{}", Table::new(data));
    }
    Ok(())
}

pub fn print_list<T: Serialize + Tabled>(data: &[T], format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Json => print_json(data),
        OutputFormat::Table => print_table(data),
    }
}

pub fn print_single<T: Serialize>(data: &T, format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Json => print_json(data),
        OutputFormat::Table => {
            println!("{}", serde_json::to_string_pretty(data)?);
            Ok(())
        }
    }
}

pub fn print_success(msg: &str, format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Json => {
            println!(r#"{{"status":"ok","message":"{}"}}"#, msg);
        }
        OutputFormat::Table => {
            println!("{}", msg);
        }
    }
    Ok(())
}

/// 统一错误输出：根据格式输出结构化错误信息到 stderr
pub fn print_error(err: &anyhow::Error, format: OutputFormat) {
    let chain: Vec<String> = err.chain().map(|e| e.to_string()).collect();
    let root = err.root_cause().to_string();

    match format {
        OutputFormat::Json => {
            let json = serde_json::json!({
                "status": "error",
                "message": chain.first().unwrap_or(&root),
                "cause": if chain.len() > 1 { Some(&chain[1..]) } else { None },
            });
            eprintln!("{}", serde_json::to_string_pretty(&json).unwrap_or_else(|_| {
                format!(r#"{{"status":"error","message":"{}"}}"#, root)
            }));
        }
        OutputFormat::Table => {
            eprintln!("{}: {}", t!("Error", "错误"), chain.first().unwrap_or(&root));
            for cause in chain.iter().skip(1) {
                eprintln!("  {}: {}", t!("Cause", "原因"), cause);
            }
        }
    }
}
