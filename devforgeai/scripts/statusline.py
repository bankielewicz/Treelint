#!/usr/bin/env python3
"""
DevForgeAI Enhanced StatusLine Script (v2.0)
Uses Claude Code v2.0.65+ native context_window data
Works on Windows, Linux, macOS, and WSL

3-Line Layout:
  Line 1: [████████░░] 78% (156k/200k) ⚠️ | Opus 4.5
  Line 2: 🌿 feat/branch | STORY-085 In Dev | Sprint-5
  Line 3: ⏱️ 23min | 📊 4.2k tok/min | 💰 ~$0.42
"""

import sys
import json
import subprocess
import os
import re
import time
from pathlib import Path

# Fix Unicode encoding on Windows
if sys.platform == 'win32':
    import codecs
    sys.stdout = codecs.getwriter('utf-8')(sys.stdout.buffer, 'strict')


def run_command(cmd):
    """Run shell command and return output"""
    try:
        result = subprocess.run(
            cmd,
            shell=True,
            capture_output=True,
            text=True,
            timeout=2
        )
        return result.stdout.strip()
    except (subprocess.CalledProcessError, subprocess.TimeoutExpired):
        return ""


def get_git_branch():
    """Get current git branch if in a repo"""
    null_redirect = "2>nul" if os.name == 'nt' else "2>/dev/null"
    git_check = run_command(f"git rev-parse --git-dir {null_redirect}")
    if not git_check:
        return ""
    branch = run_command(f"git branch --show-current {null_redirect}")
    return f"🌿 {branch}" if branch else ""


def get_context_info(input_data):
    """
    Extract context from native v2.0.65 context_window object.
    Falls back to transcript parsing for older versions.
    """
    context = input_data.get('context_window', {})

    # Try native context_window first (v2.0.65+)
    if context:
        input_tokens = context.get('total_input_tokens', 0)
        output_tokens = context.get('total_output_tokens', 0)
        context_size = context.get('context_window_size', 200000)
        total_tokens = input_tokens + output_tokens
        return total_tokens, context_size, input_tokens, output_tokens

    # Fallback: Parse transcript file (older CC versions)
    return get_context_from_transcript(input_data)


def get_context_from_transcript(input_data):
    """Fallback: Extract token usage from transcript file"""
    model_name = input_data.get('model', {}).get('display_name', 'Claude')
    transcript_path = input_data.get('transcript_path', '')

    # Determine context limit based on model
    if '1M' in model_name:
        context_size = 800000
    else:
        context_size = 160000

    total_tokens = 0
    if transcript_path and os.path.exists(transcript_path):
        try:
            with open(transcript_path, 'r', encoding='utf-8') as f:
                lines = f.readlines()

            most_recent_usage = None
            for line in lines:
                try:
                    data = json.loads(line.strip())
                    if not data.get('isSidechain', False) and data.get('message', {}).get('usage'):
                        most_recent_usage = data['message']['usage']
                except:
                    continue

            if most_recent_usage:
                total_tokens = (
                    most_recent_usage.get('input_tokens', 0) +
                    most_recent_usage.get('cache_read_input_tokens', 0) +
                    most_recent_usage.get('cache_creation_input_tokens', 0)
                )
        except:
            pass

    if total_tokens == 0:
        total_tokens = 5000  # Small default for new sessions

    return total_tokens, context_size, total_tokens, 0


def get_warning_indicator(percent):
    """Return warning indicator based on context usage"""
    if percent >= 80:
        return " 🔴"  # Auto-compact imminent
    elif percent >= 70:
        return " ⚠️"  # Warning
    return ""


