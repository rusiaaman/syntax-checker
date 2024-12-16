#[cfg(test)]
mod tests {
    use syntax_checker::check_syntax;

    #[test]
    fn test_jsx_syntax() {
        let code = r#"
function App() {
  return (
    <div className="app">
      <h1>Hello World</h1>
      <button onClick={() => alert('clicked')}>Click me</button>
    </div>
  );
}
"#;
        let result = check_syntax("jsx", code);
        assert!(result.errors.is_empty(), "JSX code should be valid but got errors: {:?}", result.description);
    }
}