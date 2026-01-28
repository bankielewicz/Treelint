#!/bin/bash
# scripts/backup_database.sh
# Automated database backup script for production deployments
# Part of DevForgeAI release workflow (Phase 3: Production Deployment)

set -euo pipefail

# Script metadata
SCRIPT_VERSION="1.0.0"
SCRIPT_NAME="backup_database.sh"

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default configuration
BACKUP_DIR="devforgeai/backups/database"
TIMESTAMP=$(date +%Y%m%d-%H%M%S)
RETENTION_DAYS=30

# Parse arguments
ENVIRONMENT=""
DB_TYPE=""
DB_HOST=""
DB_PORT=""
DB_NAME=""
DB_USER=""
DB_PASSWORD=""
BACKUP_PATH=""
COMPRESSION="gzip"
VERBOSE=false

# Usage information
usage() {
    cat <<EOF
Usage: $SCRIPT_NAME --environment <env> [options]

Required:
  --environment <env>       Target environment (staging, production)

Optional:
  --db-type <type>         Database type (postgres, mysql, mongodb, sqlserver)
                           (default: auto-detect from tech-stack.md)
  --db-host <host>         Database host (default: from config)
  --db-port <port>         Database port (default: from config)
  --db-name <name>         Database name (default: from config)
  --db-user <user>         Database user (default: from config)
  --db-password <pass>     Database password (default: from env var)
  --backup-dir <path>      Backup directory (default: devforgeai/backups/database)
  --retention-days <days>  Backup retention in days (default: 30)
  --compression <type>     Compression type (gzip, bzip2, none) (default: gzip)
  --verbose                Enable verbose output
  --help                   Display this help message

Environment Variables:
  DB_PASSWORD              Database password (recommended over --db-password)
  AWS_ACCESS_KEY_ID        AWS credentials (for RDS backups)
  AWS_SECRET_ACCESS_KEY    AWS credentials (for RDS backups)
  AZURE_STORAGE_ACCOUNT    Azure credentials (for Azure SQL backups)

Examples:
  # Basic production backup (auto-detect from config)
  $SCRIPT_NAME --environment production

  # PostgreSQL backup with custom settings
  $SCRIPT_NAME --environment production --db-type postgres --retention-days 60

  # MySQL backup with verbose output
  $SCRIPT_NAME --environment staging --db-type mysql --verbose

Exit Codes:
  0 - Backup completed successfully
  1 - Backup failed or validation error
  2 - Configuration error

EOF
    exit 0
}

# Logging functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

log_verbose() {
    if [ "$VERBOSE" = true ]; then
        echo -e "[DEBUG] $1"
    fi
}

# Parse command-line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --environment)
            ENVIRONMENT="$2"
            shift 2
            ;;
        --db-type)
            DB_TYPE="$2"
            shift 2
            ;;
        --db-host)
            DB_HOST="$2"
            shift 2
            ;;
        --db-port)
            DB_PORT="$2"
            shift 2
            ;;
        --db-name)
            DB_NAME="$2"
            shift 2
            ;;
        --db-user)
            DB_USER="$2"
            shift 2
            ;;
        --db-password)
            DB_PASSWORD="$2"
            shift 2
            ;;
        --backup-dir)
            BACKUP_DIR="$2"
            shift 2
            ;;
        --retention-days)
            RETENTION_DAYS="$2"
            shift 2
            ;;
        --compression)
            COMPRESSION="$2"
            shift 2
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        --help)
            usage
            ;;
        *)
            log_error "Unknown argument: $1"
            usage
            ;;
    esac
done

# Validate required arguments
if [ -z "$ENVIRONMENT" ]; then
    log_error "Missing required argument: --environment"
    exit 2
fi

# Validate environment
if [[ ! "$ENVIRONMENT" =~ ^(staging|production)$ ]]; then
    log_error "Invalid environment: $ENVIRONMENT (must be 'staging' or 'production')"
    exit 2
fi

log_info "Starting database backup for environment: $ENVIRONMENT"

# Create backup directory
mkdir -p "$BACKUP_DIR"
log_verbose "Backup directory: $BACKUP_DIR"

# Load database configuration from deployment config
CONFIG_FILE="devforgeai/deployment/config.json"
if [ -f "$CONFIG_FILE" ]; then
    log_verbose "Loading configuration from $CONFIG_FILE"

    # Auto-detect database type if not specified
    if [ -z "$DB_TYPE" ]; then
        DB_TYPE=$(jq -r ".environments.${ENVIRONMENT}.database.type // \"postgres\"" "$CONFIG_FILE")
        log_verbose "Auto-detected database type: $DB_TYPE"
    fi

    # Load database connection details
    if [ -z "$DB_HOST" ]; then
        DB_HOST=$(jq -r ".environments.${ENVIRONMENT}.database.host // \"localhost\"" "$CONFIG_FILE")
    fi
    if [ -z "$DB_PORT" ]; then
        DB_PORT=$(jq -r ".environments.${ENVIRONMENT}.database.port // \"\"" "$CONFIG_FILE")
    fi
    if [ -z "$DB_NAME" ]; then
        DB_NAME=$(jq -r ".environments.${ENVIRONMENT}.database.name // \"app_db\"" "$CONFIG_FILE")
    fi
    if [ -z "$DB_USER" ]; then
        DB_USER=$(jq -r ".environments.${ENVIRONMENT}.database.user // \"app_user\"" "$CONFIG_FILE")
    fi