def build_progress_bar(percent, total_tokens, context_size):
    """Build colored progress bar with percentage"""
    percent_int = int(percent)
    filled_blocks = min(percent_int // 10, 10)
    empty_blocks = 10 - filled_blocks

    # Format tokens
    formatted_tokens = f"{total_tokens // 1000}k"
    formatted_size = f"{context_size // 1000}k"

    # Color based on usage
    if percent_int < 50:
        bar_color = "\033[38;5;114m"  # Green
    elif percent_int < 80:
        bar_color = "\033[38;5;215m"  # Orange
    else:
        bar_color = "\033[38;5;203m"  # Red

    gray_color = "\033[38;5;242m"
    text_color = "\033[38;5;250m"
    reset = "\033[0m"

    progress_bar = (
        f"{bar_color}{'█' * filled_blocks}"
        f"{gray_color}{'░' * empty_blocks}{reset} "
        f"{text_color}{percent:.1f}% ({formatted_tokens}/{formatted_size}){reset}"
    )

    return progress_bar


def get_current_story():
    """Find story with 'In Development' status"""
    # Try multiple potential story directories
    story_dirs = [
        Path(".ai_docs/Stories"),
        Path("/mnt/c/Projects/DevForgeAI2/.ai_docs/Stories"),
    ]

    for stories_dir in story_dirs:
        if not stories_dir.exists():
            continue

        try:
            for story_file in stories_dir.glob("STORY-*.story.md"):
                try:
                    content = story_file.read_text(encoding='utf-8')
                    if "status: In Development" in content:
                        # Extract id and sprint from YAML frontmatter
                        id_match = re.search(r'^id:\s*(\S+)', content, re.MULTILINE)
                        sprint_match = re.search(r'^sprint:\s*(\S+)', content, re.MULTILINE)
                        story_id = id_match.group(1) if id_match else None
                        sprint = sprint_match.group(1) if sprint_match else "Backlog"
                        return story_id, "In Dev", sprint
                except:
                    continue
        except:
            continue

    return None, None, None


def get_session_metrics(input_data, total_tokens, input_tokens, output_tokens):
    """Calculate session duration, token rate, and estimated cost"""
    transcript_path = input_data.get('transcript_path', '')

    duration_min = 0
    tokens_per_min = 0.0
    est_cost = 0.0

    if transcript_path and os.path.exists(transcript_path):
        try:
            start_time = os.path.getctime(transcript_path)
            duration_sec = time.time() - start_time
            duration_min = int(duration_sec / 60)

            if duration_min > 0:
                tokens_per_min = total_tokens / duration_min

            # Cost estimation (Opus 4.5 pricing: $15/M input, $75/M output)
            # Simplified blended rate
            input_cost = (input_tokens / 1_000_000) * 15
            output_cost = (output_tokens / 1_000_000) * 75
            est_cost = input_cost + output_cost

            # If we don't have separate input/output, use blended rate
            if output_tokens == 0 and input_tokens > 0:
                est_cost = (total_tokens / 1_000_000) * 20  # Blended estimate
        except:
            pass

    return duration_min, tokens_per_min, est_cost


def main():
    """Main statusline logic - 3 line output"""
    # Read JSON input from stdin
    try:
        input_data = json.load(sys.stdin)
    except:
        input_data = {}

    # === LINE 1: Context Progress + Model ===
    total, size, inp, out = get_context_info(input_data)
    percent = (total * 100) / size if size > 0 else 0
    progress_bar = build_progress_bar(percent, total, size)
    warning = get_warning_indicator(percent)
    model = input_data.get('model', {}).get('display_name', 'Claude')
    line1 = f"{progress_bar}{warning} | {model}"

    # === LINE 2: Git Branch + Story + Sprint ===
    git_branch = get_git_branch()
    story_id, status, sprint = get_current_story()

    parts = []
    if git_branch:
        parts.append(git_branch)
    if story_id:
        parts.append(f"{story_id} {status}")
    else:
        parts.append("No active story")
    if sprint and sprint != "Backlog":
        parts.append(sprint)

    line2 = " | ".join(parts)

    # === LINE 3: Session Metrics ===
    duration, rate, cost = get_session_metrics(input_data, total, inp, out)

    # Format metrics
    if duration > 0:
        duration_str = f"⏱️ {duration}min"
        rate_str = f"📊 {rate:.0f} tok/min" if rate > 0 else "📊 --"
        cost_str = f"💰 ~${cost:.2f}" if cost > 0 else "💰 --"
        line3 = f"{duration_str} | {rate_str} | {cost_str}"
    else:
        line3 = "⏱️ <1min | 📊 -- | 💰 --"

    # Output all 3 lines
    print(line1)
    print(line2)
    print(line3)


if __name__ == "__main__":
    main()
