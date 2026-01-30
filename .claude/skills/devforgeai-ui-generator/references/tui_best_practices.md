# Terminal UI (TUI) Best Practices

This guide provides best practices for generating terminal-based user interfaces with formatted text, tables, and ASCII art.

---

## Overview

Terminal UIs provide text-based interfaces that work in command-line environments. They're ideal for:
- Server administration tools
- Data monitoring dashboards
- Log viewers and analysis tools
- Developer utilities and CLIs

---

## Column Alignment

Properly aligned columns improve readability dramatically.

### Basic Table Alignment

```python
def format_table(data, headers):
    """
    Format data as an aligned table.

    Args:
        data: List of dictionaries with column data
        headers: List of column header names

    Returns:
        Formatted table string
    """
    # Calculate column widths
    col_widths = {}
    for header in headers:
        # Start with header length
        col_widths[header] = len(header)

        # Check all data rows
        for row in data:
            value_length = len(str(row.get(header, '')))
            col_widths[header] = max(col_widths[header], value_length)

    # Build format string
    row_format = "  ".join(f"{{:<{col_widths[h]}}}" for h in headers)

    # Print header
    output = []
    output.append(row_format.format(*headers))
    output.append("-" * (sum(col_widths.values()) + 2 * (len(headers) - 1)))

    # Print rows
    for row in data:
        values = [str(row.get(h, '')) for h in headers]
        output.append(row_format.format(*values))

    return "\n".join(output)


# Example usage
servers = [
    {"name": "web-01", "status": "running", "cpu": "45%", "memory": "2.1 GB"},
    {"name": "db-primary", "status": "running", "cpu": "78%", "memory": "8.3 GB"},
    {"name": "cache-01", "status": "stopped", "cpu": "0%", "memory": "0 GB"},
]

print(format_table(servers, ["name", "status", "cpu", "memory"]))
```

**Output:**
```
name        status   cpu   memory
--------------------------------------
web-01      running  45%   2.1 GB
db-primary  running  78%   8.3 GB
cache-01    stopped  0%    0 GB
```

### Right-Aligned Numbers

```python
def format_table_with_alignment(data, headers, right_align=None):
    """
    Format table with custom alignment per column.

    Args:
        data: List of dictionaries
        headers: List of column names
        right_align: List of column names to right-align (typically numbers)
    """
    if right_align is None:
        right_align = []

    # Calculate column widths
    col_widths = {}
    for header in headers:
        col_widths[header] = len(header)
        for row in data:
            col_widths[header] = max(col_widths[header], len(str(row.get(header, ''))))

    # Build output
    output = []

    # Header
    header_parts = []
    for h in headers:
        if h in right_align:
            header_parts.append(f"{h:>{col_widths[h]}}")
        else:
            header_parts.append(f"{h:<{col_widths[h]}}")
    output.append("  ".join(header_parts))
    output.append("-" * (sum(col_widths.values()) + 2 * (len(headers) - 1)))

    # Rows
    for row in data:
        row_parts = []
        for h in headers:
            value = str(row.get(h, ''))
            if h in right_align:
                row_parts.append(f"{value:>{col_widths[h]}}")
            else:
                row_parts.append(f"{value:<{col_widths[h]}}")
        output.append("  ".join(row_parts))

    return "\n".join(output)


# Example with right-aligned numeric columns
transactions = [
    {"id": "TX-001", "date": "2024-01-15", "amount": 1250.00, "status": "completed"},
    {"id": "TX-002", "date": "2024-01-16", "amount": 89.99, "status": "pending"},
    {"id": "TX-003", "date": "2024-01-16", "amount": 3450.50, "status": "completed"},
]

print(format_table_with_alignment(
    transactions,
    ["id", "date", "amount", "status"],
    right_align=["amount"]
))
```

**Output:**
```
id      date         amount  status
---------------------------------------
TX-001  2024-01-15  1250.00  completed
TX-002  2024-01-16    89.99  pending
TX-003  2024-01-16  3450.50  completed
```

---

