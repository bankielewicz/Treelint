#!/usr/bin/env python3
"""
Health Check Script

Validates HTTP health endpoints with retry logic and exponential backoff.
Use this script to verify application availability after deployment.

Usage:
    python health_check.py --url https://api.example.com/health --retries 5 --timeout 10

Exit Codes:
    0: Success - Health check passed
    1: Failure - Health check failed after all retries

Examples:
    # Basic health check
    python health_check.py --url https://api.example.com/health

    # With custom retries and timeout
    python health_check.py --url https://staging.example.com/health --retries 10 --timeout 30

    # Multiple endpoints
    python health_check.py --url https://api.example.com/health --url https://api.example.com/status
"""

import argparse
import json
import logging
import sys
import time
from typing import Dict, Any, List
from urllib.parse import urlparse

try:
    import requests
except ImportError:
    print("ERROR: requests library not installed")
    print("Install with: pip install requests")
    sys.exit(1)

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s',
    datefmt='%Y-%m-%d %H:%M:%S'
)
logger = logging.getLogger(__name__)


def validate_url(url: str) -> bool:
    """
    Validate URL format.

    Args:
        url: URL string to validate

    Returns:
        True if valid, False otherwise
    """
    try:
        result = urlparse(url)
        return all([result.scheme in ['http', 'https'], result.netloc])
    except Exception:
        return False


def check_health_endpoint(url: str, timeout: int = 10) -> Dict[str, Any]:
    """
    Perform single health check request.

    Args:
        url: Health endpoint URL
        timeout: Request timeout in seconds

    Returns:
        Dictionary with check results:
            - success (bool): Whether check passed
            - status_code (int): HTTP status code
            - response_time (float): Response time in seconds
            - response_body (dict): Parsed JSON response (if available)
            - error (str): Error message (if failed)
    """
    start_time = time.time()

    try:
        response = requests.get(url, timeout=timeout, allow_redirects=True)
        elapsed = time.time() - start_time

        # Check if response is successful (2xx status code)
        success = 200 <= response.status_code < 300

        result = {
            'success': success,
            'status_code': response.status_code,
            'response_time': round(elapsed, 3),
            'url': url
        }

        # Try to parse JSON response
        try:
            result['response_body'] = response.json()
        except json.JSONDecodeError:
            result['response_body'] = response.text[:200]  # First 200 chars

        if not success:
            result['error'] = f"HTTP {response.status_code}: {response.reason}"

        return result

    except requests.exceptions.Timeout:
        return {
            'success': False,
            'status_code': None,
            'response_time': timeout,
            'url': url,
            'error': f"Request timeout after {timeout} seconds"
        }
    except requests.exceptions.ConnectionError as e:
        return {
            'success': False,
            'status_code': None,
            'response_time': time.time() - start_time,
            'url': url,
            'error': f"Connection error: {str(e)}"
        }
    except requests.exceptions.RequestException as e:
        return {
            'success': False,
            'status_code': None,
            'response_time': time.time() - start_time,
            'url': url,
            'error': f"Request error: {str(e)}"
        }


def health_check_with_retry(
    url: str,
    retries: int = 5,
    timeout: int = 10,
    backoff_base: float = 1.0
) -> bool:
    """
    Perform health check with exponential backoff retry logic.

    Args:
        url: Health endpoint URL
        retries: Maximum number of retry attempts
        timeout: Request timeout in seconds
        backoff_base: Base delay for exponential backoff (seconds)

    Returns:
        True if health check passed, False otherwise
    """
    attempt = 0

    while attempt < retries:
        attempt += 1

        logger.info(f"Health check attempt {attempt}/{retries} for {url}")

        result = check_health_endpoint(url, timeout)

        if result['success']:
            logger.info(f"✓ Health check PASSED")
            logger.info(f"  Status: {result['status_code']}")
            logger.info(f"  Response time: {result['response_time']}s")

            # Pretty print JSON response
            if isinstance(result.get('response_body'), dict):
                logger.info(f"  Response: {json.dumps(result['response_body'], indent=2)}")
            else:
                logger.info(f"  Response: {result.get('response_body', 'N/A')}")

            return True
        else:
            logger.warning(f"✗ Health check FAILED")
            logger.warning(f"  Error: {result['error']}")
            logger.warning(f"  Response time: {result['response_time']}s")

            if attempt < retries:
                # Exponential backoff: 1s, 2s, 4s, 8s, 16s
                delay = backoff_base * (2 ** (attempt - 1))
                logger.info(f"  Retrying in {delay}s...")
                time.sleep(delay)

    logger.error(f"✗ Health check FAILED after {retries} attempts")
    return False


