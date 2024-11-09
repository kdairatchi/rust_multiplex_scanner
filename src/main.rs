use colored::*;
use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::env;
use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!(
        "{}",
        "🚀 Initializing rust_multiplex_scanner..."
            .bright_green()
            .bold()
    );

    // Initialize file path for the payloads file
    let mut file_path = env::current_dir().expect("Cannot find current directory");
    file_path.push("nmap-payloads");
    println!(
        "🔍 {}: {:?}",
        "Looking for payload file at".bright_yellow(),
        file_path
    );

    // Read in the payloads file
    let mut data = String::new();
    if let Ok(file) = File::open(&file_path) {
        println!("{}", "📂 Payload file found. Reading file...".bright_green());
        let mut file_buf = BufReader::new(file);
        if let Err(e) = file_buf.read_to_string(&mut data) {
            eprintln!("{}: {:?}", "❌ Error reading file".bright_red(), e);
            return;
        } else {
            println!("{}", "✅ File read successfully.".bright_green());
        }
    } else {
        eprintln!(
            "{}",
            "⚠️ Warning: Payloads file 'nmap-payloads' not found. Continuing without it."
                .bright_yellow()
        );
        return;
    }

    // Parse the payload data into a map
    println!("{}", "📄 Parsing payload data...".bright_yellow());
    let fp_map = parse_file_data(&data);
    println!(
        "{}",
        format!("✅ Parsed file data into {} entries.", fp_map.len()).bright_green()
    );

    // Generate port-to-payload mappings
    println!(
        "{}",
        "🔗 Generating port-to-payload mappings...".bright_yellow()
    );
    let pb_linenr = ports_v(&fp_map);
    let payb_linenr = payloads_v(&fp_map);
    let map = port_payload_map(pb_linenr, payb_linenr);

    // Generate Rust code based on the parsed data
    println!(
        "{}",
        "📝 Generating code from parsed data...".bright_yellow()
    );
    generate_code(&map);

    // Indicate that the process is complete
    println!(
        "{}",
        "🎉 Port scan and code generation completed successfully."
            .bright_green()
            .bold()
    );
}

/// Parses the input file data into a BTreeMap by line numbers and content
///
/// # Arguments
///
/// * `data` - A string slice containing the file contents
///
/// # Returns
///
/// A BTreeMap where keys are line numbers and values are strings
fn parse_file_data(data: &str) -> BTreeMap<usize, String> {
    let mut fp_map: BTreeMap<usize, String> = BTreeMap::new();

    for (idx, line) in data.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue; // Skip empty lines and comments
        }

        fp_map.insert(idx + 1, line.to_string());
    }

    fp_map
}

