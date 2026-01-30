#!/usr/bin/env python3
"""
Smoke Test Runner

Orchestrates pytest smoke test suite with environment-specific configuration,
parallel execution, and comprehensive reporting.

Usage:
    python smoke_test_runner.py --environment production --tests critical_path
    python smoke_test_runner.py --environment staging --tests all --parallel 4

Exit Codes:
    0: Success - All tests passed
    1: Failure - One or more tests failed

Examples:
    # Run all smoke tests in production
    python smoke_test_runner.py --environment production

    # Run only critical path tests in staging
    python smoke_test_runner.py --environment staging --tests critical_path

    # Run specific test categories with parallel execution
    python smoke_test_runner.py --environment production --tests api,database --parallel 4

    # Generate HTML report
    python smoke_test_runner.py --environment production --html-report results.html
"""

import argparse
import json
import logging
import os
import subprocess
import sys
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Optional

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s',
    datefmt='%Y-%m-%d %H:%M:%S'
)
logger = logging.getLogger(__name__)


# Test category markers
TEST_CATEGORIES = {
    'all': '',  # Run all smoke tests
    'critical_path': 'critical_path',
    'api': 'api',
    'database': 'database',
    'authentication': 'auth',
    'integration': 'integration',
    'health': 'health'
}


def load_environment_config(environment: str, config_path: Optional[str] = None) -> Dict:
    """
    Load environment-specific configuration.

    Args:
        environment: Environment name (staging, production, etc.)
        config_path: Optional path to config file

    Returns:
        Configuration dictionary

    Raises:
        FileNotFoundError: If config file not found
        json.JSONDecodeError: If config file invalid
    """
    if config_path is None:
        # Default config location
        config_path = 'devforgeai/smoke-tests/config.json'

    config_file = Path(config_path)

    if not config_file.exists():
        logger.warning(f"Config file not found: {config_path}")
        logger.warning("Using default configuration")
        return {
            'environments': {
                environment: {
                    'base_url': f'https://{environment}.example.com',
                    'test_user': f'{environment}_test@example.com',
                    'test_password': 'test_password'
                }
            }
        }

    try:
        with open(config_file, 'r') as f:
            config = json.load(f)

        if environment not in config.get('environments', {}):
            raise ValueError(f"Environment '{environment}' not found in config")

        logger.info(f"✓ Loaded configuration for environment: {environment}")
        return config['environments'][environment]

    except json.JSONDecodeError as e:
        logger.error(f"Invalid JSON in config file: {e}")
        raise
    except Exception as e:
        logger.error(f"Error loading config: {e}")
        raise


def build_pytest_command(
    environment: str,
    env_config: Dict,
    test_categories: List[str],
    parallel: Optional[int] = None,
    html_report: Optional[str] = None,
    junit_xml: Optional[str] = None,
    verbose: bool = True
) -> List[str]:
    """
    Build pytest command with appropriate arguments.

    Args:
        environment: Environment name
        env_config: Environment configuration
        test_categories: List of test categories to run
        parallel: Number of parallel workers (requires pytest-xdist)
        html_report: Path for HTML report output
        junit_xml: Path for JUnit XML output
        verbose: Enable verbose output

    Returns:
        List of command arguments
    """
    cmd = ['pytest', 'tests/smoke/']

    # Add markers for test filtering
    if test_categories and 'all' not in test_categories:
        markers = [TEST_CATEGORIES[cat] for cat in test_categories if cat in TEST_CATEGORIES]
        if markers:
            # Only run tests with smoke marker AND one of the specified category markers
            marker_expr = ' or '.join(markers)
            cmd.extend(['-m', f'smoke and ({marker_expr})'])
        else:
            # Default: run all smoke tests
            cmd.extend(['-m', 'smoke'])
    else:
        # Run all tests marked as smoke
        cmd.extend(['-m', 'smoke'])

    # Parallel execution
    if parallel and parallel > 1:
        cmd.extend(['-n', str(parallel)])
        logger.info(f"Parallel execution enabled: {parallel} workers")

    # Verbose output
    if verbose:
        cmd.append('--verbose')

    # Traceback style
    cmd.append('--tb=short')

    # HTML report
    if html_report:
        cmd.extend(['--html', html_report, '--self-contained-html'])
        logger.info(f"HTML report will be generated: {html_report}")

    # JUnit XML report
    if junit_xml:
        cmd.extend(['--junit-xml', junit_xml])
        logger.info(f"JUnit XML report will be generated: {junit_xml}")

    # Environment configuration (passed as pytest options)
    cmd.extend([
        f'--environment={environment}',
        f'--base-url={env_config.get("base_url", "")}',
    ])

    # Add environment variables for sensitive data
    # (Credentials should NOT be passed via command line)

    return cmd


def set_environment_variables(env_config: Dict):
    """
    Set environment variables for test execution.

    Args:
        env_config: Environment configuration
    """
    if 'test_user' in env_config:
        os.environ['TEST_USER'] = env_config['test_user']

    if 'test_password' in env_config:
        os.environ['TEST_PASSWORD'] = env_config['test_password']

    if 'api_key' in env_config:
        os.environ['TEST_API_KEY'] = env_config['api_key']

    # Add other environment-specific variables as needed


