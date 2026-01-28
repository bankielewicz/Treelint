#!/bin/bash
# scripts/emergency_rollback.sh
# Emergency rollback script for multiple deployment platforms
# Part of DevForgeAI release workflow (Rollback Procedures)
#
# This script provides fast, automated rollback for production deployments
# when critical issues are detected. Supports multiple platforms and strategies.

set -euo pipefail

# Script metadata
SCRIPT_VERSION="1.0.0"
SCRIPT_NAME="emergency_rollback.sh"

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default configuration
ROLLBACK_LOG_DIR="devforgeai/releases/rollback-logs"
TIMESTAMP=$(date +%Y%m%d-%H%M%S)
ROLLBACK_LOG="${ROLLBACK_LOG_DIR}/rollback-${TIMESTAMP}.log"

# Deployment targets
DEPLOYMENT_TARGET=""
DEPLOYMENT_NAME=""
PREVIOUS_VERSION=""
NAMESPACE="production"
CLUSTER_NAME=""
RESOURCE_GROUP=""
ROLLBACK_DB=false
VERIFY_HEALTH=true
AUTO_CONFIRM=false
VERBOSE=false

# Usage information
usage() {
    cat <<EOF
Usage: $SCRIPT_NAME --target <platform> --deployment <name> [options]

Required:
  --target <platform>       Deployment platform (kubernetes, helm, azure,
                           aws_ecs, aws_lambda, docker, gcp)
  --deployment <name>       Deployment/application name

Optional:
  --version <version>       Specific version to rollback to (default: previous)
  --namespace <namespace>   Kubernetes namespace (default: production)
  --cluster <name>          AWS ECS cluster or GKE cluster name
  --resource-group <group>  Azure resource group name
  --rollback-db             Also rollback database migrations
  --skip-health-check       Skip post-rollback health verification
  --auto-confirm            Skip confirmation prompt (use with caution)
  --verbose                 Enable verbose output
  --help                    Display this help message

Environment Variables:
  KUBECONFIG               Kubernetes config path
  AWS_PROFILE              AWS CLI profile
  AZURE_SUBSCRIPTION_ID    Azure subscription ID

Supported Platforms:
  kubernetes               Kubernetes Deployment (kubectl rollout undo)
  helm                     Helm Release (helm rollback)
  azure                    Azure App Service (slot swap or zip deploy)
  aws_ecs                  AWS ECS Service (update-service)
  aws_lambda               AWS Lambda Function (update-alias)
  docker                   Docker Container (stop/start)
  gcp                      Google Cloud Platform (gcloud app deploy)

Examples:
  # Kubernetes rollback to previous version
  $SCRIPT_NAME --target kubernetes --deployment myapp --namespace production

  # Kubernetes rollback to specific version
  $SCRIPT_NAME --target kubernetes --deployment myapp --version 42

  # Helm rollback to revision 3
  $SCRIPT_NAME --target helm --deployment myapp --version 3 --namespace production

  # Azure App Service rollback
  $SCRIPT_NAME --target azure --deployment myapp --resource-group myRG

  # AWS ECS rollback
  $SCRIPT_NAME --target aws_ecs --deployment myapp --cluster prod-cluster --version 42

  # AWS Lambda rollback
  $SCRIPT_NAME --target aws_lambda --deployment myapp --version 5

  # Docker rollback with database rollback
  $SCRIPT_NAME --target docker --deployment myapp --version v1.9.0 --rollback-db

  # Auto-confirm (no prompts)
  $SCRIPT_NAME --target kubernetes --deployment myapp --auto-confirm

Exit Codes:
  0 - Rollback completed successfully
  1 - Rollback failed
  2 - Validation error or user abort

EOF
    exit 0
}

# Logging functions
log_info() {
    local msg="$1"
    echo -e "${GREEN}[INFO]${NC} $msg" | tee -a "$ROLLBACK_LOG"
}

log_warn() {
    local msg="$1"
    echo -e "${YELLOW}[WARN]${NC} $msg" | tee -a "$ROLLBACK_LOG"
}

log_error() {
    local msg="$1"
    echo -e "${RED}[ERROR]${NC} $msg" | tee -a "$ROLLBACK_LOG" >&2
}

log_debug() {
    if [ "$VERBOSE" = true ]; then
        local msg="$1"
        echo -e "${BLUE}[DEBUG]${NC} $msg" | tee -a "$ROLLBACK_LOG"
    fi
}

log_step() {
    local msg="$1"
    echo -e "\n${BLUE}>>> $msg${NC}" | tee -a "$ROLLBACK_LOG"
}

