#!/bin/bash
#
# Rollback Automation Script
#
# Automated rollback for multiple deployment platforms.
# Supports Kubernetes, Azure App Service, AWS ECS, and Docker.
#
# Usage:
#   ./rollback_automation.sh --platform kubernetes --deployment myapp --version v1.9.0 --namespace production
#
# Exit Codes:
#   0: Success - Rollback completed
#   1: Failure - Rollback failed
#
# Examples:
#   # Kubernetes rollback
#   ./rollback_automation.sh --platform kubernetes --deployment myapp --namespace production
#
#   # Kubernetes rollback to specific version
#   ./rollback_automation.sh --platform kubernetes --deployment myapp --version v1.9.0 --namespace production
#
#   # Azure App Service rollback
#   ./rollback_automation.sh --platform azure --deployment myapp --resource-group myRG
#
#   # AWS ECS rollback
#   ./rollback_automation.sh --platform aws_ecs --deployment myapp --cluster production --version 42
#
#   # Docker rollback
#   ./rollback_automation.sh --platform docker --deployment myapp --version v1.9.0
#
#   # With database rollback
#   ./rollback_automation.sh --platform kubernetes --deployment myapp --namespace production --rollback-db

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $(date '+%Y-%m-%d %H:%M:%S') - $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $(date '+%Y-%m-%d %H:%M:%S') - $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $(date '+%Y-%m-%d %H:%M:%S') - $1"
}

# Function to display usage
usage() {
    cat << EOF
Usage: $0 --platform <platform> --deployment <name> [OPTIONS]

Required Arguments:
  --platform          Deployment platform: kubernetes, azure, aws_ecs, docker
  --deployment        Deployment/application name

Optional Arguments:
  --version           Version to rollback to (default: previous)
  --namespace         Kubernetes namespace (default: production)
  --cluster           AWS ECS cluster name
  --resource-group    Azure resource group name
  --rollback-db       Also rollback database migrations (flag)
  --help              Display this help message

Examples:
  # Kubernetes rollback to previous version
  $0 --platform kubernetes --deployment myapp --namespace production

  # Kubernetes rollback to specific version
  $0 --platform kubernetes --deployment myapp --version v1.9.0 --namespace production

  # Azure App Service rollback
  $0 --platform azure --deployment myapp --resource-group myRG

  # AWS ECS rollback to specific task definition
  $0 --platform aws_ecs --deployment myapp --cluster production --version 42

  # Docker rollback
  $0 --platform docker --deployment myapp --version v1.9.0
EOF
    exit 0
}

# Parse command line arguments
PLATFORM=""
DEPLOYMENT=""
VERSION=""
NAMESPACE="production"
CLUSTER=""
RESOURCE_GROUP=""
ROLLBACK_DB=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --platform)
            PLATFORM="$2"
            shift 2
            ;;
        --deployment)
            DEPLOYMENT="$2"
            shift 2
            ;;
        --version)
            VERSION="$2"
            shift 2
            ;;
        --namespace)
            NAMESPACE="$2"
            shift 2
            ;;
        --cluster)
            CLUSTER="$2"
            shift 2
            ;;
        --resource-group)
            RESOURCE_GROUP="$2"
            shift 2
            ;;
        --rollback-db)
            ROLLBACK_DB=true
            shift
            ;;
        --help)
            usage
            ;;
        *)
            log_error "Unknown option: $1"
            usage
            ;;
    esac
done

# Validate required arguments
if [ -z "$PLATFORM" ]; then
    log_error "Platform is required"
    usage
fi

if [ -z "$DEPLOYMENT" ]; then
    log_error "Deployment name is required"
    usage
fi

# Create rollback log directory
LOG_DIR="devforgeai/releases/rollback-logs"
mkdir -p "$LOG_DIR"
LOG_FILE="$LOG_DIR/rollback-$(date '+%Y%m%d-%H%M%S').log"

log_info "Starting rollback process" | tee -a "$LOG_FILE"
log_info "Platform: $PLATFORM" | tee -a "$LOG_FILE"
log_info "Deployment: $DEPLOYMENT" | tee -a "$LOG_FILE"
log_info "Version: ${VERSION:-previous}" | tee -a "$LOG_FILE"
log_info "Log file: $LOG_FILE" | tee -a "$LOG_FILE"

