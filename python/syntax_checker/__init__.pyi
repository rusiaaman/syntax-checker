from typing import List, Tuple, Union
from typing_extensions import Literal, TypeAlias

# Type alias for supported file extensions
FileExtension: TypeAlias = Literal[
    "py", "js", "ts", "tsx", "jsx", "rs", "c", "h", "cpp", 
    "hpp", "sh", "css", "html", "htm", "java", "json", "go", 
    "rb", "toml", "php", "cs"
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