## Box-Drawing Characters

Use Unicode box-drawing characters for professional-looking tables.

### Box-Drawing Character Set

```python
# Box drawing characters
BOX_CHARS = {
    'horizontal': '─',
    'vertical': '│',
    'top_left': '┌',
    'top_right': '┐',
    'bottom_left': '└',
    'bottom_right': '┘',
    'cross': '┼',
    'top_join': '┬',
    'bottom_join': '┴',
    'left_join': '├',
    'right_join': '┤',
}
```

### Table with Box Drawing

```python
def format_box_table(data, headers):
    """
    Format table with box-drawing characters.

    Args:
        data: List of dictionaries
        headers: List of column names

    Returns:
        Formatted table with box borders
    """
    # Calculate column widths
    col_widths = {}
    for header in headers:
        col_widths[header] = len(header)
        for row in data:
            col_widths[header] = max(col_widths[header], len(str(row.get(header, ''))))

    # Build separator lines
    top_line = '┌' + '┬'.join('─' * (w + 2) for w in col_widths.values()) + '┐'
    mid_line = '├' + '┼'.join('─' * (w + 2) for w in col_widths.values()) + '┤'
    bottom_line = '└' + '┴'.join('─' * (w + 2) for w in col_widths.values()) + '┘'

    # Build output
    output = [top_line]

    # Header row
    header_parts = [f" {h:<{col_widths[h]}} " for h in headers]
    output.append('│' + '│'.join(header_parts) + '│')
    output.append(mid_line)

    # Data rows
    for row in data:
        row_parts = [f" {str(row.get(h, '')):<{col_widths[h]}} " for h in headers]
        output.append('│' + '│'.join(row_parts) + '│')

    output.append(bottom_line)

    return "\n".join(output)


# Example
servers = [
    {"name": "web-01", "status": "running", "uptime": "15d 3h"},
    {"name": "db-primary", "status": "running", "uptime": "42d 8h"},
    {"name": "cache-01", "status": "stopped", "uptime": "0d 0h"},
]

print(format_box_table(servers, ["name", "status", "uptime"]))
```

**Output:**
```
┌────────────┬─────────┬─────────┐
│ name       │ status  │ uptime  │
├────────────┼─────────┼─────────┤
│ web-01     │ running │ 15d 3h  │
│ db-primary │ running │ 42d 8h  │
│ cache-01   │ stopped │ 0d 0h   │
└────────────┴─────────┴─────────┘
```

---

## ANSI Color Codes

Use colors to highlight important information.

### Basic Colors

```python
class Colors:
    """ANSI color codes for terminal output"""
    RESET = '\033[0m'
    BOLD = '\033[1m'
    DIM = '\033[2m'
    UNDERLINE = '\033[4m'

    # Foreground colors
    BLACK = '\033[30m'
    RED = '\033[31m'
    GREEN = '\033[32m'
    YELLOW = '\033[33m'
    BLUE = '\033[34m'
    MAGENTA = '\033[35m'
    CYAN = '\033[36m'
    WHITE = '\033[37m'

    # Background colors
    BG_BLACK = '\033[40m'
    BG_RED = '\033[41m'
    BG_GREEN = '\033[42m'
    BG_YELLOW = '\033[43m'
    BG_BLUE = '\033[44m'
    BG_MAGENTA = '\033[45m'
    BG_CYAN = '\033[46m'
    BG_WHITE = '\033[47m'


def colorize(text, color):
    """Apply color to text"""
    return f"{color}{text}{Colors.RESET}"


# Example usage
print(colorize("Error: File not found", Colors.RED))
print(colorize("Success: Operation completed", Colors.GREEN))
print(colorize("Warning: Disk space low", Colors.YELLOW))
print(colorize("Info: Starting process...", Colors.CYAN))
```

### Status Indicators with Color

