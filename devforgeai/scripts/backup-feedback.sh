#!/bin/bash

# Backup feedback data weekly
# Run via cron: 0 2 * * 6 /path/to/backup-feedback.sh

BACKUP_DIR="devforgeai/backups/feedback"
SOURCE_DIR="devforgeai/feedback"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
LOG_FILE="$BACKUP_DIR/backup.log"

# Create backup directory if not exists
mkdir -p "$BACKUP_DIR"

# Check if source directory exists
if [ ! -d "$SOURCE_DIR" ]; then
    echo "$(date): ERROR - Source directory not found: $SOURCE_DIR" >> "$LOG_FILE"
    exit 1
fi

# Check available disk space (need at least 100 MB)
AVAILABLE=$(df "$BACKUP_DIR" | tail -1 | awk '{print $4}')
if [ $AVAILABLE -lt 102400 ]; then
    echo "$(date): ERROR - Insufficient disk space for backup ($AVAILABLE KB available)" >> "$LOG_FILE"
    exit 1
fi

# Create backup
echo "$(date): Starting feedback backup..." >> "$LOG_FILE"

tar -czf "$BACKUP_DIR/feedback_backup_${TIMESTAMP}.tar.gz" "$SOURCE_DIR" 2>> "$LOG_FILE"

if [ $? -eq 0 ]; then
    BACKUP_SIZE=$(du -h "$BACKUP_DIR/feedback_backup_${TIMESTAMP}.tar.gz" | cut -f1)
    echo "$(date): Backup created successfully - ${TIMESTAMP} (${BACKUP_SIZE})" >> "$LOG_FILE"
else
    echo "$(date): ERROR - Backup failed for ${TIMESTAMP}" >> "$LOG_FILE"
    exit 1
fi

# Keep only last 12 backups (12 weeks = 3 months rolling window)
BACKUP_COUNT=$(ls -1 "$BACKUP_DIR"/feedback_backup_*.tar.gz 2>/dev/null | wc -l)
if [ $BACKUP_COUNT -gt 12 ]; then
    echo "$(date): Cleaning old backups (keeping last 12)..." >> "$LOG_FILE"
    ls -t "$BACKUP_DIR"/feedback_backup_*.tar.gz | tail -n +13 | while read old_backup; do
        echo "$(date): Removing old backup: $(basename $old_backup)" >> "$LOG_FILE"
        rm "$old_backup"
    done
fi

# Verify backup integrity
tar -tzf "$BACKUP_DIR/feedback_backup_${TIMESTAMP}.tar.gz" > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "$(date): Backup integrity verified" >> "$LOG_FILE"
else
    echo "$(date): WARNING - Backup integrity check failed for ${TIMESTAMP}" >> "$LOG_FILE"
fi

# Print summary
echo ""
echo "=== Feedback Backup Summary ==="
echo "Timestamp: ${TIMESTAMP}"
echo "Backup size: ${BACKUP_SIZE:-unknown}"
echo "Total backups: $(ls -1 $BACKUP_DIR/feedback_backup_*.tar.gz 2>/dev/null | wc -l)"
echo "Log: $LOG_FILE"
echo ""

exit 0
