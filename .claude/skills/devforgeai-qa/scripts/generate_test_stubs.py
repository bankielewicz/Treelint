#!/usr/bin/env python3
"""
Test Stub Generator

Auto-generates test stub templates for untested code:
- Parses source files to identify methods/functions
- Identifies untested methods based on test file naming
- Generates test stubs with framework-specific syntax

Supports:
- C#: xUnit
- Python: pytest
- JavaScript/TypeScript: Jest
"""

import re
import sys
from pathlib import Path
from typing import Dict, List, Tuple
from dataclasses import dataclass
import argparse


@dataclass
class Method:
    """Method/function metadata"""
    name: str
    parameters: List[str]
    return_type: str
    class_name: str
    file: str
    line: int


class TestStubGenerator:
    """Generate test stubs for untested code"""

    def __init__(self, source_file: str, test_framework: str = None):
        self.source_file = Path(source_file)
        self.test_framework = test_framework or self._detect_framework()

    def _detect_framework(self) -> str:
        """Detect test framework from file extension"""
        ext = self.source_file.suffix

        if ext == '.cs':
            return 'xunit'
        elif ext == '.py':
            return 'pytest'
        elif ext in ['.js', '.ts', '.tsx']:
            return 'jest'
        else:
            raise ValueError(f"Unsupported file type: {ext}")

    def generate_stubs(self) -> str:
        """Generate test stubs for source file"""
        # Parse source file
        methods = self._parse_source_file()

        if not methods:
            return "# No methods found to test"

        # Generate test file content
        if self.test_framework == 'xunit':
            return self._generate_xunit_stubs(methods)
        elif self.test_framework == 'pytest':
            return self._generate_pytest_stubs(methods)
        elif self.test_framework == 'jest':
            return self._generate_jest_stubs(methods)
        else:
            raise ValueError(f"Unsupported framework: {self.test_framework}")

    def _parse_source_file(self) -> List[Method]:
        """Parse source file and extract methods"""
        with open(self.source_file, 'r', encoding='utf-8') as f:
            content = f.read()

        if self.source_file.suffix == '.cs':
            return self._parse_csharp(content)
        elif self.source_file.suffix == '.py':
            return self._parse_python(content)
        elif self.source_file.suffix in ['.js', '.ts', '.tsx']:
            return self._parse_javascript(content)
        else:
            return []

    def _parse_csharp(self, content: str) -> List[Method]:
        """Parse C# source file"""
        methods = []

        # Extract class name
        class_match = re.search(r'public\s+class\s+(\w+)', content)
        class_name = class_match.group(1) if class_match else 'UnknownClass'

        # Find public methods
        method_pattern = r'public\s+(?:async\s+)?(?:Task<)?(\w+)(?:>)?\s+(\w+)\s*\((.*?)\)'
        matches = re.finditer(method_pattern, content)

        for match in matches:
            return_type = match.group(1)
            method_name = match.group(2)
            params_str = match.group(3)

            # Parse parameters
            parameters = []
            if params_str.strip():
                param_parts = params_str.split(',')
                for param in param_parts:
                    param = param.strip()
                    if param:
                        # Extract parameter name (last word)
                        param_name = param.split()[-1]
                        parameters.append(param_name)

            line_num = content[:match.start()].count('\n') + 1

            methods.append(Method(
                name=method_name,
                parameters=parameters,
                return_type=return_type,
                class_name=class_name,
                file=str(self.source_file),
                line=line_num
            ))

        return methods

    def _parse_python(self, content: str) -> List[Method]:
        """Parse Python source file"""
        methods = []

        # Extract class name
        class_match = re.search(r'class\s+(\w+)', content)
        class_name = class_match.group(1) if class_match else 'Module'

        # Find public methods/functions (not starting with _)
        method_pattern = r'def\s+((?!_)\w+)\s*\((.*?)\)(?:\s*->\s*(\w+))?'
        matches = re.finditer(method_pattern, content)

        for match in matches:
            method_name = match.group(1)
            params_str = match.group(2)
            return_type = match.group(3) or 'None'

            # Parse parameters
            parameters = []
            if params_str.strip():
                param_parts = params_str.split(',')
                for param in param_parts:
                    param = param.strip()
                    if param and param != 'self':
                        # Extract parameter name (before : if type hint)
                        param_name = param.split(':')[0].strip()
                        parameters.append(param_name)

            line_num = content[:match.start()].count('\n') + 1

            methods.append(Method(
                name=method_name,
                parameters=parameters,
                return_type=return_type,
                class_name=class_name,
                file=str(self.source_file),
                line=line_num
            ))

        return methods

    def _parse_javascript(self, content: str) -> List[Method]:
        """Parse JavaScript/TypeScript source file"""
        methods = []

        # Extract class name if exists
        class_match = re.search(r'(?:export\s+)?class\s+(\w+)', content)
        class_name = class_match.group(1) if class_match else 'Module'

        # Find functions/methods
        patterns = [
            r'(?:export\s+)?function\s+(\w+)\s*\((.*?)\)',  # function declarations
            r'(?:export\s+)?const\s+(\w+)\s*=\s*\((.*?)\)\s*=>',  # arrow functions
            r'(\w+)\s*\((.*?)\)\s*\{',  # method declarations
        ]

        for pattern in patterns:
            matches = re.finditer(pattern, content)
            for match in matches:
                method_name = match.group(1)
                params_str = match.group(2)

                # Parse parameters
                parameters = []
                if params_str.strip():
                    param_parts = params_str.split(',')
                    for param in param_parts:
                        param = param.strip()
                        if param:
                            # Extract parameter name (before : if type annotation)
                            param_name = param.split(':')[0].strip()
                            parameters.append(param_name)

                line_num = content[:match.start()].count('\n') + 1

                methods.append(Method(
                    name=method_name,
                    parameters=parameters,
                    return_type='unknown',
                    class_name=class_name,
                    file=str(self.source_file),
                    line=line_num
                ))

        return methods

    def _generate_xunit_stubs(self, methods: List[Method]) -> str:
        """Generate xUnit test stubs for C#"""
        class_name = methods[0].class_name if methods else 'UnknownClass'

        stubs = f"""using Xunit;
using System;

namespace Tests
{{
    public class {class_name}Tests
    {{
"""

        for method in methods:
            # Generate test for valid input
            stubs += f"""        [Fact]
        public void {method.name}_ValidInput_ReturnsExpectedResult()
        {{
            // Arrange
            var sut = new {class_name}();
            // TODO: Set up test data
"""

            # Add parameter setup
            for param in method.parameters:
                stubs += f"            // var {param} = ...;\n"

            stubs += f"""
            // Act
            var result = sut.{method.name}({', '.join(method.parameters)});

            // Assert
            Assert.NotNull(result);
            // TODO: Add specific assertions
        }}

"""

            # Generate test for null input (if has parameters)
            if method.parameters:
                stubs += f"""        [Fact]
        public void {method.name}_NullInput_ThrowsArgumentNullException()
        {{
            // Arrange
            var sut = new {class_name}();

            // Act & Assert
            Assert.Throws<ArgumentNullException>(() => sut.{method.name}(null));
        }}

"""

        stubs += """    }
}
"""
        return stubs

    def _generate_pytest_stubs(self, methods: List[Method]) -> str:
        """Generate pytest test stubs for Python"""
        class_name = methods[0].class_name if methods else 'Module'
        module_name = self.source_file.stem

        stubs = f"""import pytest
from {module_name} import {class_name}


class Test{class_name}:
    \"\"\"Tests for {class_name}\"\"\"

"""

        for method in methods:
            # Generate test for valid input
            stubs += f"""    def test_{method.name}_valid_input_returns_expected_result(self):
        \"\"\"Test {method.name} with valid input\"\"\"
        # Arrange
        sut = {class_name}()
        # TODO: Set up test data
"""

            # Add parameter setup
            for param in method.parameters:
                stubs += f"        # {param} = ...\n"

            param_str = ', '.join(method.parameters) if method.parameters else ''
            stubs += f"""
        # Act
        result = sut.{method.name}({param_str})

        # Assert
        assert result is not None
        # TODO: Add specific assertions

"""

            # Generate test for invalid input
            if method.parameters:
                stubs += f"""    def test_{method.name}_none_input_raises_exception(self):
        \"\"\"Test {method.name} with None input raises exception\"\"\"
        # Arrange
        sut = {class_name}()

        # Act & Assert
        with pytest.raises(TypeError):
            sut.{method.name}(None)

"""

        return stubs

    def _generate_jest_stubs(self, methods: List[Method]) -> str:
        """Generate Jest test stubs for JavaScript/TypeScript"""
        class_name = methods[0].class_name if methods else 'Module'
        module_name = self.source_file.stem

        stubs = f"""import {{ {class_name} }} from './{module_name}';

describe('{class_name}', () => {{
"""

        for method in methods:
            stubs += f"""  describe('{method.name}', () => {{
    it('should return expected result with valid input', () => {{
      // Arrange
      const sut = new {class_name}();
      // TODO: Set up test data
"""

            # Add parameter setup
            for param in method.parameters:
                stubs += f"      // const {param} = ...;\n"

            param_str = ', '.join(method.parameters) if method.parameters else ''
            stubs += f"""
      // Act
      const result = sut.{method.name}({param_str});

      // Assert
      expect(result).toBeDefined();
      // TODO: Add specific assertions
    }});

"""

            # Generate test for null input
            if method.parameters:
                stubs += f"""    it('should throw error with null input', () => {{
      // Arrange
      const sut = new {class_name}();

      // Act & Assert
      expect(() => sut.{method.name}(null)).toThrow();
    }});

"""

            stubs += "  });\n\n"

        stubs += "});\n"
        return stubs

    def write_test_file(self, output_file: str = None):
        """Generate and write test file"""
        stubs = self.generate_stubs()

        # Determine output file name if not provided
        if not output_file:
            if self.test_framework == 'xunit':
                output_file = f"{self.source_file.stem}Tests.cs"
            elif self.test_framework == 'pytest':
                output_file = f"test_{self.source_file.stem}.py"
            elif self.test_framework == 'jest':
                output_file = f"{self.source_file.stem}.test.{self.source_file.suffix[1:]}"

        # Write file
        with open(output_file, 'w') as f:
            f.write(stubs)

        print(f"✅ Test stubs generated: {output_file}")
        print(f"   Framework: {self.test_framework}")
        print(f"   TODO: Complete test implementations and verify assertions")


def main():
    parser = argparse.ArgumentParser(
        description='Generate test stubs for untested code',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Auto-detect framework from file extension
  python generate_test_stubs.py src/Services/OrderService.cs

  # Specify framework explicitly
  python generate_test_stubs.py src/order_service.py --framework pytest

  # Specify output file
  python generate_test_stubs.py src/user.js --output tests/user.test.js
        """
    )
    parser.add_argument('source_file', help='Source file to generate tests for')
    parser.add_argument('--framework', choices=['xunit', 'pytest', 'jest'],
                        help='Test framework (auto-detected if not specified)')
    parser.add_argument('--output', help='Output test file path')

    args = parser.parse_args()

    try:
        generator = TestStubGenerator(args.source_file, args.framework)
        generator.write_test_file(args.output)
        return 0
    except Exception as e:
        print(f"❌ Error: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        return 1


if __name__ == '__main__':
    sys.exit(main())