```python
def format_status_table(data):
    """Format table with colored status indicators"""

    def get_status_color(status):
        """Return color based on status"""
        status_lower = status.lower()
        if status_lower == 'running':
            return Colors.GREEN
        elif status_lower == 'stopped':
            return Colors.RED
        elif status_lower == 'pending':
            return Colors.YELLOW
        else:
            return Colors.RESET

    # Calculate column widths (without color codes)
    col_widths = {
        'name': max(len(s['name']) for s in data) + 2,
        'status': max(len(s['status']) for s in data) + 2,
        'uptime': max(len(s['uptime']) for s in data) + 2,
    }

    # Build output
    output = []

    # Header
    output.append(f"{'Name':<{col_widths['name']}}  {'Status':<{col_widths['status']}}  {'Uptime':<{col_widths['uptime']}}")
    output.append("-" * (sum(col_widths.values()) + 4))

    # Data rows with colored status
    for row in data:
        name = f"{row['name']:<{col_widths['name']}}"
        status_color = get_status_color(row['status'])
        status = f"{colorize(row['status'], status_color):<{col_widths['status']}}"
        uptime = f"{row['uptime']:<{col_widths['uptime']}}"

        output.append(f"{name}  {status}  {uptime}")

    return "\n".join(output)


# Example
servers = [
    {"name": "web-01", "status": "running", "uptime": "15d 3h"},
    {"name": "db-primary", "status": "running", "uptime": "42d 8h"},
    {"name": "cache-01", "status": "stopped", "uptime": "0d 0h"},
    {"name": "worker-01", "status": "pending", "uptime": "0d 0h"},
]

print(format_status_table(servers))
```

---

## Progress Bars and Indicators

### Simple Progress Bar

```python
def progress_bar(current, total, width=50, label="Progress"):
    """
    Display a progress bar in the terminal.

    Args:
        current: Current progress value
        total: Total value (100%)
        width: Width of progress bar in characters
        label: Label to display before bar
    """
    percent = (current / total) * 100
    filled = int((current / total) * width)
    bar = '█' * filled + '░' * (width - filled)

    print(f"\r{label}: [{bar}] {percent:.1f}%", end='', flush=True)

    if current >= total:
        print()  # New line when complete


# Example usage
import time

total = 100
for i in range(total + 1):
    progress_bar(i, total, label="Processing")
    time.sleep(0.05)
```

### Spinner for Indeterminate Operations

```python
import itertools
import time

def spinner(message="Loading"):
    """Display a spinner for long-running operations"""
    spinner_chars = itertools.cycle(['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'])

    for char in spinner_chars:
        print(f"\r{message} {char}", end='', flush=True)
        time.sleep(0.1)
        # In real usage, check if operation is complete and break
```

---

## Terminal Width Handling

Adapt output to terminal width for responsive displays.

```python
import shutil

def get_terminal_width():
    """Get current terminal width"""
    return shutil.get_terminal_size().columns


def truncate_text(text, max_width, suffix="..."):
    """Truncate text to fit within max width"""
    if len(text) <= max_width:
        return text
    return text[:max_width - len(suffix)] + suffix


def format_responsive_table(data, headers):
    """Format table that adapts to terminal width"""
    terminal_width = get_terminal_width()

    # Calculate minimum required width
    min_col_widths = {h: len(h) for h in headers}
    for row in data:
        for h in headers:
            min_col_widths[h] = max(min_col_widths[h], len(str(row.get(h, ''))))

    total_min_width = sum(min_col_widths.values()) + 2 * (len(headers) - 1)

    # If table fits, display normally
    if total_min_width <= terminal_width:
        return format_table(data, headers)

    # Otherwise, truncate columns proportionally
    available_width = terminal_width - 2 * (len(headers) - 1)
    col_widths = {}

    for h in headers:
        proportion = min_col_widths[h] / total_min_width
        col_widths[h] = int(available_width * proportion)

    # Build output with truncation
    output = []

    # Header
    header_parts = [truncate_text(h, col_widths[h]) for h in headers]
    output.append("  ".join(f"{h:<{col_widths[headers[i]]}}" for i, h in enumerate(header_parts)))
    output.append("-" * terminal_width)

    # Rows
    for row in data:
        row_parts = []
        for h in headers:
            value = truncate_text(str(row.get(h, '')), col_widths[h])
            row_parts.append(f"{value:<{col_widths[h]}}")
        output.append("  ".join(row_parts))

    return "\n".join(output)
```