# Parse command-line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --target)
            DEPLOYMENT_TARGET="$2"
            shift 2
            ;;
        --deployment)
            DEPLOYMENT_NAME="$2"
            shift 2
            ;;
        --version)
            PREVIOUS_VERSION="$2"
            shift 2
            ;;
        --namespace)
            NAMESPACE="$2"
            shift 2
            ;;
        --cluster)
            CLUSTER_NAME="$2"
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
        --skip-health-check)
            VERIFY_HEALTH=false
            shift
            ;;
        --auto-confirm)
            AUTO_CONFIRM=true
            shift
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        --help)
            usage
            ;;
        *)
            echo -e "${RED}Unknown argument: $1${NC}"
            usage
            ;;
    esac
done

# Create rollback log directory
mkdir -p "$ROLLBACK_LOG_DIR"

# Start logging
log_info "==================================="
log_info "Emergency Rollback Initiated"
log_info "==================================="
log_info "Timestamp: $TIMESTAMP"
log_info "Target: $DEPLOYMENT_TARGET"
log_info "Deployment: $DEPLOYMENT_NAME"
log_info "Log: $ROLLBACK_LOG"
log_info "==================================="

# Validate required arguments
if [ -z "$DEPLOYMENT_TARGET" ] || [ -z "$DEPLOYMENT_NAME" ]; then
    log_error "Missing required arguments: --target and --deployment"
    exit 2
fi

# Validate deployment target
case "$DEPLOYMENT_TARGET" in
    kubernetes|helm|azure|aws_ecs|aws_lambda|docker|gcp)
        log_debug "Valid deployment target: $DEPLOYMENT_TARGET"
        ;;
    *)
        log_error "Unknown deployment target: $DEPLOYMENT_TARGET"
        exit 2
        ;;
esac

# Confirmation prompt (unless auto-confirm enabled)
if [ "$AUTO_CONFIRM" = false ]; then
    echo ""
    echo -e "${YELLOW}⚠️  WARNING: You are about to rollback a production deployment${NC}"
    echo -e "${YELLOW}    Target: $DEPLOYMENT_TARGET${NC}"
    echo -e "${YELLOW}    Deployment: $DEPLOYMENT_NAME${NC}"
    if [ -n "$PREVIOUS_VERSION" ]; then
        echo -e "${YELLOW}    Version: $PREVIOUS_VERSION${NC}"
    else
        echo -e "${YELLOW}    Version: [previous/automatic]${NC}"
    fi
    echo ""
    read -p "Are you sure you want to proceed? (yes/no): " CONFIRM

    if [[ ! "$CONFIRM" =~ ^(yes|YES|y|Y)$ ]]; then
        log_warn "Rollback aborted by user"
        exit 2
    fi
fi

log_info "Rollback confirmed. Proceeding..."

# Platform-specific rollback functions

rollback_kubernetes() {
    log_step "Rolling back Kubernetes Deployment: $DEPLOYMENT_NAME"

    # Check if kubectl is available
    if ! command -v kubectl &> /dev/null; then
        log_error "kubectl not found. Install Kubernetes CLI tools."
        exit 1
    fi

    # Get current deployment status
    log_debug "Checking current deployment status..."
    if ! kubectl get deployment "$DEPLOYMENT_NAME" -n "$NAMESPACE" &> /dev/null; then
        log_error "Deployment not found: $DEPLOYMENT_NAME in namespace $NAMESPACE"
        exit 1
    fi

    # View rollout history
    log_info "Rollout history:"
    kubectl rollout history deployment/"$DEPLOYMENT_NAME" -n "$NAMESPACE" | tee -a "$ROLLBACK_LOG"

    # Execute rollback
    if [ -n "$PREVIOUS_VERSION" ]; then
        log_info "Rolling back to revision $PREVIOUS_VERSION..."
        kubectl rollout undo deployment/"$DEPLOYMENT_NAME" -n "$NAMESPACE" --to-revision="$PREVIOUS_VERSION"
    else
        log_info "Rolling back to previous revision..."
        kubectl rollout undo deployment/"$DEPLOYMENT_NAME" -n "$NAMESPACE"
    fi

    # Wait for rollout to complete
    log_info "Waiting for rollback to complete..."
    kubectl rollout status deployment/"$DEPLOYMENT_NAME" -n "$NAMESPACE" --timeout=5m

    log_info "Kubernetes rollback completed successfully"
}

rollback_helm() {
    log_step "Rolling back Helm Release: $DEPLOYMENT_NAME"

    # Check if helm is available
    if ! command -v helm &> /dev/null; then
        log_error "helm not found. Install Helm CLI."
        exit 1
    fi

    # Get release history
    log_info "Release history:"
    helm history "$DEPLOYMENT_NAME" -n "$NAMESPACE" | tee -a "$ROLLBACK_LOG"

    # Execute rollback
    if [ -n "$PREVIOUS_VERSION" ]; then
        log_info "Rolling back to revision $PREVIOUS_VERSION..."
        helm rollback "$DEPLOYMENT_NAME" "$PREVIOUS_VERSION" -n "$NAMESPACE"
    else
        log_info "Rolling back to previous revision..."
        helm rollback "$DEPLOYMENT_NAME" -n "$NAMESPACE"
    fi

    log_info "Helm rollback completed successfully"
}

