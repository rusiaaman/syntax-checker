  - Support for multiple languages including:
  - Bash
  - C
  - C++
  - C#
  - CSS
  - Elisp (Emacs Lisp)
  - Elixir
  - Elm
  - Go
  - HTML
  - Java
  - JavaScript
  - JSON
  - Lua
  - PHP
  - Python
  - ReScript
  - Ruby
  - Rust
  - Solidity
  - TOML
  - TypeScript/TSX
```bash
git clone https://github.com/rusiaaman/syntax-checker.git
cd syntax-checker
cargo build --release
```

The binary will be available at `target/release/multi-parser`

## Usage

```bash
multi-parser <language> <file>
```

For example:
```bash
# Check a Python file
multi-parser python script.py

# Check a JavaScript file
multi-parser javascript app.js

# Check an Elm file
multi-parser elm Main.elm

# Check a Solidity smart contract
multi-parser solidity Contract.sol
```

## Examples

### Python Syntax Check
```python
# test.py
def test():
    print("Hello")
    print("Missing closing parenthesis"
    return "test"

x = test()
```

Running the checker:
```bash
$ multi-parser python test.py
Syntax errors detected:
  - Error at line 3, column 10: Type "ERROR"
  - Error at line 4, column 5: Type "ERROR"
```

### Elm Syntax Check
```elm
module Main exposing (..)

import Html exposing (text)

main =
    text "Hello World"    -- Missing parentheses
    let 
        x = 42
    in
        x + y            -- Unknown variable y
```

Running the checker:
```bash
$ multi-parser elm test.elm
Syntax errors detected:
  - Error at line 6: Missing parentheses
  - Error at line 10: Undefined variable 'y'
```

## Supported Languages and File Extensions

| Language       | Extension(s)        | Command            |
|---------------|-------------------|-------------------|
| Bash          | .sh               | bash              |
| C             | .c, .h           | c                 |
| C++           | .cpp, .hpp       | cpp               |
| C#            | .cs              | c_sharp           |
| CSS           | .css             | css               |
| Elisp         | .el              | elisp             |
| Elixir        | .ex, .exs        | elixir           |
| Elm           | .elm             | elm              |
| Go            | .go              | go                |
| HTML          | .html, .htm      | html             |
| Java          | .java            | java             |
| JavaScript    | .js              | javascript        |
| JSON          | .json            | json             |
| Lua           | .lua             | lua              |
| PHP           | .php             | php              |
| Python        | .py              | python           |
| ReScript      | .res             | rescript         |
| Ruby          | .rb              | ruby             |
| Rust          | .rs              | rust             |
| Solidity      | .sol             | solidity         |
| TOML          | .toml            | toml             |
| TypeScript    | .ts              | typescript        |
| TSX           | .tsx             | tsx              |

## How It Works

Multi-Parser uses [tree-sitter](https://tree-sitter.github.io/tree-sitter/) parsers to analyze source code and detect syntax errors. Tree-sitter is a parser generator tool and incremental parsing library that can build a concrete syntax tree for source files and efficiently update it as the source file is edited.

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