/// Generates the output file with port-to-payload mappings
///
/// # Arguments
///
/// * `port_payload_map` - A reference to a BTreeMap mapping port numbers to payload data
fn generate_code(port_payload_map: &BTreeMap<Vec<u16>, Vec<u8>>) {
    let dest_path = PathBuf::from("src/generated.rs");

    let mut generated_code = String::new();
    generated_code.push_str("use std::collections::BTreeMap;\n");
    generated_code.push_str("use once_cell::sync::Lazy;\n\n");

    generated_code.push_str("fn generated_data() -> BTreeMap<Vec<u16>, Vec<u8>> {\n");
    generated_code.push_str("    let mut map = BTreeMap::new();\n");

    for (ports, payloads) in port_payload_map {
        generated_code.push_str("    map.insert(vec![");
        generated_code.push_str(
            &ports
                .iter()
                .map(|&p| p.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        );
        generated_code.push_str("], vec![");
        generated_code.push_str(
            &payloads
                .iter()
                .map(|&p| p.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        );
        generated_code.push_str("]);\n");
    }

    generated_code.push_str("    map\n");
    generated_code.push_str("}\n\n");

    generated_code.push_str(
        "pub static PARSED_DATA: Lazy<BTreeMap<Vec<u16>, Vec<u8>>> = Lazy::new(generated_data);\n",
    );
    generated_code.push_str("pub fn get_parsed_data() -> &'static BTreeMap<Vec<u16>, Vec<u8>> {\n");
    generated_code.push_str("    &PARSED_DATA\n");
    generated_code.push_str("}\n");

    if let Err(e) = fs::write(&dest_path, generated_code) {
        eprintln!(
            "{} {:?}: {:?}",
            "❌ Failed to write generated code to".bright_red(),
            dest_path,
            e
        );
    } else {
        println!(
            "✅ {}",
            format!("Generated code written to {:?}", dest_path).bright_green()
        );

        // Format the generated code
        if let Err(e) = Command::new("cargo").arg("fmt").output() {
            eprintln!("{}: {:?}", "❌ Failed to execute cargo fmt".bright_red(), e);
        } else {
            println!("{}", "🎨 Code formatting completed successfully.".bright_green());
        }
    }
}

/// Creates a BTreeMap of line numbers mapped to a Vec<u16> of ports
///
/// # Arguments
///
/// * `fp_map` - A reference to a BTreeMap containing the parsed file data
///
/// # Returns
///
/// A BTreeMap where keys are line numbers and values are vectors of ports
fn ports_v(fp_map: &BTreeMap<usize, String>) -> BTreeMap<usize, Vec<u16>> {
    let mut pb_linenr: BTreeMap<usize, Vec<u16>> = BTreeMap::new();

    for (&line_nr, entry) in fp_map {
        // Split the entry by spaces to get protocol, port, and payload parts
        let parts: Vec<&str> = entry.splitn(3, ' ').collect();
        if parts[0] == "icmp" {
            // ICMP protocol does not use ports
            pb_linenr.insert(line_nr, vec![]); // Empty vector for ICMP to indicate no ports
            println!("Detected ICMP entry on line {}: {:?}", line_nr, entry);
            continue;
        } else if parts.len() < 3 {
            eprintln!(
                "{} {}: {:?}",
                "⚠️ Malformed entry on line".bright_red(),
                line_nr,
                entry
            );
            continue;
        }

        let port_part = parts[1]; // Port part for non-ICMP entries

        // Split by commas to handle multiple ports or ranges
        let port_segments: Vec<&str> = port_part.split(',').collect();
        let mut port_list: Vec<u16> = Vec::new();

        for segment in port_segments {
            if segment.contains('-') {
                // Handle port ranges like "100-200"
                let range: Vec<&str> = segment.split('-').collect();
                if range.len() == 2 {
                    let start = range[0].parse::<u16>();
                    let end = range[1].parse::<u16>();

                    if let (Ok(start), Ok(end)) = (start, end) {
                        for port in start..=end {
                            port_list.push(port);
                        }
                    } else {
                        eprintln!(
                            "Error parsing port range on line {}: {:?}",
                            line_nr, segment
                        );
                    }
                }
            } else {
                // Parse single port numbers
                match segment.parse::<u16>() {
                    Ok(port) => port_list.push(port),
                    Err(e) => eprintln!("Error parsing port on line {}: {:?}", line_nr, e),
                }
            }
        }

        pb_linenr.insert(line_nr, port_list);
    }

    println!("Generated port list for {} entries.", pb_linenr.len());
    pb_linenr
}

/// Parses out the Payloads into a BTreeMap of line numbers mapped to vectors of payload bytes
///
/// # Arguments
///
/// * `fp_map` - A reference to a BTreeMap containing the parsed file data
///
/// # Returns
///
/// A BTreeMap where keys are line numbers and values are vectors of payload bytes
fn payloads_v(fp_map: &BTreeMap<usize, String>) -> BTreeMap<usize, Vec<u8>> {
    let mut payb_linenr: BTreeMap<usize, Vec<u8>> = BTreeMap::new();

    for (&line_nr, data) in fp_map {
        // Split the entry to extract the payload
        let parts: Vec<&str> = data.splitn(3, ' ').collect();
        if parts.len() < 2 {
            eprintln!(
                "{} {}: {:?}",
                "⚠️ Malformed entry on line".bright_red(),
                line_nr,
                data
            );
            continue;
        }

        let payload_str = if parts[0] == "icmp" {
            parts[1]
        } else {
            parts[2]
        };

        payb_linenr.insert(line_nr, parser(payload_str));
    }

    println!("Parsed payloads for {} entries.", payb_linenr.len());
    payb_linenr
}

/// Converts a hexadecimal string to a Vec<u8>
///
/// # Arguments
///
/// * `payload` - A string slice containing the hexadecimal payload
///
/// # Returns
///
/// A vector of bytes representing the decoded payload
fn parser(payload: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut idx = 0;
    let chars: Vec<char> = payload.chars().collect();

    while idx < chars.len() {
        if chars[idx] == '\\' && idx + 3 < chars.len() && chars[idx + 1] == 'x' {
            let hex_str = format!("{}{}", chars[idx + 2], chars[idx + 3]);
            match u8::from_str_radix(&hex_str, 16) {
                Ok(byte) => bytes.push(byte),
                Err(e) => eprintln!("Error parsing hex byte at index {}: {:?}", idx, e),
            }
            idx += 4;
        } else {
            bytes.push(chars[idx] as u8);
            idx += 1;
        }
    }

    bytes
}

/// Combines the ports BTreeMap and the Payloads BTreeMap
///
/// # Arguments
///
/// * `pb_linenr` - A BTreeMap mapping line numbers to vectors of ports
/// * `payb_linenr` - A BTreeMap mapping line numbers to vectors of payload bytes
///
/// # Returns
///
/// A BTreeMap mapping vectors of ports to vectors of payload bytes
fn port_payload_map(
    pb_linenr: BTreeMap<usize, Vec<u16>>,
    payb_linenr: BTreeMap<usize, Vec<u8>>,
) -> BTreeMap<Vec<u16>, Vec<u8>> {
    let mut ppm_fin: BTreeMap<Vec<u16>, Vec<u8>> = BTreeMap::new();

    for (line_nr, ports) in pb_linenr {
        if let Some(payload) = payb_linenr.get(&line_nr) {
            ppm_fin.insert(ports.clone(), payload.clone());
        } else {
            eprintln!("No payloads found for line {}", line_nr);
        }
    }

    println!(
        "Generated final port-payload map with {} entries.",
        ppm_fin.len()
    );
    ppm_fin
}