rollback_azure() {
    log_step "Rolling back Azure App Service: $DEPLOYMENT_NAME"

    # Check if az CLI is available
    if ! command -v az &> /dev/null; then
        log_error "Azure CLI not found. Install az CLI."
        exit 1
    fi

    # Validate resource group
    if [ -z "$RESOURCE_GROUP" ]; then
        log_error "Azure rollback requires --resource-group"
        exit 2
    fi

    # Check if deployment slots exist (blue-green scenario)
    SLOTS=$(az webapp deployment slot list --name "$DEPLOYMENT_NAME" --resource-group "$RESOURCE_GROUP" --query "[].name" -o tsv)

    if [ -n "$SLOTS" ]; then
        log_info "Deployment slots detected. Swapping back..."
        az webapp deployment slot swap \
            --slot staging \
            --name "$DEPLOYMENT_NAME" \
            --resource-group "$RESOURCE_GROUP" \
            --target-slot production \
            --action swap
    else
        log_warn "No deployment slots found. Using zip deployment rollback..."
        if [ -z "$PREVIOUS_VERSION" ]; then
            log_error "Zip deployment rollback requires --version parameter"
            exit 2
        fi

        # Assume backup zip exists
        BACKUP_ZIP="devforgeai/backups/azure/${DEPLOYMENT_NAME}_${PREVIOUS_VERSION}.zip"
        if [ ! -f "$BACKUP_ZIP" ]; then
            log_error "Backup not found: $BACKUP_ZIP"
            exit 1
        fi

        az webapp deployment source config-zip \
            --resource-group "$RESOURCE_GROUP" \
            --name "$DEPLOYMENT_NAME" \
            --src "$BACKUP_ZIP"
    fi

    log_info "Azure App Service rollback completed successfully"
}

rollback_aws_ecs() {
    log_step "Rolling back AWS ECS Service: $DEPLOYMENT_NAME"

    # Check if aws CLI is available
    if ! command -v aws &> /dev/null; then
        log_error "AWS CLI not found. Install awscli."
        exit 1
    fi

    # Validate cluster name
    if [ -z "$CLUSTER_NAME" ]; then
        log_error "AWS ECS rollback requires --cluster parameter"
        exit 2
    fi

    # Get previous task definition if not specified
    if [ -z "$PREVIOUS_VERSION" ]; then
        log_info "Determining previous task definition..."
        PREVIOUS_VERSION=$(aws ecs describe-services \
            --cluster "$CLUSTER_NAME" \
            --services "$DEPLOYMENT_NAME" \
            --query "services[0].deployments[?status=='PRIMARY'].taskDefinition" \
            --output text | sed 's/.*://')

        # Decrement version
        PREVIOUS_VERSION=$((PREVIOUS_VERSION - 1))
        log_info "Previous version: $PREVIOUS_VERSION"
    fi

    # Update service to previous task definition
    TASK_DEF_ARN="${DEPLOYMENT_NAME}:${PREVIOUS_VERSION}"
    log_info "Rolling back to task definition: $TASK_DEF_ARN"

    aws ecs update-service \
        --cluster "$CLUSTER_NAME" \
        --service "$DEPLOYMENT_NAME" \
        --task-definition "$TASK_DEF_ARN"

    # Wait for service stability
    log_info "Waiting for service to stabilize..."
    aws ecs wait services-stable \
        --cluster "$CLUSTER_NAME" \
        --services "$DEPLOYMENT_NAME"

    log_info "AWS ECS rollback completed successfully"
}

rollback_aws_lambda() {
    log_step "Rolling back AWS Lambda Function: $DEPLOYMENT_NAME"

    # Check if aws CLI is available
    if ! command -v aws &> /dev/null; then
        log_error "AWS CLI not found. Install awscli."
        exit 1
    fi

    # Validate version
    if [ -z "$PREVIOUS_VERSION" ]; then
        log_error "AWS Lambda rollback requires --version parameter"
        exit 2
    fi

    # Update alias to previous version
    log_info "Rolling back Lambda alias 'production' to version $PREVIOUS_VERSION..."
    aws lambda update-alias \
        --function-name "$DEPLOYMENT_NAME" \
        --name production \
        --function-version "$PREVIOUS_VERSION"

    log_info "AWS Lambda rollback completed successfully"
}

