from typing import List, Tuple, Union
from typing_extensions import Literal, TypeAlias

# Type alias for supported file extensions
FileExtension: TypeAlias = Literal[
    "py",     # Python parser
    "js",     # JavaScript parser
    "ts",     # TypeScript parser
    "tsx",    # TypeScript parser
    "jsx",    # JavaScript parser
    "rs",     # Rust parser
    "c",      # C parser
    "h",      # C parser
    "cpp",    # C++ parser
    "hpp",    # C++ parser
    "sh",     # Bash parser
    "css",    # CSS parser
    "html",   # HTML parser
    "htm",    # HTML parser
    "java",   # Java parser
    "json",   # JSON parser
    "go",     # Go parser
    "rb",     # Ruby parser
    "toml",   # TOML parser
    "php",    # PHP parser
    "cs"      # C# parser
]

class Output:
    errors: List[Tuple[int, int]]  # [(line, column), ...] for each error
    description: str  # Detailed error descriptions
    
    def __init__(self) -> None: ...

def check_syntax(program_extension: Union[str, FileExtension], program_content: str) -> Output:
    """Check syntax of source code using tree-sitter parsers.

    Args:
        program_extension: File extension to determine the parser (e.g. 'py', 'js')
        program_content: Source code to check

    Returns:
        Output object containing:
            - errors: List of (line, column) positions where errors were found
            - description: Detailed description of each error

    Raises:
        ValueError: If program_extension is not supported
        RuntimeError: If parser fails to load or parse the content
    """
    ...