else
    log_warn "Configuration file not found: $CONFIG_FILE"
    log_warn "Using default values or command-line arguments"

    # Set defaults if not provided
    DB_TYPE=${DB_TYPE:-"postgres"}
    DB_HOST=${DB_HOST:-"localhost"}
    DB_NAME=${DB_NAME:-"app_db"}
    DB_USER=${DB_USER:-"app_user"}
fi

# Get database password from environment variable if not provided
if [ -z "$DB_PASSWORD" ] && [ -n "${DB_PASSWORD_ENV:-}" ]; then
    DB_PASSWORD="$DB_PASSWORD_ENV"
fi

# Set default ports if not specified
case "$DB_TYPE" in
    postgres|postgresql)
        DB_PORT=${DB_PORT:-5432}
        ;;
    mysql)
        DB_PORT=${DB_PORT:-3306}
        ;;
    mongodb)
        DB_PORT=${DB_PORT:-27017}
        ;;
    sqlserver|mssql)
        DB_PORT=${DB_PORT:-1433}
        ;;
    *)
        log_error "Unsupported database type: $DB_TYPE"
        exit 2
        ;;
esac

log_verbose "Database configuration:"
log_verbose "  Type: $DB_TYPE"
log_verbose "  Host: $DB_HOST"
log_verbose "  Port: $DB_PORT"
log_verbose "  Database: $DB_NAME"
log_verbose "  User: $DB_USER"

# Generate backup filename
BACKUP_FILENAME="${ENVIRONMENT}_${DB_NAME}_${TIMESTAMP}"
BACKUP_PATH="${BACKUP_DIR}/${BACKUP_FILENAME}"

# Perform database-specific backup
backup_postgres() {
    log_info "Creating PostgreSQL backup..."

    # Check if pg_dump is available
    if ! command -v pg_dump &> /dev/null; then
        log_error "pg_dump not found. Install PostgreSQL client tools."
        exit 1
    fi

    # Set password via environment variable (more secure than command line)
    export PGPASSWORD="$DB_PASSWORD"

    # Create backup
    case "$COMPRESSION" in
        gzip)
            BACKUP_PATH="${BACKUP_PATH}.sql.gz"
            pg_dump -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" \
                --format=plain --no-owner --no-acl | gzip > "$BACKUP_PATH"
            ;;
        bzip2)
            BACKUP_PATH="${BACKUP_PATH}.sql.bz2"
            pg_dump -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" \
                --format=plain --no-owner --no-acl | bzip2 > "$BACKUP_PATH"
            ;;
        none)
            BACKUP_PATH="${BACKUP_PATH}.sql"
            pg_dump -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" \
                --format=plain --no-owner --no-acl > "$BACKUP_PATH"
            ;;
        *)
            log_error "Unsupported compression type: $COMPRESSION"
            exit 2
            ;;
    esac

    unset PGPASSWORD
}

backup_mysql() {
    log_info "Creating MySQL backup..."

    # Check if mysqldump is available
    if ! command -v mysqldump &> /dev/null; then
        log_error "mysqldump not found. Install MySQL client tools."
        exit 1
    fi

    # Create backup
    case "$COMPRESSION" in
        gzip)
            BACKUP_PATH="${BACKUP_PATH}.sql.gz"
            mysqldump -h "$DB_HOST" -P "$DB_PORT" -u "$DB_USER" -p"$DB_PASSWORD" \
                --single-transaction --routines --triggers "$DB_NAME" | gzip > "$BACKUP_PATH"
            ;;
        bzip2)
            BACKUP_PATH="${BACKUP_PATH}.sql.bz2"
            mysqldump -h "$DB_HOST" -P "$DB_PORT" -u "$DB_USER" -p"$DB_PASSWORD" \
                --single-transaction --routines --triggers "$DB_NAME" | bzip2 > "$BACKUP_PATH"
            ;;
        none)
            BACKUP_PATH="${BACKUP_PATH}.sql"
            mysqldump -h "$DB_HOST" -P "$DB_PORT" -u "$DB_USER" -p"$DB_PASSWORD" \
                --single-transaction --routines --triggers "$DB_NAME" > "$BACKUP_PATH"
            ;;
        *)
            log_error "Unsupported compression type: $COMPRESSION"
            exit 2
            ;;
    esac
}

