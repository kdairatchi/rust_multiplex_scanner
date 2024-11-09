
<!--<u>**The Modern Port Scanner.**</u>-->
**Fast, smart, effective.**

![Arch Linux package][badge-1] ![Built with Rust][badge-2] ![GitHub All Releases][badge-3] ![Crates.io][badge-4] ![Discord][badge-5] ![Actions][badge-6]

</div>

# 🤔 What is this?

![fast][speed-1]

The Modern Port Scanner. **Find ports quickly (3 seconds at its fastest)**. Run scripts through our scripting engine (Python, Lua, Shell supported).

# 🛠️ Installation

RustScan is in many repositories already. Install it with whatever tools you wish:

[![Packaging status](https://repology.org/badge/vertical-allrepos/rustscan.svg)](https://repology.org/project/rustscan/versions)

RustScan only officially supports Cargo installations, if you want to use that please install Rust and then `cargo install rustscan`
# ✨ Features

- Scans all 65k ports in **3 seconds**.
- Full scripting engine support. Automatically pipe results into Nmap, or use our scripts (or write your own) to do whatever you want.
- Adaptive learning. RustScan improves the more you use it. No bloated machine learning here, just basic maths.
- The usuals you would expect. IPv6, CIDR, file input and more.
- Automatically pipes ports into Nmap.


# 🔭 Why This Rust Scan?

Rust Scan is a modern take on the port scanner. Sleek & fast. All while providing extensive extendability to you.

Not to mention RustScan uses Adaptive Learning to improve itself over time, making it the best port scanner for **you**.

## 🧋 Speed

![fast][speed-1]

Speed is guaranteed via RustScan. However, if you want to run a slow scan due to stealth, that is possible too.

Firstly, let's talk code.

We have tests that check to see if RustScan is significantly slower than the previous version. If it is, the continuous integration fails, and we can't commit code to master unless we make it faster.

[HyperFine][speed-2] is used to monitor RustScan's performance over time to answer the question, "Are we getting faster? Are we getting slower?".

Every pull request is reviewed by **one** person, but more often than not, **two** people review it. We test it manually and ensure the code doesn't negatively affect performance.

[Read more here][speed-3].

## ⚙️ Extensible

![scripts][extensible-1]

### _RustScan piping results into the custom Python script_

RustScan has a new scripting engine that allows anyone to write scripts in most languages. Python, Lua, and Shell are all supported.

Want to take your found ports and pipe them into Nmap for further analysis? That's possible. Want to run `smb-enum` if SMB is found open? Possible.

The possibilities are endless -- and you can write scripts in whatever language you feel comfortable with.

[Read more here][extensible-2].

## 🌊 Adaptive

![adaptive][adaptive-1]

### _RustScan automatically fine-tunes itself to match the host OS_

RustScan has a cool set of features called "Adaptive Learning". These features "learn" about the environment you are scanning and how _you_ use RustScan to **improve itself over time**.

We use this umbrella term for any feature that fits this criterion. The list constantly changes, so [check out our wiki for more information][adaptive-learning].

## 👩‍🦯 Accessible

![fast][accessible-1]

RustScan is one of the first penetration testing tools that aims to be entirely accessible.

[Most penetration testing tools are not accessible][accessible-2], which negatively affects the whole industry.

RustScan has continuous integration testing that aims to ensure it is accessible, and we are constantly working on ways to improve our accessibility and ensure _everyone_ can use RustScan.

# 🤸 Usage

Here's an updated usage guide for `rust_multiplex_scanner`, covering basic commands, advanced options, and practical examples to help users get started and leverage the scanner's full capabilities.

---

# 🔧 Usage Guide for `rust_multiplex_scanner`

## Basic Usage

The simplest way to scan a host with specific ports:

```bash
rust_multiplex_scanner -H example.com -p 80,443
```

### Options:

- `-H`, `--host`: Specify the host or IP address to scan.
- `-p`, `--ports`: Specify a single port (e.g., `80`) or a range (e.g., `1-1024`).
- `-t`, `--timeout`: Set a timeout (in seconds) for each connection attempt (default: 5 seconds).
- `-o`, `--output`: Choose the output format (`json`, `text`, `xml`).
- `-v`, `--verbose`: Enable verbose output for detailed logging.
- `--script`: Specify a custom script to run after scanning. Supports scripts in Python, Lua, and Shell.

## Advanced Usage Examples

### 1. Scanning a Range of Ports

To scan a range of ports (e.g., `1-1024`) on a host:

```bash
rust_multiplex_scanner -H example.com -p 1-1024
```

### 2. Increasing Timeout for Slow Connections

Some servers may respond slowly. Use `--timeout` to increase the connection attempt limit.

```bash
rust_multiplex_scanner -H example.com -p 22,80,443 --timeout 10
```

### 3. Verbose Output for Debugging

Verbose mode provides additional information useful for debugging.

```bash
rust_multiplex_scanner -H example.com -p 22,80,443 -v
```

### 4. JSON Output Format

Save results in JSON format for easier parsing and integration with other tools.

```bash
rust_multiplex_scanner -H example.com -p 22,80,443 -o json
```

### 5. Running a Custom Script Post-Scan

Run a custom script after the scan. This script will receive the scan results as input, allowing for additional processing.

```bash
rust_multiplex_scanner -H example.com -p 22,80,443 --script post_scan_analysis.py
```

## Docker Usage

To run `rust_multiplex_scanner` in a Docker container:

```bash
docker run --rm -it username/rust_multiplex_scanner -H example.com -p 80,443
```

## Full Command Reference

```bash
rust_multiplex_scanner -H <host> -p <ports> [options]
```

| Option            | Description                                                             |
|-------------------|-------------------------------------------------------------------------|
| `-H`, `--host`    | Host or IP address to scan (required).                                 |
| `-p`, `--ports`   | Port(s) or range to scan (e.g., `80`, `1-1024`).                       |
| `-t`, `--timeout` | Timeout per connection attempt in seconds (default: `5`).              |
| `-o`, `--output`  | Output format: `json`, `text`, or `xml`.                               |
| `-v`, `--verbose` | Verbose mode for additional logging.                                   |
| `--script`        | Specify a script to run post-scan (e.g., `post_scan_analysis.py`).     |

---

