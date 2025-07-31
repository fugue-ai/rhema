#!/bin/bash

# Lock File CI/CD Integration Script
# This script provides standalone commands for lock file operations in CI/CD pipelines

set -euo pipefail

# Default configuration
LOCK_FILE_PATH="${LOCK_FILE_PATH:-rhema.lock}"
LOCK_REPORTS_DIR="${LOCK_REPORTS_DIR:-lock-reports}"
RHEMA_BIN="${RHEMA_BIN:-rhema}"
EXIT_CODE="${EXIT_CODE:-1}"
MAX_CIRCULAR_DEPS="${MAX_CIRCULAR_DEPS:-0}"
MAX_AGE="${MAX_AGE:-24}"
FAIL_ON_WARNINGS="${FAIL_ON_WARNINGS:-true}"
FORMAT="${FORMAT:-json}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if Rhema CLI is available
check_rhema() {
    if ! command -v "$RHEMA_BIN" &> /dev/null; then
        log_error "Rhema CLI not found. Please install it first."
        exit 1
    fi
}

# Create reports directory
create_reports_dir() {
    mkdir -p "$LOCK_REPORTS_DIR"
}

# Lock file validation
validate_lock_file() {
    local report_file="$LOCK_REPORTS_DIR/validation-report.$FORMAT"
    local fail_on_warnings_flag=""
    
    if [[ "$FAIL_ON_WARNINGS" == "true" ]]; then
        fail_on_warnings_flag="--fail-on-warnings"
    fi
    
    log_info "Validating lock file: $LOCK_FILE_PATH"
    
    if ! "$RHEMA_BIN" lock ci-validate \
        --file "$LOCK_FILE_PATH" \
        --exit-code "$EXIT_CODE" \
        --max-circular-deps "$MAX_CIRCULAR_DEPS" \
        --max-age "$MAX_AGE" \
        $fail_on_warnings_flag \
        --report-file "$report_file" \
        --format "$FORMAT"; then
        log_error "Lock file validation failed"
        return 1
    fi
    
    log_success "Lock file validation completed"
    log_info "Report saved to: $report_file"
}

# Lock file generation
generate_lock_file() {
    local report_file="$LOCK_REPORTS_DIR/generation-report.$FORMAT"
    local strategy="${STRATEGY:-latest}"
    local timeout="${TIMEOUT:-300}"
    local fail_on_circular_flag=""
    
    if [[ "${FAIL_ON_CIRCULAR:-true}" == "true" ]]; then
        fail_on_circular_flag="--fail-on-circular"
    fi
    
    log_info "Generating lock file: $LOCK_FILE_PATH"
    log_info "Strategy: $strategy, Timeout: ${timeout}s"
    
    if ! "$RHEMA_BIN" lock ci-generate \
        --output "$LOCK_FILE_PATH" \
        --strategy "$strategy" \
        $fail_on_circular_flag \
        --timeout "$timeout" \
        --report-file "$report_file" \
        --format "$FORMAT" \
        --exit-code "$EXIT_CODE"; then
        log_error "Lock file generation failed"
        return 1
    fi
    
    log_success "Lock file generation completed"
    log_info "Report saved to: $report_file"
    
    # Validate the generated lock file
    log_info "Validating generated lock file"
    validate_lock_file
}

# Lock file consistency check
check_consistency() {
    local report_file="$LOCK_REPORTS_DIR/consistency-report.$FORMAT"
    local git_branch="${GIT_BRANCH:-main}"
    local allow_semver_diffs_flag=""
    local max_version_drift="${MAX_VERSION_DRIFT:-0.1.0}"
    
    if [[ "${ALLOW_SEMVER_DIFFS:-true}" == "true" ]]; then
        allow_semver_diffs_flag="--allow-semver-diffs"
    fi
    
    log_info "Checking lock file consistency"
    log_info "Reference branch: $git_branch"
    
    if ! "$RHEMA_BIN" lock ci-consistency \
        --file "$LOCK_FILE_PATH" \
        --git-branch "$git_branch" \
        $allow_semver_diffs_flag \
        --max-version-drift "$max_version_drift" \
        --report-file "$report_file" \
        --format "$FORMAT" \
        --exit-code "$EXIT_CODE"; then
        log_error "Lock file consistency check failed"
        return 1
    fi
    
    log_success "Lock file consistency check completed"
    log_info "Report saved to: $report_file"
}

# Lock file update
update_lock_file() {
    local report_file="$LOCK_REPORTS_DIR/update-report.$FORMAT"
    local update_strategy="${UPDATE_STRATEGY:-auto}"
    local strategy="${STRATEGY:-latest}"
    local max_updates="${MAX_UPDATES:-10}"
    local security_only_flag=""
    local backup_flag=""
    
    if [[ "${SECURITY_ONLY:-true}" == "true" ]]; then
        security_only_flag="--security-only"
    fi
    
    if [[ "${BACKUP:-true}" == "true" ]]; then
        backup_flag="--backup"
    fi
    
    log_info "Updating lock file: $LOCK_FILE_PATH"
    log_info "Update strategy: $update_strategy, Max updates: $max_updates"
    
    if ! "$RHEMA_BIN" lock ci-update \
        --file "$LOCK_FILE_PATH" \
        --update-strategy "$update_strategy" \
        --strategy "$strategy" \
        $security_only_flag \
        --max-updates "$max_updates" \
        $backup_flag \
        --report-file "$report_file" \
        --format "$FORMAT" \
        --exit-code "$EXIT_CODE"; then
        log_error "Lock file update failed"
        return 1
    fi
    
    log_success "Lock file update completed"
    log_info "Report saved to: $report_file"
    
    # Validate the updated lock file
    log_info "Validating updated lock file"
    validate_lock_file
}

