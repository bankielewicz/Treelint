"""Test fixture: Python classes with methods for symbol extraction testing."""

from dataclasses import dataclass


class Calculator:
    """A simple calculator class."""

    def __init__(self, initial_value: float = 0.0):
        """Initialize calculator with a value."""
        self.value = initial_value

    def add(self, x: float) -> float:
        """Add a number to the current value."""
        self.value += x
        return self.value

    def subtract(self, x: float) -> float:
        """Subtract a number from the current value."""
        self.value -= x
        return self.value

    def reset(self) -> None:
        """Reset the calculator to zero."""
        self.value = 0.0


@dataclass
class Point:
    """A 2D point."""

    x: float
    y: float

    def distance_from_origin(self) -> float:
        """Calculate distance from origin."""
        return (self.x**2 + self.y**2) ** 0.5


class _PrivateHelper:
    """A private helper class (convention)."""

    def _internal_method(self) -> None:
        """Internal method."""
        pass
