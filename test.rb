def test_method
    puts "Hello"
    x = 5
    return x
end  # This end is fine

puts test_method(  # Missing closing parenthesis
