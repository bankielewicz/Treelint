"""Test fixture: Malformed Python file for error handling testing."""

# This function is valid
def valid_function():
    return "I work!"

# This has a syntax error (missing colon)
def broken_function()
    return "I don't work"

# Another valid function after the error
def another_valid_function(x):
    return x * 2

# Incomplete class definition
class IncompleteClass
    pass

# Valid class after error
class ValidClass:
    def method(self):
        return True
