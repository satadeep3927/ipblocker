# IPBlocker CLI

## Overview

**IPBlocker CLI** is a lightweight command-line interface tool designed to help system administrators and developers prevent DDoS attacks by efficiently managing IP blocking and monitoring suspicious activities. It offers straightforward commands to block or unblock IPs, view blocked IP lists, and configure settings, all through a terminal-friendly interface.

This CLI tool uses configuration files to define its behavior and ensures ease of use for professionals who prefer working within a terminal environment. It integrates seamlessly with systems, providing enhanced security with minimal setup.

---

## Features

- **Scan & Block**: Automatically detect and block malicious IP addresses.
- **IP Blocking**: Block specific IP addresses with optional reasons for auditing.
- **IP Unblocking**: Remove specific IP addresses from the blocklist.
- **View Blocked IPs**: List all currently blocked IPs.
- **Custom Configurations**: Easily adapt the tool to different environments using configuration files.
- **AbuseIPDB Integration**: Utilize AbuseIPDB to detect malicious IPs based on confidence thresholds.

---

## Installation

1. Install Rust and Cargo if not already installed. Refer to [Rust's official site](https://www.rust-lang.org/) for installation instructions.
2. Clone the repository:
   ```bash
   git clone https://github.com/your-repo/ipblocker.git
   cd ipblocker
   ```
3. Build the tool:
   ```bash
   cargo build --release
   ```
4. Run the executable:
   ```bash
   ./target/release/ipblocker
   ```

---

## Usage

### Command Structure

```bash
ipblocker <COMMAND> [OPTIONS]
```

### Available Commands

#### **ScanBlock**
Scan and automatically block potentially malicious IP addresses.

```bash
ipblocker scanblock --config <path_to_config>
```

- `--config`: Path to the configuration file (default: `config.json`).

#### **Scan**
Only scan potentially malicious IP addresses without blocking.

```bash
ipblocker scan --config <path_to_config>
```

- `--config`: Path to the configuration file (default: `config.json`).

#### **Block**
Manually block a specific IP address.

```bash
ipblocker block --config <path_to_config> --ip <ip_address> [--reason <reason>]
```

- `--config`: Path to the configuration file (default: `config.json`).
- `--ip`: The IP address to block.
- `--reason`: Reason for blocking the IP (default: empty).

#### **Show**
View all blocked IP addresses.

```bash
ipblocker show --config <path_to_config>
```

- `--config`: Path to the configuration file (default: `config.json`).

#### **Unblock**
Unblock a specific IP address.

```bash
ipblocker unblock --config <path_to_config> --ip <ip_address>
```

- `--config`: Path to the configuration file (default: `config.json`).
- `--ip`: The IP address to unblock.

---

## Configuration File

The configuration file (`config.json`) defines how the tool operates. Below is an example configuration:

```json
{
  "name": "Brainium Staging",
  "database": "./ipblockerdb.db",
  "abuseip": {
    "token": "bdf47c7deffdfa4f3511533a97883f6e6883d4c0d1ea5491283e79b52d26b017b9c1ea954ccb64eb"
  },
  "whitelists": [],
  "server": {
    "conf": {
      "location": "./access/{MM}-vhost.conf",
      "template": "conf.jinja",
      "reload": "systemctl reload apache2"
    },
    "log": {
      "location": "{YYYYMMDD}-log.txt",
      "timestamp": "%d/%b/%Y:%H:%M:%S %z"
    }
  },
  "rules": [
    {
      "name": "Rate Limiter",
      "path": "xmlrpc.php",
      "type": "rate_limit_rule",
      "requests": 6,
      "window": 30
    },
    {
      "name": "Abuse Report",
      "path": "",
      "type": "abuse_report_rule",
      "confidence": 40
    }
  ]
}
```

### Key Configuration Parameters

- **name**: Identifier for the current environment (e.g., staging or production).
- **database**: Path to the database file for storing IP data.
- **abuseip.token**: API token for AbuseIPDB integration.
- **whitelists**: List of IPs to exclude from blocking.
- **server.conf**: Configuration for managing server-related rules.
- **server.log**: Path and format for server logs.
- **rules**: Custom rules for rate-limiting and abuse reporting.

---

## Example Scenarios

### Blocking a Specific IP

```bash
ipblocker block --config ./config.json --ip 192.168.1.100 --reason "Suspicious activity detected"
```

### Viewing All Blocked IPs

```bash
ipblocker show --config ./config.json
```

### Unblocking an IP

```bash
ipblocker unblock --config ./config.json --ip 192.168.1.100
```

### Automated Scan and Block

```bash
ipblocker scanblock --config ./config.json
```

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## Contributing

Contributions are welcome! Feel free to submit a pull request or open an issue to improve the tool.

---

With **IPBlocker CLI**, safeguard your servers with efficiency and simplicity. Perfect for developers and administrators committed to maintaining secure and reliable environments.
