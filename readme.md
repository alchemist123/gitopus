# GitOpus

GitOpus is a CLI tool designed for managing multiple Git accounts seamlessly. It allows users to create and manage profiles, each with its own Git configuration, making it easier to switch between different identities when working with various repositories.

## Features

- Create and manage multiple Git profiles
- Set a default profile for easy switching
- Automatically add SSH keys to the SSH agent
- Execute Git commands with the configured profile

## Installation

To install GitOpus, clone the repository and build it using Cargo:
```bash
git clone https://github.com/alchemist123/gitopus.git
cd gitopus
cargo build --release
```

## Usage

After building the project, you can run the GitOpus CLI tool:

```bash
./target/release/gitopus
```

### Commands

- **Profile**
  - `create`: Create a new profile with the specified name, email, username, SSH key path, and domain.
  - `list`: List all profiles.
  - `setdefault`: Set a profile as the default.

- **Git**
  - Execute any Git command with the configured profile.

## Example

To create a new profile:

```bash
./target/release/gitopus Profile create --name "John Doe" --email "john@example.com" --username "johndoe" --ssh_key_path "/path/to/ssh/key" --domain "example.com"
```

To list all profiles:

```bash
./target/release/gitopus Profile list
```

## Documentation

For detailed documentation, please refer to the following link: [GitOpus Documentation](https://medium.com/@rjamaltdt123/gitopus-beece90d58f1)

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## Support

If you find this project helpful and would like to support its development, consider buying me a coffee:

[![Buy Me A Coffee](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://buymeacoffee.com/amal_vs)

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