---

## Multi-Line Cell Content

Handle cells with multiple lines of content.

```python
def format_table_with_wrapping(data, headers, col_widths=None):
    """
    Format table with automatic text wrapping in cells.

    Args:
        data: List of dictionaries
        headers: List of column names
        col_widths: Optional dict of column widths
    """
    import textwrap

    if col_widths is None:
        col_widths = {h: 20 for h in headers}  # Default width

    def wrap_cell(text, width):
        """Wrap text to fit within width"""
        return textwrap.wrap(str(text), width=width) or ['']

    # Build output
    output = []

    # Header
    header_parts = [f"{h:<{col_widths[h]}}" for h in headers]
    output.append("  ".join(header_parts))
    output.append("-" * (sum(col_widths.values()) + 2 * (len(headers) - 1)))

    # Rows
    for row in data:
        # Wrap all cells
        wrapped_cells = {}
        max_lines = 0

        for h in headers:
            wrapped_cells[h] = wrap_cell(row.get(h, ''), col_widths[h])
            max_lines = max(max_lines, len(wrapped_cells[h]))

        # Print each line of the row
        for line_num in range(max_lines):
            line_parts = []
            for h in headers:
                cell_lines = wrapped_cells[h]
                if line_num < len(cell_lines):
                    line_parts.append(f"{cell_lines[line_num]:<{col_widths[h]}}")
                else:
                    line_parts.append(" " * col_widths[h])
            output.append("  ".join(line_parts))

    return "\n".join(output)


# Example
logs = [
    {
        "timestamp": "2024-01-15 10:30:45",
        "level": "ERROR",
        "message": "Database connection failed after 3 retry attempts. Check network connectivity and database server status."
    },
    {
        "timestamp": "2024-01-15 10:31:02",
        "level": "INFO",
        "message": "Retrying connection..."
    },
]

print(format_table_with_wrapping(
    logs,
    ["timestamp", "level", "message"],
    col_widths={"timestamp": 20, "level": 8, "message": 40}
))
```

---

## Rich Library Integration

For advanced TUI features, recommend the `rich` library.

```python
from rich.console import Console
from rich.table import Table
from rich.progress import Progress

# Create console
console = Console()

# Simple table
table = Table(title="Server Status")
table.add_column("Name", style="cyan")
table.add_column("Status", style="magenta")
table.add_column("Uptime", style="green")

table.add_row("web-01", "running", "15d 3h")
table.add_row("db-primary", "running", "42d 8h")
table.add_row("cache-01", "stopped", "0d 0h")

console.print(table)

# Progress bar
with Progress() as progress:
    task = progress.add_task("[cyan]Processing...", total=100)

    while not progress.finished:
        progress.update(task, advance=1)
        time.sleep(0.05)

# Syntax highlighting
from rich.syntax import Syntax

code = '''
def hello_world():
    print("Hello, World!")
'''

syntax = Syntax(code, "python", theme="monokai", line_numbers=True)
console.print(syntax)
```

---

## Summary Checklist

When generating terminal UI components, ensure:

- [ ] Columns properly aligned (left for text, right for numbers)
- [ ] Column widths calculated dynamically based on content
- [ ] Box-drawing characters used for tables (when appropriate)
- [ ] Colors used meaningfully (green=success, red=error, yellow=warning)
- [ ] Terminal width detected and respected
- [ ] Long text truncated or wrapped appropriately
- [ ] Progress indicators provided for long operations
- [ ] Clear headers and separators for readability
- [ ] Consistent spacing between columns (typically 2 spaces)
- [ ] Status messages cleared properly (use `\r` for same-line updates)
- [ ] Consider recommending `rich` library for advanced features

These best practices ensure generated terminal UIs are readable, professional, and provide excellent user experience in command-line environments.
