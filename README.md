# Agent Parse

Parse an output file from [Agent GPT](https://agentgpt.reworkd.ai/)

## Features

- Parses an AgentGPT file containing task information and constructs a tree structure of nodes.
- Supports filtering nodes based on specified flags.
- Supports filtering nodes based on the presence of output.
- Provides a command-line interface for easy usage.

## Prerequisites

- Rust programming language (stable version)

## Installation

1. Clone the repository:

   ```shell
   git clone https://github.com/EthanJWright/agent_parse.git
   ```

2. Navigate to the project directory:

   ```shell
   cd agent_parse
   ```

3. Build the project using Cargo:

   ```shell
   cargo build --release
   ```

4. The executable will be generated in the `target/release` directory.

## Usage

The Log File Parser accepts the following command-line arguments:

```shell
cargo run -- --input=<input_file> --include_flags=<flags> [--require_output]
```

- `--input`: Specifies the path to the input log file.
- `--include_flags`: Comma-separated list of flags to filter the nodes. Nodes will be included if they contain any of the specified flags.
- `--require_output` (optional): If provided, nodes without any output will be excluded from the filtered result.

Example usage:

```shell
cargo run -- --input=input/haiku.txt --include_flags=executing,adding --require_output
```

This will parse the log file `input/haiku.txt`, filter the nodes based on the flags "executing" and "adding", and exclude nodes without any output.

## Chain with Open AI (again) 

Use Chat GPT, or an [AI Formatting Tool](https://github.com/EthanJWright/ai_format) to take the output of Agent Parse and turn it into a summarized easy to work with document.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).
