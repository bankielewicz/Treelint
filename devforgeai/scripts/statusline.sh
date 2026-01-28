#!/bin/bash
# DevForgeAI StatusLine Script
# Archaeological Pattern: cc-sessions statusline + DevForgeAI database integration
# Adaptation: DevForgeAI-specific status with database-driven information

# Read JSON input from stdin
input=$(cat)

# Extract basic info using Python (cross-platform)
cwd=$(echo "$input" | python3 -c "import sys, json; data = json.load(sys.stdin); print(data.get('workspace', {}).get('current_dir') or data.get('cwd', ''))")
model_name=$(echo "$input" | python3 -c "import sys, json; data = json.load(sys.stdin); print(data.get('model', {}).get('display_name', 'Claude'))")

# Show git branch if in a git repo
GIT_BRANCH=""
if git rev-parse --git-dir > /dev/null 2>&1; then
    BRANCH=$(git branch --show-current 2>/dev/null)
    if [ -n "$BRANCH" ]; then
        GIT_BRANCH=" | 🌿 $BRANCH"
    fi
fi

# Calculate context usage (adapted from cc-sessions)
calculate_context() {
    transcript_path=$(echo "$input" | python3 -c "import sys, json; data = json.load(sys.stdin); print(data.get('transcript_path', ''))")
    
    # DevForgeAI context limits (adjusted for our usage)
    if [[ "$model_name" =~ "1M" ]]; then
        context_limit=800000   # 800k usable for 1M Sonnet models
    else
        context_limit=160000   # 160k usable for 200k models
    fi
    
    # Extract token usage (simplified from cc-sessions)
    if [[ -n "$transcript_path" && -f "$transcript_path" ]]; then
        total_tokens=$(python3 -c "
import sys, json

try:
    with open('$transcript_path', 'r') as f:
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
        context_length = (
            most_recent_usage.get('input_tokens', 0) +
            most_recent_usage.get('cache_read_input_tokens', 0) +
            most_recent_usage.get('cache_creation_input_tokens', 0)
        )
        print(context_length)
    else:
        print(0)
except:
    print(0)
" 2>/dev/null)
        
        progress_pct=$(python3 -c "print(f'{$total_tokens * 100 / $context_limit:.1f}')")
        progress_pct_int=$(python3 -c "print(int($total_tokens * 100 / $context_limit))")
    else
        total_tokens=17900
        progress_pct="2.2"
        progress_pct_int=2
    fi
    
    # Format and create progress bar
    formatted_tokens=$(python3 -c "print(f'{$total_tokens // 1000}k')")
    formatted_limit=$(python3 -c "print(f'{$context_limit // 1000}k')")
    
    filled_blocks=$((progress_pct_int / 10))
    if [[ $filled_blocks -gt 10 ]]; then filled_blocks=10; fi
    empty_blocks=$((10 - filled_blocks))
    
    # Color based on usage
    if [[ $progress_pct_int -lt 50 ]]; then
        bar_color="\033[38;5;114m"  # Green
    elif [[ $progress_pct_int -lt 80 ]]; then
        bar_color="\033[38;5;215m"  # Orange  
    else
        bar_color="\033[38;5;203m"  # Red
    fi
    gray_color="\033[38;5;242m"
    text_color="\033[38;5;250m"
    reset="\033[0m"
    
    progress_bar="${bar_color}"
    for ((i=0; i<filled_blocks; i++)); do progress_bar+="█"; done
    progress_bar+="${gray_color}"
    for ((i=0; i<empty_blocks; i++)); do progress_bar+="░"; done
    progress_bar+="${reset} ${text_color}${progress_pct}% (${formatted_tokens}/${formatted_limit})${reset}"
    
    echo -e "$progress_bar"
}

# Build DevForgeAI statusline
progress_info=$(calculate_context)
story_info=$(get_current_story_progress)
model_info=$(get_devforgeai_model)
evidence_info=$(count_deliverable_evidence)
db_info=$(get_database_performance)

# DevForgeAI-specific two-line statusline
# Line 1: Context progress | Current story progress
# Line 2: DAIC mode | Deliverable evidence | Database performance
echo -e "$progress_info | Model: $model_name"
echo -e "$GIT_BRANCH | $evidence_info | $db_info"