def run_smoke_tests(
    environment: str,
    test_categories: List[str],
    config_path: Optional[str] = None,
    parallel: Optional[int] = None,
    html_report: Optional[str] = None,
    junit_xml: Optional[str] = None,
    verbose: bool = True
) -> bool:
    """
    Execute smoke test suite.

    Args:
        environment: Environment name
        test_categories: List of test categories to run
        config_path: Optional path to config file
        parallel: Number of parallel workers
        html_report: Path for HTML report output
        junit_xml: Path for JUnit XML output
        verbose: Enable verbose output

    Returns:
        True if all tests passed, False otherwise
    """
    try:
        # Load environment configuration
        env_config = load_environment_config(environment, config_path)

        # Set environment variables
        set_environment_variables(env_config)

        # Build pytest command
        cmd = build_pytest_command(
            environment,
            env_config,
            test_categories,
            parallel,
            html_report,
            junit_xml,
            verbose
        )

        # Log command
        logger.info(f"\n{'='*60}")
        logger.info(f"Running smoke tests for environment: {environment}")
        logger.info(f"Test categories: {', '.join(test_categories) if test_categories else 'all'}")
        logger.info(f"{'='*60}")
        logger.debug(f"Command: {' '.join(cmd)}")

        # Execute pytest
        start_time = datetime.now()

        result = subprocess.run(
            cmd,
            capture_output=False,  # Show pytest output in real-time
            text=True
        )

        end_time = datetime.now()
        duration = (end_time - start_time).total_seconds()

        # Parse results
        logger.info(f"\n{'='*60}")
        logger.info(f"SMOKE TEST RESULTS")
        logger.info(f"{'='*60}")
        logger.info(f"Environment: {environment}")
        logger.info(f"Duration: {duration:.2f}s")

        if result.returncode == 0:
            logger.info("Status: ✓ ALL TESTS PASSED")
            logger.info(f"{'='*60}\n")
            return True
        else:
            logger.error("Status: ✗ TESTS FAILED")
            logger.error(f"Exit code: {result.returncode}")
            logger.info(f"{'='*60}\n")
            return False

    except FileNotFoundError:
        logger.error("pytest not found. Install with: pip install pytest")
        return False
    except Exception as e:
        logger.error(f"Error running smoke tests: {e}")
        return False


def parse_test_categories(categories_arg: str) -> List[str]:
    """
    Parse comma-separated test categories.

    Args:
        categories_arg: Comma-separated category names

    Returns:
        List of category names
    """
    if not categories_arg or categories_arg.lower() == 'all':
        return ['all']

    categories = [cat.strip() for cat in categories_arg.split(',')]

    # Validate categories
    invalid = [cat for cat in categories if cat not in TEST_CATEGORIES]
    if invalid:
        logger.warning(f"Invalid test categories will be ignored: {', '.join(invalid)}")

    valid = [cat for cat in categories if cat in TEST_CATEGORIES]

    if not valid:
        logger.warning("No valid test categories specified, running all tests")
        return ['all']

    return valid


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description='Orchestrate pytest smoke test suite with environment-specific configuration',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=f"""
Test Categories:
  {', '.join(TEST_CATEGORIES.keys())}

Examples:
  # Run all smoke tests in production
  python smoke_test_runner.py --environment production

  # Run critical path tests in staging
  python smoke_test_runner.py --environment staging --tests critical_path

  # Run API and database tests with parallel execution
  python smoke_test_runner.py --environment production --tests api,database --parallel 4

  # Generate HTML and JUnit XML reports
  python smoke_test_runner.py --environment production --html-report results.html --junit-xml results.xml
        """
    )

    parser.add_argument(
        '--environment',
        required=True,
        choices=['staging', 'production', 'production-green', 'production-canary'],
        help='Target environment for testing'
    )

    parser.add_argument(
        '--tests',
        default='all',
        help=f'Comma-separated test categories to run (default: all). Options: {", ".join(TEST_CATEGORIES.keys())}'
    )

    parser.add_argument(
        '--config',
        help='Path to configuration file (default: devforgeai/smoke-tests/config.json)'
    )

    parser.add_argument(
        '--parallel',
        type=int,
        help='Number of parallel test workers (requires pytest-xdist)'
    )

    parser.add_argument(
        '--html-report',
        help='Generate HTML report at specified path (requires pytest-html)'
    )

    parser.add_argument(
        '--junit-xml',
        help='Generate JUnit XML report at specified path'
    )

    parser.add_argument(
        '--quiet',
        action='store_true',
        help='Suppress verbose pytest output'
    )

    parser.add_argument(
        '--debug',
        action='store_true',
        help='Enable debug logging'
    )

    args = parser.parse_args()

    # Adjust logging level
    if args.debug:
        logger.setLevel(logging.DEBUG)

    # Parse test categories
    test_categories = parse_test_categories(args.tests)

    try:
        # Run smoke tests
        success = run_smoke_tests(
            environment=args.environment,
            test_categories=test_categories,
            config_path=args.config,
            parallel=args.parallel,
            html_report=args.html_report,
            junit_xml=args.junit_xml,
            verbose=not args.quiet
        )

        sys.exit(0 if success else 1)

    except KeyboardInterrupt:
        logger.warning("\nSmoke tests interrupted by user")
        sys.exit(1)
    except Exception as e:
        logger.error(f"Unexpected error: {e}")
        if args.debug:
            import traceback
            traceback.print_exc()
        sys.exit(1)


if __name__ == '__main__':
    main()
