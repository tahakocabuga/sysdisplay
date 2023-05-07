# SysDisplay
A lightweight app that allows you to monitor your system's resource usage in real-time

## Preview
![curl](https://media.discordapp.net/attachments/396643431327465472/1104372425841459251/image.png?width=377&height=467)
![web](https://media.discordapp.net/attachments/396643431327465472/1104372694994124850/image.png?width=395&height=468)

## Prerequisites
- Rust (latest stable version recommended)
- Cargo

## Installation
1. Clone the repository:
```bash
git clone https://github.com/tahakocabuga/sysdisplay
cd sysdisplay
```
2. Build the project:
```bash
cargo build --release
```
3. Run the server:
```bash
cargo run --release
```
4. Access the web app in your browser:
```
http://localhost:8080
```
- Or, use `curl` in your terminal:
```bash
curl http://localhost:8080
```

