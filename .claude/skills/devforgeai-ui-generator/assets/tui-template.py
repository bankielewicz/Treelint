#!/usr/bin/env python3
"""
Terminal UI Template

Provides formatted text output for terminal interfaces with tables and color support.

Usage:
    python tui-template.py
"""


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
    """
    Apply color to text.

    Args:
        text: Text to colorize
        color: Color code from Colors class

    Returns:
        Colorized text string
    """
    return f"{color}{text}{Colors.RESET}"


def format_table(data, headers, right_align=None):
    """
    Format data as an aligned table.

    Args:
        data: List of dictionaries with column data
        headers: List of column header names
        right_align: Optional list of column names to right-align

    Returns:
        Formatted table string
    """
    if right_align is None:
        right_align = []

    # Calculate column widths
    col_widths = {}
    for header in headers:
        col_widths[header] = len(header)
        for row in data:
            value_length = len(str(row.get(header, '')))
            col_widths[header] = max(col_widths[header], value_length)

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

    # Separator
    separator = "-" * (sum(col_widths.values()) + 2 * (len(headers) - 1))
    output.append(separator)

    # Data rows
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


def format_colored_status_table(data, headers, status_column='status'):
    """
    Format table with colored status indicators.

    Args:
        data: List of dictionaries
        headers: List of column names
        status_column: Name of column containing status

    Returns:
        Formatted table with colored status
    """
    def get_status_color(status):
        """Return color based on status value"""
        status_lower = str(status).lower()
        if status_lower in ['running', 'active', 'success', 'completed']:
            return Colors.GREEN
        elif status_lower in ['stopped', 'failed', 'error']:
            return Colors.RED
        elif status_lower in ['pending', 'warning', 'paused']:
            return Colors.YELLOW
        else:
            return Colors.RESET

    # Calculate column widths (without color codes)
    col_widths = {}
    for header in headers:
        col_widths[header] = len(header)
        for row in data:
            col_widths[header] = max(col_widths[header], len(str(row.get(header, ''))))

    # Build output
    output = []

    # Header
    header_parts = [f"{h:<{col_widths[h]}}" for h in headers]
    output.append("  ".join(header_parts))

    # Separator
    separator = "-" * (sum(col_widths.values()) + 2 * (len(headers) - 1))
    output.append(separator)

    # Data rows
    for row in data:
        row_parts = []
        for h in headers:
            value = str(row.get(h, ''))

            # Apply color to status column
            if h == status_column:
                color = get_status_color(value)
                # Pad after coloring to maintain alignment
                colored_value = colorize(value, color)
                # Note: Colored text appears longer due to ANSI codes, but we pad the original value
                row_parts.append(colored_value + " " * (col_widths[h] - len(value)))
            else:
                row_parts.append(f"{value:<{col_widths[h]}}")

        output.append("  ".join(row_parts))

    return "\n".join(output)


def print_section_header(title):
    """
    Print a formatted section header.

    Args:
        title: Section title text
    """
    print()
    print(colorize(f"{'=' * 60}", Colors.CYAN))
    print(colorize(f"{title.center(60)}", Colors.BOLD + Colors.CYAN))
    print(colorize(f"{'=' * 60}", Colors.CYAN))
    print()


def print_error(message):
    """Print error message in red."""
    print(colorize(f"ERROR: {message}", Colors.RED))


def print_success(message):
    """Print success message in green."""
    print(colorize(f"SUCCESS: {message}", Colors.GREEN))


def print_warning(message):
    """Print warning message in yellow."""
    print(colorize(f"WARNING: {message}", Colors.YELLOW))


def print_info(message):
    """Print info message in cyan."""
    print(colorize(f"INFO: {message}", Colors.CYAN))


# Example usage
def main():
    """Example demonstrating terminal UI formatting"""

    # Example data
    servers = [
        {"name": "web-01", "status": "running", "cpu": "45%", "memory": "2.1 GB"},
        {"name": "db-primary", "status": "running", "cpu": "78%", "memory": "8.3 GB"},
        {"name": "cache-01", "status": "stopped", "cpu": "0%", "memory": "0 GB"},
        {"name": "worker-01", "status": "pending", "cpu": "12%", "memory": "512 MB"},
    ]

    # Print section header
    print_section_header("Server Status Dashboard")

    # Simple table
    print(colorize("Simple Table Format:", Colors.BOLD))
    print()
    print(format_table(
        servers,
        ["name", "status", "cpu", "memory"],
        right_align=["cpu", "memory"]
    ))
    print()

    # Box table
    print(colorize("Box Drawing Table Format:", Colors.BOLD))
    print()
    print(format_box_table(servers, ["name", "status", "cpu", "memory"]))
    print()

    # Colored status table
    print(colorize("Colored Status Table Format:", Colors.BOLD))
    print()
    print(format_colored_status_table(
        servers,
        ["name", "status", "cpu", "memory"],
        status_column="status"
    ))
    print()

    # Status messages
    print_section_header("Status Messages")
    print_success("All systems operational")
    print_warning("Disk space running low on web-01")
    print_error("Connection to cache-01 failed")
    print_info("Scheduled maintenance in 2 hours")
    print()


if __name__ == "__main__":
    main()