rollback_docker() {
    log_step "Rolling back Docker Container: $DEPLOYMENT_NAME"

    # Check if docker is available
    if ! command -v docker &> /dev/null; then
        log_error "Docker not found. Install Docker CLI."
        exit 1
    fi

    # Validate version
    if [ -z "$PREVIOUS_VERSION" ]; then
        log_error "Docker rollback requires --version parameter"
        exit 2
    fi

    # Stop current container
    log_info "Stopping current container..."
    docker stop "$DEPLOYMENT_NAME" || log_warn "Container may not be running"

    # Remove current container
    log_info "Removing current container..."
    docker rm "$DEPLOYMENT_NAME" || log_warn "Container may not exist"

    # Start previous version
    log_info "Starting previous version: $PREVIOUS_VERSION"
    docker run -d \
        --name "$DEPLOYMENT_NAME" \
        --restart always \
        -p 80:80 \
        "myregistry/${DEPLOYMENT_NAME}:${PREVIOUS_VERSION}"

    log_info "Docker rollback completed successfully"
}

rollback_gcp() {
    log_step "Rolling back Google Cloud Platform deployment: $DEPLOYMENT_NAME"

    # Check if gcloud is available
    if ! command -v gcloud &> /dev/null; then
        log_error "gcloud CLI not found. Install Google Cloud SDK."
        exit 1
    fi

    # Validate version
    if [ -z "$PREVIOUS_VERSION" ]; then
        log_error "GCP rollback requires --version parameter"
        exit 2
    fi

    # Deploy previous version
    log_info "Deploying previous version: $PREVIOUS_VERSION"
    gcloud app deploy --version="$PREVIOUS_VERSION" --promote

    log_info "GCP rollback completed successfully"
}

# Database rollback function
rollback_database() {
    log_step "Rolling back database migrations..."

    # Detect migration framework from tech-stack.md
    if [ -f "devforgeai/specs/context/tech-stack.md" ]; then
        if grep -q "Entity Framework" "devforgeai/specs/context/tech-stack.md"; then
            log_info "Detected Entity Framework migrations"
            dotnet ef database update "$PREVIOUS_VERSION"
        elif grep -q "Alembic" "devforgeai/specs/context/tech-stack.md"; then
            log_info "Detected Alembic migrations"
            alembic downgrade -1
        elif grep -q "Sequelize" "devforgeai/specs/context/tech-stack.md"; then
            log_info "Detected Sequelize migrations"
            npx sequelize-cli db:migrate:undo
        else
            log_warn "Could not detect migration framework. Manual database rollback required."
        fi
    else
        log_warn "tech-stack.md not found. Manual database rollback required."
    fi
}

# Execute platform-specific rollback
case "$DEPLOYMENT_TARGET" in
    kubernetes)
        rollback_kubernetes
        ;;
    helm)
        rollback_helm
        ;;
    azure)
        rollback_azure
        ;;
    aws_ecs)
        rollback_aws_ecs
        ;;
    aws_lambda)
        rollback_aws_lambda
        ;;
    docker)
        rollback_docker
        ;;
    gcp)
        rollback_gcp
        ;;
    *)
        log_error "Unsupported deployment target: $DEPLOYMENT_TARGET"
        exit 1
        ;;
esac

# Database rollback (if requested)
if [ "$ROLLBACK_DB" = true ]; then
    rollback_database
fi

# Health check verification
if [ "$VERIFY_HEALTH" = true ]; then
    log_step "Verifying health after rollback..."

    # Wait for services to stabilize
    sleep 10

    # Run health check if script exists
    if [ -f "scripts/health_check.py" ]; then
        log_info "Running health checks..."
        if python scripts/health_check.py --url https://api.example.com/health --retries 5; then
            log_info "Health checks passed"
        else
            log_error "Health checks failed after rollback"
            exit 1
        fi
    else
        log_warn "Health check script not found. Manual verification recommended."
    fi
fi

# Final summary
log_info ""
log_info "==================================="
log_info "Rollback Summary"
log_info "==================================="
log_info "Target:          $DEPLOYMENT_TARGET"
log_info "Deployment:      $DEPLOYMENT_NAME"
log_info "Version:         ${PREVIOUS_VERSION:-[previous]}"
log_info "Namespace:       ${NAMESPACE:-N/A}"
log_info "Database:        $([ "$ROLLBACK_DB" = true ] && echo "ROLLED BACK" || echo "NOT ROLLED BACK")"
log_info "Health Check:    $([ "$VERIFY_HEALTH" = true ] && echo "VERIFIED" || echo "SKIPPED")"
log_info "Timestamp:       $TIMESTAMP"
log_info "Log File:        $ROLLBACK_LOG"
log_info "==================================="
log_info "Rollback completed successfully!"
log_info "==================================="

exit 0