# Kubernetes rollback
rollback_kubernetes() {
    log_info "Executing Kubernetes rollback..." | tee -a "$LOG_FILE"

    # Check if kubectl is available
    if ! command -v kubectl &> /dev/null; then
        log_error "kubectl not found. Please install kubectl."
        exit 1
    fi

    # Verify namespace exists
    if ! kubectl get namespace "$NAMESPACE" &> /dev/null; then
        log_error "Namespace '$NAMESPACE' not found"
        exit 1
    fi

    # Verify deployment exists
    if ! kubectl get deployment "$DEPLOYMENT" -n "$NAMESPACE" &> /dev/null; then
        log_error "Deployment '$DEPLOYMENT' not found in namespace '$NAMESPACE'"
        exit 1
    fi

    if [ -n "$VERSION" ]; then
        # Rollback to specific revision
        log_info "Rolling back to specific version: $VERSION" | tee -a "$LOG_FILE"

        # Find revision number for the version
        REVISION=$(kubectl rollout history deployment/"$DEPLOYMENT" -n "$NAMESPACE" | grep "$VERSION" | awk '{print $1}')

        if [ -z "$REVISION" ]; then
            log_error "Version $VERSION not found in rollout history"
            exit 1
        fi

        kubectl rollout undo deployment/"$DEPLOYMENT" -n "$NAMESPACE" --to-revision="$REVISION" 2>&1 | tee -a "$LOG_FILE"
    else
        # Rollback to previous revision
        log_info "Rolling back to previous version" | tee -a "$LOG_FILE"
        kubectl rollout undo deployment/"$DEPLOYMENT" -n "$NAMESPACE" 2>&1 | tee -a "$LOG_FILE"
    fi

    # Wait for rollback to complete
    log_info "Waiting for rollback to complete..." | tee -a "$LOG_FILE"
    if kubectl rollout status deployment/"$DEPLOYMENT" -n "$NAMESPACE" --timeout=5m 2>&1 | tee -a "$LOG_FILE"; then
        log_info "✓ Kubernetes rollback successful" | tee -a "$LOG_FILE"

        # Verify health after rollback
        verify_kubernetes_health
    else
        log_error "✗ Kubernetes rollback failed or timed out" | tee -a "$LOG_FILE"
        exit 1
    fi
}

# Verify Kubernetes deployment health
verify_kubernetes_health() {
    log_info "Verifying deployment health..." | tee -a "$LOG_FILE"

    # Check pod status
    READY_PODS=$(kubectl get deployment "$DEPLOYMENT" -n "$NAMESPACE" -o jsonpath='{.status.readyReplicas}')
    DESIRED_PODS=$(kubectl get deployment "$DEPLOYMENT" -n "$NAMESPACE" -o jsonpath='{.spec.replicas}')

    log_info "Pods ready: $READY_PODS/$DESIRED_PODS" | tee -a "$LOG_FILE"

    if [ "$READY_PODS" -eq "$DESIRED_PODS" ]; then
        log_info "✓ All pods ready" | tee -a "$LOG_FILE"
    else
        log_warn "⚠ Not all pods ready yet" | tee -a "$LOG_FILE"
    fi
}

# Azure App Service rollback
rollback_azure() {
    log_info "Executing Azure App Service rollback..." | tee -a "$LOG_FILE"

    # Check if az CLI is available
    if ! command -v az &> /dev/null; then
        log_error "Azure CLI (az) not found. Please install Azure CLI."
        exit 1
    fi

    if [ -z "$RESOURCE_GROUP" ]; then
        log_error "Resource group is required for Azure rollback (--resource-group)"
        exit 1
    fi

    # Swap deployment slots (if using blue-green)
    log_info "Swapping staging and production slots..." | tee -a "$LOG_FILE"

    if az webapp deployment slot swap \
        --slot staging \
        --name "$DEPLOYMENT" \
        --resource-group "$RESOURCE_GROUP" \
        --target-slot production \
        --action swap 2>&1 | tee -a "$LOG_FILE"; then
        log_info "✓ Azure slot swap successful" | tee -a "$LOG_FILE"
    else
        log_error "✗ Azure slot swap failed" | tee -a "$LOG_FILE"
        exit 1
    fi

    # Verify app is running
    APP_STATE=$(az webapp show --name "$DEPLOYMENT" --resource-group "$RESOURCE_GROUP" --query state -o tsv)
    log_info "App state: $APP_STATE" | tee -a "$LOG_FILE"

    if [ "$APP_STATE" = "Running" ]; then
        log_info "✓ Azure App Service rollback successful" | tee -a "$LOG_FILE"
    else
        log_error "✗ App not running after rollback" | tee -a "$LOG_FILE"
        exit 1
    fi
}

