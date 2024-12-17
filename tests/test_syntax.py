import syntax_checker

# Test Python syntax
python_code = """
def test():
    print("Hello")
    print("World"  # Missing parenthesis
    return 42
"""

result = syntax_checker.check_syntax("test.py", python_code)
print("Python errors:", result.errors)

# Test JavaScript syntax
js_code = """
function test() {
    console.log("Hello");
    console.log("World"  // Missing semicolon and parenthesis
    return 42;
}
"""

result = syntax_checker.check_syntax("test.js", js_code)
print("JavaScript errors:", result.errors)