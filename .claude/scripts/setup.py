"""
DevForgeAI CLI Setup

Installation:
    pip install -e .claude/scripts/

Usage:
    devforgeai validate-dod devforgeai/specs/Stories/STORY-001.story.md
    devforgeai check-git
    devforgeai validate-context
"""

from setuptools import setup, find_packages
from pathlib import Path

# Read version from __init__.py
init_file = Path(__file__).parent / 'devforgeai_cli' / '__init__.py'
version = '0.1.0'
for line in init_file.read_text().splitlines():
    if line.startswith('__version__'):
        version = line.split('=')[1].strip(' \'"')
        break

setup(
    name='devforgeai-cli',
    version=version,
    description='DevForgeAI workflow validators for spec-driven development',
    long_description='Automated validation tools that prevent autonomous deferrals, '
                     'validate Git availability, and enforce workflow quality gates.',
    author='DevForgeAI Contributors',
    python_requires='>=3.8',
    packages=find_packages(),
    install_requires=[
        'PyYAML>=6.0',
    ],
    entry_points={
        'console_scripts': [
            # Renamed from 'devforgeai' to avoid conflict with npm global CLI
            # The npm CLI (devforgeai) is user-facing for installation
            # This Python CLI is internal for framework validation
            'devforgeai-validate=devforgeai_cli.cli:main',
        ],
    },
    classifiers=[
        'Development Status :: 3 - Alpha',
        'Intended Audience :: Developers',
        'Topic :: Software Development :: Quality Assurance',
        'Programming Language :: Python :: 3',
        'Programming Language :: Python :: 3.8',
        'Programming Language :: Python :: 3.9',
        'Programming Language :: Python :: 3.10',
        'Programming Language :: Python :: 3.11',
        'Programming Language :: Python :: 3.12',
    ],
)