def check_multiple_endpoints(
    urls: List[str],
    retries: int,
    timeout: int
) -> bool:
    """
    Check multiple health endpoints.

    Args:
        urls: List of health endpoint URLs
        retries: Maximum retry attempts per endpoint
        timeout: Request timeout in seconds

    Returns:
        True if ALL endpoints passed, False if ANY failed
    """
    all_passed = True
    results = []

    for i, url in enumerate(urls, 1):
        logger.info(f"\n{'='*60}")
        logger.info(f"Checking endpoint {i}/{len(urls)}: {url}")
        logger.info(f"{'='*60}")

        passed = health_check_with_retry(url, retries, timeout)
        results.append({
            'url': url,
            'passed': passed
        })

        if not passed:
            all_passed = False

    # Summary report
    logger.info(f"\n{'='*60}")
    logger.info("HEALTH CHECK SUMMARY")
    logger.info(f"{'='*60}")

    for result in results:
        status = "✓ PASS" if result['passed'] else "✗ FAIL"
        logger.info(f"{status} - {result['url']}")

    logger.info(f"\nTotal: {len(results)} endpoints")
    logger.info(f"Passed: {sum(1 for r in results if r['passed'])}")
    logger.info(f"Failed: {sum(1 for r in results if not r['passed'])}")

    return all_passed


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description='HTTP health endpoint checker with retry logic and exponential backoff',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Basic health check
  python health_check.py --url https://api.example.com/health

  # With custom retries and timeout
  python health_check.py --url https://staging.example.com/health --retries 10 --timeout 30

  # Check multiple endpoints
  python health_check.py \\
    --url https://api.example.com/health \\
    --url https://api.example.com/ready \\
    --retries 5

  # Quiet mode (errors only)
  python health_check.py --url https://api.example.com/health --quiet
        """
    )

    parser.add_argument(
        '--url',
        action='append',
        required=True,
        help='Health endpoint URL (can specify multiple times)'
    )

    parser.add_argument(
        '--retries',
        type=int,
        default=5,
        help='Maximum number of retry attempts (default: 5)'
    )

    parser.add_argument(
        '--timeout',
        type=int,
        default=10,
        help='Request timeout in seconds (default: 10)'
    )

    parser.add_argument(
        '--backoff',
        type=float,
        default=1.0,
        help='Base delay for exponential backoff in seconds (default: 1.0)'
    )

    parser.add_argument(
        '--quiet',
        action='store_true',
        help='Suppress info logs (errors only)'
    )

    args = parser.parse_args()

    # Adjust logging level
    if args.quiet:
        logger.setLevel(logging.ERROR)

    # Validate URLs
    for url in args.url:
        if not validate_url(url):
            logger.error(f"Invalid URL: {url}")
            logger.error("URL must start with http:// or https://")
            sys.exit(1)

    # Validate parameters
    if args.retries < 1:
        logger.error("Retries must be >= 1")
        sys.exit(1)

    if args.timeout < 1:
        logger.error("Timeout must be >= 1")
        sys.exit(1)

    if args.backoff < 0.1:
        logger.error("Backoff must be >= 0.1")
        sys.exit(1)

    try:
        # Check all endpoints
        if len(args.url) == 1:
            # Single endpoint
            success = health_check_with_retry(
                args.url[0],
                args.retries,
                args.timeout,
                args.backoff
            )
        else:
            # Multiple endpoints
            success = check_multiple_endpoints(
                args.url,
                args.retries,
                args.timeout
            )

        if success:
            logger.info("\n✓ All health checks PASSED")
            sys.exit(0)
        else:
            logger.error("\n✗ Health check FAILED")
            sys.exit(1)

    except KeyboardInterrupt:
        logger.warning("\nHealth check interrupted by user")
        sys.exit(1)
    except Exception as e:
        logger.error(f"Unexpected error: {e}")
        sys.exit(1)


if __name__ == '__main__':
    main()