backup_mongodb() {
    log_info "Creating MongoDB backup..."

    # Check if mongodump is available
    if ! command -v mongodump &> /dev/null; then
        log_error "mongodump not found. Install MongoDB Database Tools."
        exit 1
    fi

    # Create backup directory for MongoDB
    MONGO_BACKUP_DIR="${BACKUP_PATH}_mongodb"
    mkdir -p "$MONGO_BACKUP_DIR"

    # Connection string
    if [ -n "$DB_PASSWORD" ]; then
        MONGO_URI="mongodb://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"
    else
        MONGO_URI="mongodb://${DB_HOST}:${DB_PORT}/${DB_NAME}"
    fi

    # Create backup
    mongodump --uri="$MONGO_URI" --out="$MONGO_BACKUP_DIR"

    # Compress if requested
    case "$COMPRESSION" in
        gzip)
            BACKUP_PATH="${BACKUP_PATH}.tar.gz"
            tar -czf "$BACKUP_PATH" -C "$BACKUP_DIR" "$(basename "$MONGO_BACKUP_DIR")"
            rm -rf "$MONGO_BACKUP_DIR"
            ;;
        bzip2)
            BACKUP_PATH="${BACKUP_PATH}.tar.bz2"
            tar -cjf "$BACKUP_PATH" -C "$BACKUP_DIR" "$(basename "$MONGO_BACKUP_DIR")"
            rm -rf "$MONGO_BACKUP_DIR"
            ;;
        none)
            BACKUP_PATH="$MONGO_BACKUP_DIR"
            ;;
        *)
            log_error "Unsupported compression type: $COMPRESSION"
            exit 2
            ;;
    esac
}

backup_sqlserver() {
    log_info "Creating SQL Server backup..."
    log_warn "SQL Server backup requires server-side execution"
    log_warn "Creating backup command for manual execution or Azure automation"

    # Generate T-SQL backup command
    BACKUP_COMMAND="BACKUP DATABASE [$DB_NAME] TO DISK = N'/var/opt/mssql/backup/${BACKUP_FILENAME}.bak' WITH COMPRESSION, STATS = 10;"

    # Save command to file
    BACKUP_SCRIPT="${BACKUP_DIR}/${BACKUP_FILENAME}_command.sql"
    echo "$BACKUP_COMMAND" > "$BACKUP_SCRIPT"

    log_info "Backup command saved to: $BACKUP_SCRIPT"
    log_info "Execute on SQL Server or use Azure SQL automated backups"

    BACKUP_PATH="$BACKUP_SCRIPT"
}

# Execute backup based on database type
case "$DB_TYPE" in
    postgres|postgresql)
        backup_postgres
        ;;
    mysql)
        backup_mysql
        ;;
    mongodb)
        backup_mongodb
        ;;
    sqlserver|mssql)
        backup_sqlserver
        ;;
    *)
        log_error "Unsupported database type: $DB_TYPE"
        exit 2
        ;;
esac

# Verify backup was created
if [ ! -e "$BACKUP_PATH" ]; then
    log_error "Backup file not created: $BACKUP_PATH"
    exit 1
fi

# Get backup size
BACKUP_SIZE=$(du -h "$BACKUP_PATH" | cut -f1)
log_info "Backup created successfully: $BACKUP_PATH (Size: $BACKUP_SIZE)"

# Create backup metadata
METADATA_FILE="${BACKUP_PATH}.meta.json"
cat > "$METADATA_FILE" <<EOF
{
  "backup_timestamp": "$TIMESTAMP",
  "environment": "$ENVIRONMENT",
  "database_type": "$DB_TYPE",
  "database_name": "$DB_NAME",
  "database_host": "$DB_HOST",
  "database_port": "$DB_PORT",
  "backup_path": "$BACKUP_PATH",
  "backup_size": "$BACKUP_SIZE",
  "compression": "$COMPRESSION",
  "script_version": "$SCRIPT_VERSION"
}
EOF

log_verbose "Backup metadata: $METADATA_FILE"

# Clean up old backups (retention policy)
log_info "Cleaning up backups older than $RETENTION_DAYS days..."
find "$BACKUP_DIR" -type f -name "${ENVIRONMENT}_${DB_NAME}_*" -mtime +$RETENTION_DAYS -delete
OLD_METADATA_COUNT=$(find "$BACKUP_DIR" -type f -name "*.meta.json" -mtime +$RETENTION_DAYS | wc -l)
find "$BACKUP_DIR" -type f -name "*.meta.json" -mtime +$RETENTION_DAYS -delete
log_verbose "Deleted $OLD_METADATA_COUNT old backup(s)"

# Summary
log_info "==================================="
log_info "Database Backup Summary"
log_info "==================================="
log_info "Environment:     $ENVIRONMENT"
log_info "Database Type:   $DB_TYPE"
log_info "Database Name:   $DB_NAME"
log_info "Backup Path:     $BACKUP_PATH"
log_info "Backup Size:     $BACKUP_SIZE"
log_info "Compression:     $COMPRESSION"
log_info "Retention:       $RETENTION_DAYS days"
log_info "==================================="
log_info "Backup completed successfully!"

exit 0
