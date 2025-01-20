# Syntax-checker: A Multi-Language Syntax Checker

A Python library that uses Tree-sitter to check syntax errors across multiple programming languages based on Rust.

⚠️ Warning: this repo is 100% AI generated and auto tested. However, some human review and testing has been done.

## Installation
`pip install syntax_checker`

## Features

  - Support for multiple languages including:
  - Fast parsing using tree-sitter
  - Precise error reporting with line and column numbers
  - No need for language-specific toolchains
  - Easy to extend with additional language support

## Usage
The library provides a simple interface to check syntax errors in source code:

```python
import syntax_checker

# Check Python syntax
output = syntax_checker.check_syntax("py", """
def invalid_function[T](x):  # Invalid type parameter syntax
    print(\"unclosed string')  # Unclosed string
""")

# Get error positions as (line, column) tuples
print(output.errors)  # [(2, 19), (3, 11)]

# Get error descriptions
print(output.description)
# Line 2, Column 19: Syntax error: unexpected ERROR in function_definition
# Line 3, Column 11: Missing string_content in string

# Check a file without errors
output = syntax_checker.check_syntax("py", """
def valid_function(x: int) -> int:
    return x * 2
""")

print(output.errors)  # []
print(output.description)  # ""
```



## Supported Languages and File Extensions

| Language       | Extension(s)        |
|---------------|-------------------|
| Bash          | sh               |
| C             | c, h            |
| C++           | cpp, hpp        |
| C#            | cs              |
| CSS           | css             |
| Elisp         | el              |
| Elixir        | ex, exs        |
| Elm           | elm             |
| Go            | go              |
| HTML          | html, htm      |
| Java          | java           |
| JavaScript    | js              |
| JSX           | jsx             |
| JSON          | json            |
| Lua           | lua             |
| PHP           | php             |
| Python        | py              |
| ReScript      | res             |
| Ruby          | rb              |
| Rust          | rs              |
| Solidity      | sol             |
| TOML          | toml            |
| TypeScript    | ts              |
| TSX           | tsx             |

## How It Works

Syntax checker uses [tree-sitter](https://tree-sitter.github.io/tree-sitter/) parsers to analyze source code and detect syntax errors. Tree-sitter is a parser generator tool and incremental parsing library that can build a concrete syntax tree for source files and efficiently update it as the source file is edited.

The tool:
1. Determines the appropriate parser based on the specified language
2. Parses the input file
3. Traverses the syntax tree looking for ERROR nodes
4. Reports the location and type of any syntax errors found

## Contributing

Contributions are welcome! Here are some ways you can contribute:
1. Add support for new languages
2. Improve error reporting
3. Add new features
4. Report bugs
5. Improve documentation

## Notes

Some experimental language support (OCaml, QL, SystemRDL) has been temporarily disabled due to parser integration issues. Contributions to fix these parsers are welcome.

## License

MIT License - see LICENSE file for details
