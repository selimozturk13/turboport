# Turboport V1.0.0

`turboport` is a very fast port-scanning tool written in Rust.

---

## Features

* Very fast scanning. (16383.75 ports/second)
* Specify starting and ending ports.
* Specify timeout.

---

## Usage

Basic usage:

```
turboport <host>
```

Example:

```
turboport scanme.nmap.org
```

Turboport will scan ports **1–1024** by default.

---

### Specify start and end ports

```
turboport -s 1 -e 65535 example.com
```

---

### Set timeout (milliseconds)

```
turboport -t 500 google.com
```

---

### Set Concurrency

```
turboport -c 300 scanme.nmap.org
```

---

### Full example

```
turboport -s 20 -e 9000 -t 300 -c 500 192.168.1.10 
```

---

## Command Line Options

| Flag | Long Option | Description | Default |
|------|-------------|-------------|---------|
| `-s` | `--start`   | Starting port | `1` |
| `-e` | `--end`     | Ending port | `1024` |
| `-t` | `--timeout` | Timeout in ms per connection attempt | `1000` |
| `-c` | `--concurrency` | Number of the paralel threads | `1000` |
|      | *(host)*    | Target host or IP | required |

---

## Performance

Turboport is fully asynchronous and built on `tokio`, allowing thousands of concurrent connection attempts.

Typical performance:

- ~16,000 ports/sec on local network  
- ~14,000 ports/sec on internet targets (depending on latency and firewall rules)

---

## Installation

### Downloading with `cargo`(Recomented)

```
cargo install turboport
```

This command will install turboport to system PATH so you can run it from anywhere.

---

### Build from source

```
git clone https://github.com/selimozturk13/turboport
cd turboport
cargo build --release
```

Binary will be located at:

```
target/release/turboport
```

### Install the binary system‑wide

```
sudo install -m 755 target/release/turboport /usr/local/bin/
```
This will place the turboport executable into /usr/local/bin, which is already included in the system PATH. After installation, you can run it from anywhere:

````
turboport --help
```

---

## Notes

- Use responsibly. Scanning networks without permission can be illegal.
- Firewalls and rate-limiting may affect port visibility.
- Some ports may appear filtered or “stealth” rather than open/closed.

---

## License

This project is licensed under the MIT License.
