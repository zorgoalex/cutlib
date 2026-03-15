mod cli_options;
mod csv_reader;
mod json_formatter;
mod json_reader;
mod text_formatter;

use libcut_core::contracts::LibCutRequest;
use libcut_core::engine::LibCutEngine;
use std::path::Path;
use std::process;

fn main() {
    let exit_code = run();
    process::exit(exit_code);
}

fn run() -> i32 {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let cli_opts = match cli_options::parse(&args) {
        Ok(opts) => opts,
        Err(msg) => {
            eprintln!("Error: {}", msg);
            println!("{}", cli_options::USAGE);
            return 1;
        }
    };

    if cli_opts.show_help {
        println!("{}", cli_options::USAGE);
        return 0;
    }

    let input_file = cli_opts.input_file.as_deref().unwrap();
    let input_path = Path::new(input_file);

    if !input_path.exists() {
        eprintln!("Error: input file not found: {}", input_file);
        return 1;
    }

    let mut request = match load_request(input_path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: {}", e.message);
            for issue in &e.issues {
                if issue.message == e.message && issue.path == "request" {
                    continue;
                }
                eprintln!("  - {}: {}", issue.path, issue.message);
            }
            return 1;
        }
    };

    apply_overrides(&mut request, &cli_opts);

    let result = match LibCutEngine::optimize(&request) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: {}", e.message);
            for issue in &e.issues {
                if issue.message == e.message && issue.path == "request" {
                    continue;
                }
                eprintln!("  - {}: {}", issue.path, issue.message);
            }
            return 1;
        }
    };

    // Print summary to stderr
    let resolved_options = match request.resolve_options() {
        Ok(opts) => opts,
        Err(e) => {
            eprintln!("Error: {}", e.message);
            return 1;
        }
    };

    let parts_list = request.parts_list();
    let total_pieces: i32 = parts_list.iter().map(|p| p.qty).sum();

    eprintln!(
        "Sheet: {}x{} mm",
        result.sheet_size.length, result.sheet_size.width
    );
    eprintln!(
        "Blade: {} mm, Padding: {} mm",
        resolved_options.blade_mm, resolved_options.padding_mm
    );
    eprintln!("Algorithm: {:?}", resolved_options.algorithm);
    eprintln!(
        "Parts: {} types, {} total pieces",
        parts_list.len(),
        total_pieces
    );

    let output = if cli_opts.format == "json" {
        json_formatter::format(&result)
    } else {
        text_formatter::format(&request, &result)
    };

    if let Some(ref output_file) = cli_opts.output_file {
        match std::fs::write(output_file, &output) {
            Ok(_) => {
                eprintln!("Results written to {}", output_file);
            }
            Err(e) => {
                eprintln!("Error: failed to write output file: {}", e);
                return 1;
            }
        }
    } else {
        print!("{}", output);
    }

    0
}

fn load_request(
    input_path: &Path,
) -> Result<LibCutRequest, libcut_core::error::LibCutValidationError> {
    let extension = input_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "json" => json_reader::read(input_path),
        "csv" => csv_reader::read(input_path),
        _ => Err(libcut_core::error::LibCutValidationError::new(
            format!(
                "Unsupported input format '.{}'. Use CSV or JSON.",
                extension
            ),
            vec![libcut_core::error::LibCutValidationIssue {
                path: "input".to_string(),
                message: format!(
                    "Unsupported input format '.{}'. Use CSV or JSON.",
                    extension
                ),
            }],
        )),
    }
}

fn apply_overrides(request: &mut LibCutRequest, cli_opts: &cli_options::CliOptions) {
    if request.sheet.is_none() {
        request.sheet = Some(Default::default());
    }
    if request.options.is_none() {
        request.options = Some(Default::default());
    }

    if let (Some(length), Some(width)) = (cli_opts.sheet_length_mm, cli_opts.sheet_width_mm) {
        let sheet = request.sheet.as_mut().unwrap();
        sheet.length = length;
        sheet.width = width;
    }

    if let Some(blade) = cli_opts.blade_mm {
        request.blade = Some(blade);
    }

    if let Some(padding) = cli_opts.padding_mm {
        request.padding = Some(padding);
    }

    if let Some(ref algorithm) = cli_opts.algorithm {
        if !algorithm.is_empty() {
            request.algorithm = Some(algorithm.clone());
        }
    }
}