# AWS ECS rollback
rollback_aws_ecs() {
    log_info "Executing AWS ECS rollback..." | tee -a "$LOG_FILE"

    # Check if aws CLI is available
    if ! command -v aws &> /dev/null; then
        log_error "AWS CLI not found. Please install AWS CLI."
        exit 1
    fi

    if [ -z "$CLUSTER" ]; then
        log_error "Cluster name is required for AWS ECS rollback (--cluster)"
        exit 1
    fi

    if [ -z "$VERSION" ]; then
        log_error "Task definition version is required for AWS ECS rollback (--version)"
        exit 1
    fi

    # Update service to previous task definition
    log_info "Updating service to task definition version: $VERSION" | tee -a "$LOG_FILE"

    TASK_DEF="${DEPLOYMENT}:${VERSION}"

    if aws ecs update-service \
        --cluster "$CLUSTER" \
        --service "$DEPLOYMENT" \
        --task-definition "$TASK_DEF" 2>&1 | tee -a "$LOG_FILE"; then
        log_info "Service update initiated" | tee -a "$LOG_FILE"
    else
        log_error "✗ Service update failed" | tee -a "$LOG_FILE"
        exit 1
    fi

    # Wait for service stability
    log_info "Waiting for service to stabilize..." | tee -a "$LOG_FILE"

    if aws ecs wait services-stable \
        --cluster "$CLUSTER" \
        --services "$DEPLOYMENT" 2>&1 | tee -a "$LOG_FILE"; then
        log_info "✓ AWS ECS rollback successful" | tee -a "$LOG_FILE"
    else
        log_error "✗ Service did not stabilize" | tee -a "$LOG_FILE"
        exit 1
    fi
}

# Docker rollback
rollback_docker() {
    log_info "Executing Docker rollback..." | tee -a "$LOG_FILE"

    # Check if docker is available
    if ! command -v docker &> /dev/null; then
        log_error "Docker not found. Please install Docker."
        exit 1
    fi

    if [ -z "$VERSION" ]; then
        log_error "Version/tag is required for Docker rollback (--version)"
        exit 1
    fi

    # Stop current container
    log_info "Stopping current container: $DEPLOYMENT" | tee -a "$LOG_FILE"

    if docker stop "$DEPLOYMENT" 2>&1 | tee -a "$LOG_FILE"; then
        log_info "Container stopped" | tee -a "$LOG_FILE"
    else
        log_warn "Could not stop container (may not be running)" | tee -a "$LOG_FILE"
    fi

    # Remove current container
    docker rm "$DEPLOYMENT" 2>&1 | tee -a "$LOG_FILE" || true

    # Start container with previous version
    log_info "Starting container with version: $VERSION" | tee -a "$LOG_FILE"

    # Determine image name (assumes registry/image:version format)
    IMAGE="${DEPLOYMENT}:${VERSION}"

    if docker run -d \
        --name "$DEPLOYMENT" \
        --restart always \
        -p 80:80 \
        "$IMAGE" 2>&1 | tee -a "$LOG_FILE"; then
        log_info "✓ Docker rollback successful" | tee -a "$LOG_FILE"

        # Verify container is running
        sleep 5
        CONTAINER_STATUS=$(docker inspect -f '{{.State.Status}}' "$DEPLOYMENT")
        log_info "Container status: $CONTAINER_STATUS" | tee -a "$LOG_FILE"

        if [ "$CONTAINER_STATUS" = "running" ]; then
            log_info "✓ Container is running" | tee -a "$LOG_FILE"
        else
            log_error "✗ Container is not running" | tee -a "$LOG_FILE"
            exit 1
        fi
    else
        log_error "✗ Failed to start container" | tee -a "$LOG_FILE"
        exit 1
    fi
}

# Database rollback
rollback_database() {
    log_info "Executing database rollback..." | tee -a "$LOG_FILE"

    # Check for migration script
    MIGRATION_SCRIPT="./scripts/rollback_migration.sh"

    if [ -f "$MIGRATION_SCRIPT" ]; then
        log_info "Running migration rollback script..." | tee -a "$LOG_FILE"

        if bash "$MIGRATION_SCRIPT" 2>&1 | tee -a "$LOG_FILE"; then
            log_info "✓ Database rollback successful" | tee -a "$LOG_FILE"
        else
            log_error "✗ Database rollback failed" | tee -a "$LOG_FILE"
            exit 1
        fi
    else
        log_warn "Migration rollback script not found: $MIGRATION_SCRIPT" | tee -a "$LOG_FILE"
        log_warn "Database rollback skipped (manual intervention may be required)" | tee -a "$LOG_FILE"
    fi
}

# Execute rollback based on platform
case "$PLATFORM" in
    kubernetes)
        rollback_kubernetes
        ;;
    azure)
        rollback_azure
        ;;
    aws_ecs)
        rollback_aws_ecs
        ;;
    docker)
        rollback_docker
        ;;
    *)
        log_error "Unknown platform: $PLATFORM"
        log_error "Supported platforms: kubernetes, azure, aws_ecs, docker"
        exit 1
        ;;
esac

# Database rollback if requested
if [ "$ROLLBACK_DB" = true ]; then
    rollback_database
fi

# Success
log_info "============================================" | tee -a "$LOG_FILE"
log_info "✓ Rollback completed successfully" | tee -a "$LOG_FILE"
log_info "Platform: $PLATFORM" | tee -a "$LOG_FILE"
log_info "Deployment: $DEPLOYMENT" | tee -a "$LOG_FILE"
log_info "Version: ${VERSION:-previous}" | tee -a "$LOG_FILE"
log_info "Log file: $LOG_FILE" | tee -a "$LOG_FILE"
log_info "============================================" | tee -a "$LOG_FILE"

exit 0