# Lock file health check
health_check() {
    local report_file="$LOCK_REPORTS_DIR/health-report.$FORMAT"
    local integrity_flag=""
    local freshness_flag=""
    local availability_flag=""
    local performance_flag=""
    
    if [[ "${CHECK_INTEGRITY:-true}" == "true" ]]; then
        integrity_flag="--integrity"
    fi
    
    if [[ "${CHECK_FRESHNESS:-true}" == "true" ]]; then
        freshness_flag="--freshness"
    fi
    
    if [[ "${CHECK_AVAILABILITY:-true}" == "true" ]]; then
        availability_flag="--availability"
    fi
    
    if [[ "${CHECK_PERFORMANCE:-true}" == "true" ]]; then
        performance_flag="--performance"
    fi
    
    log_info "Performing lock file health check"
    
    if ! "$RHEMA_BIN" lock ci-health \
        --file "$LOCK_FILE_PATH" \
        $integrity_flag \
        $freshness_flag \
        $availability_flag \
        $performance_flag \
        --report-file "$report_file" \
        --format "$FORMAT" \
        --exit-code "$EXIT_CODE"; then
        log_error "Lock file health check failed"
        return 1
    fi
    
    log_success "Lock file health check completed"
    log_info "Report saved to: $report_file"
}

# Full CI/CD pipeline
run_full_pipeline() {
    log_info "Running full lock file CI/CD pipeline"
    
    create_reports_dir
    
    # Run all checks
    validate_lock_file
    check_consistency
    health_check
    
    log_success "Full pipeline completed successfully"
}

# Show usage
show_usage() {
    cat << EOF
Lock File CI/CD Integration Script

Usage: $0 <command> [options]

Commands:
    validate          Validate lock file integrity and consistency
    generate          Generate new lock file
    consistency       Check lock file consistency across environments
    update            Update lock file dependencies
    health            Perform health check on lock file
    pipeline          Run full CI/CD pipeline (validate + consistency + health)

Environment Variables:
    LOCK_FILE_PATH        Path to lock file (default: rhema.lock)
    LOCK_REPORTS_DIR      Directory for reports (default: lock-reports)
    RHEMA_BIN             Rhema CLI binary (default: rhema)
    EXIT_CODE             Exit code for failures (default: 1)
    MAX_CIRCULAR_DEPS     Max circular dependencies (default: 0)
    MAX_AGE               Max lock file age in hours (default: 24)
    FAIL_ON_WARNINGS      Fail on warnings (default: true)
    FORMAT                Output format (default: json)
    STRATEGY              Resolution strategy (default: latest)
    TIMEOUT               Generation timeout in seconds (default: 300)
    FAIL_ON_CIRCULAR      Fail on circular deps (default: true)
    GIT_BRANCH            Reference git branch (default: main)
    ALLOW_SEMVER_DIFFS    Allow semver differences (default: true)
    MAX_VERSION_DRIFT     Max version drift (default: 0.1.0)
    UPDATE_STRATEGY       Update strategy (default: auto)
    SECURITY_ONLY         Security-only updates (default: true)
    MAX_UPDATES           Max updates (default: 10)
    BACKUP                Create backup (default: true)
    CHECK_INTEGRITY       Check integrity (default: true)
    CHECK_FRESHNESS       Check freshness (default: true)
    CHECK_AVAILABILITY    Check availability (default: true)
    CHECK_PERFORMANCE     Check performance (default: true)

Examples:
    # Validate lock file
    $0 validate

    # Generate new lock file
    $0 generate

    # Run full pipeline
    $0 pipeline

    # Custom configuration
    LOCK_FILE_PATH=my.lock MAX_CIRCULAR_DEPS=1 $0 validate

EOF
}

# Main script logic
main() {
    local command="${1:-}"
    
    case "$command" in
        validate)
            check_rhema
            create_reports_dir
            validate_lock_file
            ;;
        generate)
            check_rhema
            create_reports_dir
            generate_lock_file
            ;;
        consistency)
            check_rhema
            create_reports_dir
            check_consistency
            ;;
        update)
            check_rhema
            create_reports_dir
            update_lock_file
            ;;
        health)
            check_rhema
            create_reports_dir
            health_check
            ;;
        pipeline)
            check_rhema
            run_full_pipeline
            ;;
        help|--help|-h)
            show_usage
            ;;
        *)
            log_error "Unknown command: $command"
            show_usage
            exit 1
            ;;
    esac
}

# Run main function with all arguments
main "$@" 