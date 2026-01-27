"""Test fixture: Simple Python functions for symbol extraction testing."""


def greet(name: str) -> str:
    """Greet a person by name."""
    return f"Hello, {name}!"


def add(a: int, b: int) -> int:
    """Add two numbers together."""
    return a + b


def calculate_area(width: float, height: float) -> float:
    """Calculate rectangle area."""
    result = width * height
    return result


async def fetch_data(url: str) -> dict:
    """Async function to fetch data."""
    return {"url": url, "data": None}
