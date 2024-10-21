# Discord Vanity Sniper

A Rust-based tool to "snipe" or claim desired Discord vanity URLs as soon as they become available. The program continuously checks a list of vanity URLs and attempts to claim them once they are not taken.

## Features
- Asynchronous HTTP requests using `reqwest` to check if the vanity URL is available.
- Automatic claiming of available vanity URLs.
- Multi-worker support for parallel checking of vanity URLs.
- Configuration via `config.json`.

## Requirements

- Rust (version 1.53 or higher recommended)
- A valid Discord token and server ID to claim vanity URLs
- `tokio` for asynchronous operations
- `reqwest` for making HTTP requests
- `serde` and `serde_json` for handling configuration files

## Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/aurorius-products/discord-vanity-sniper.git
   cd discord-vanity-sniper
   ```

3. **Install Rust (if not already installed):**
   Follow the instructions at [rust-lang.org](https://www.rust-lang.org/) to install Rust.

4. **Build the project:**
   ```bash
   cargo build --release
   ```

5. **Configure the tool:**
   When you first run the tool, a `config.json` file will be automatically generated. You will need to fill in your Discord token, server ID, and the vanity URLs you'd like to snipe.

## Configuration
After the first run, you will find a `config.json` file in the project directory. It will look something like this:
```json
{
    "discord_token": "",
    "discord_id": "",
    "vanities": [],
    "workers_each": 1
}
```

### Parameters:
- `discord_token`: Your Discord bot token or account token.
- `discord_id`: The ID of your Discord server (guild) where you want to claim the vanity URL.
- `vanities`: A list of vanity URLs you want to snipe (e.g., `["coolvanity", "anothervanity"]`).
- `workers_each`: The number of workers to spawn for each vanity URL, for parallel execution.

## Usage

After configuring the `config.json`, you can run the project:

```bash
cargo run --release
```

The tool will continuously check the specified vanity URLs and attempt to claim them when available.

### Logs:

- If a vanity URL is available and successfully claimed, the tool will output:
  ```
  Successfully claimed discord.gg/<vanity>
  ```
- Otherwise, it will log each check with:
  ```
  Checked discord.gg/<vanity>
  ```

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request if you have any improvements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Disclaimer

This tool is intended for educational purposes only. Use it responsibly and in accordance with Discord's [Terms of Service](https://discord.com/terms).
