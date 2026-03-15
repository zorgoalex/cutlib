/// CLI options and hand-rolled argument parser.

#[derive(Debug, Clone)]
pub struct CliOptions {
    pub show_help: bool,
    pub input_file: Option<String>,
    pub output_file: Option<String>,
    pub format: String,
    pub sheet_length_mm: Option<i32>,
    pub sheet_width_mm: Option<i32>,
    pub blade_mm: Option<i32>,
    pub padding_mm: Option<i32>,
    pub algorithm: Option<String>,
}

impl Default for CliOptions {
    fn default() -> Self {
        Self {
            show_help: false,
            input_file: None,
            output_file: None,
            format: "text".to_string(),
            sheet_length_mm: None,
            sheet_width_mm: None,
            blade_mm: None,
            padding_mm: None,
            algorithm: None,
        }
    }
}

pub const USAGE: &str = "\
LibCut CLI - 2D sheet cutting optimizer

Usage: LibCutCLI -i <input> [options]

Options:
  -i, --input <file>       Input CSV/JSON file with parts list
  -o, --output <file>      Output file (default: stdout)
  -s, --sheet <LxW>        Sheet size in mm, e.g. 2440x1220
  -b, --blade <mm>         Blade/kerf width in mm (default: from input or 4)
  -p, --padding <mm>       Edge padding in mm (default: from input or 0)
  -a, --algorithm <alg>    Algorithm: length|width|optimal
  -f, --format <fmt>       Output format: text|json (default: text)
  -h, --help               Show usage info and exit";

pub fn parse(args: &[String]) -> Result<CliOptions, String> {
    if args.is_empty() {
        return Ok(CliOptions {
            show_help: true,
            ..Default::default()
        });
    }

    let mut options = CliOptions::default();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                options.show_help = true;
                return Ok(options);
            }
            "-i" | "--input" => {
                options.input_file = Some(get_value(args, &mut i, "input")?);
            }
            "-o" | "--output" => {
                options.output_file = Some(get_value(args, &mut i, "output")?);
            }
            "-a" | "--algorithm" => {
                options.algorithm = Some(get_value(args, &mut i, "algorithm")?);
            }
            "-s" | "--sheet" => {
                let val = get_value(args, &mut i, "sheet")?;
                parse_sheet(&val, &mut options)?;
            }
            "-b" | "--blade" => {
                let val = get_value(args, &mut i, "blade")?;
                options.blade_mm = Some(parse_int(&val, "blade")?);
            }
            "-p" | "--padding" => {
                let val = get_value(args, &mut i, "padding")?;
                options.padding_mm = Some(parse_int(&val, "padding")?);
            }
            "-f" | "--format" => {
                let val = get_value(args, &mut i, "format")?;
                let fmt = val.trim().to_lowercase();
                if fmt != "text" && fmt != "json" {
                    return Err(format!("Unsupported format '{}'.", fmt));
                }
                options.format = fmt;
            }
            other => {
                return Err(format!("Unknown option '{}'.", other));
            }
        }
        i += 1;
    }

    if options.input_file.is_none() || options.input_file.as_deref().unwrap_or("").is_empty() {
        return Err("Input file is required (-i).".to_string());
    }

    Ok(options)
}

fn get_value(args: &[String], index: &mut usize, option_name: &str) -> Result<String, String> {
    if *index + 1 >= args.len() {
        return Err(format!("Option '{}' requires a value.", option_name));
    }
    *index += 1;
    Ok(args[*index].clone())
}

fn parse_int(value: &str, option_name: &str) -> Result<i32, String> {
    value
        .parse::<i32>()
        .map_err(|_| format!("Option '{}' requires an integer value.", option_name))
}

fn parse_sheet(value: &str, options: &mut CliOptions) -> Result<(), String> {
    let dims: Vec<&str> = value.split(|c| c == 'x' || c == 'X' || c == '*').collect();
    if dims.len() != 2 {
        return Err("Sheet size must be in LxW format, for example 2440x1220.".to_string());
    }
    let length = dims[0]
        .parse::<i32>()
        .map_err(|_| "Sheet size must be in LxW format, for example 2440x1220.".to_string())?;
    let width = dims[1]
        .parse::<i32>()
        .map_err(|_| "Sheet size must be in LxW format, for example 2440x1220.".to_string())?;
    options.sheet_length_mm = Some(length);
    options.sheet_width_mm = Some(width);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(s: &str) -> Vec<String> {
        s.split_whitespace().map(String::from).collect()
    }

    #[test]
    fn test_parse_all_flags() {
        let a = args("-i parts.csv -o out.txt -s 2440x1220 -b 3 -p 15 -a length -f json");
        let opts = parse(&a).unwrap();
        assert_eq!(opts.input_file.as_deref(), Some("parts.csv"));
        assert_eq!(opts.output_file.as_deref(), Some("out.txt"));
        assert_eq!(opts.sheet_length_mm, Some(2440));
        assert_eq!(opts.sheet_width_mm, Some(1220));
        assert_eq!(opts.blade_mm, Some(3));
        assert_eq!(opts.padding_mm, Some(15));
        assert_eq!(opts.algorithm.as_deref(), Some("length"));
        assert_eq!(opts.format, "json");
        assert!(!opts.show_help);
    }

    #[test]
    fn test_parse_help_flag() {
        let a = args("-h");
        let opts = parse(&a).unwrap();
        assert!(opts.show_help);
    }

    #[test]
    fn test_parse_long_flags() {
        let a = args("--input order.json --sheet 3000*1500 --blade 5 --format text");
        let opts = parse(&a).unwrap();
        assert_eq!(opts.input_file.as_deref(), Some("order.json"));
        assert_eq!(opts.sheet_length_mm, Some(3000));
        assert_eq!(opts.sheet_width_mm, Some(1500));
        assert_eq!(opts.blade_mm, Some(5));
        assert_eq!(opts.format, "text");
    }

    #[test]
    fn test_reject_unsupported_format() {
        let a = args("-i parts.csv -f xml");
        let err = parse(&a).unwrap_err();
        assert!(err.contains("Unsupported format"));
        assert!(err.contains("xml"));
    }

    #[test]
    fn test_missing_input_file() {
        let a = args("-s 2440x1220");
        let err = parse(&a).unwrap_err();
        assert!(err.contains("Input file is required"));
    }

    #[test]
    fn test_unknown_option() {
        let a = args("-i foo.csv --bogus");
        let err = parse(&a).unwrap_err();
        assert!(err.contains("Unknown option"));
    }

    #[test]
    fn test_empty_args_shows_help() {
        let opts = parse(&[]).unwrap();
        assert!(opts.show_help);
    }

    #[test]
    fn test_sheet_separator_variants() {
        for sep in &["2440x1220", "2440X1220", "2440*1220"] {
            let a = vec!["-i".to_string(), "f.csv".to_string(), "-s".to_string(), sep.to_string()];
            let opts = parse(&a).unwrap();
            assert_eq!(opts.sheet_length_mm, Some(2440));
            assert_eq!(opts.sheet_width_mm, Some(1220));
        }
    }
}
