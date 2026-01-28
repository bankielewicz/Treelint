# Language-Specific Tooling Reference

Comprehensive guide to testing, coverage, linting, and quality tools for all supported languages in DevForgeAI.

---

## .NET / C#

### Testing Frameworks

**Primary: xUnit**
```bash
# Run tests
dotnet test

# Run specific test
dotnet test --filter "FullyQualifiedName~OrderService"

# Run tests with verbosity
dotnet test --logger "console;verbosity=detailed"
```

**Alternative: NUnit**
```bash
dotnet test --framework net6.0
```

**Alternative: MSTest**
```bash
dotnet test --settings test.runsettings
```

### Coverage Tools

**coverlet (Cross-platform)**
```bash
# Collect coverage
dotnet test --collect:"XPlat Code Coverage"

# Coverage with specific format
dotnet test --collect:"XPlat Code Coverage" --results-directory ./TestResults

# Find coverage file
find ./TestResults -name "coverage.cobertura.xml"
```

**dotnet-coverage (Microsoft official)**
```bash
# Install
dotnet tool install -g dotnet-coverage

# Collect coverage
dotnet-coverage collect "dotnet test" -f xml -o coverage.xml

# Merge multiple coverage files
dotnet-coverage merge -o merged.xml coverage1.xml coverage2.xml
```

**Report Generation**
```bash
# Install ReportGenerator
dotnet tool install -g dotnet-reportgenerator-globaltool

# Generate HTML report
reportgenerator -reports:coverage.cobertura.xml -targetdir:coverage-report -reporttypes:Html

# Generate multiple formats
reportgenerator -reports:coverage.cobertura.xml -targetdir:reports -reporttypes:"Html;JsonSummary;Badges"
```

### Linting & Formatting

**dotnet format (Built-in)**
```bash
# Check formatting (dry run)
dotnet format --verify-no-changes

# Fix formatting
dotnet format

# Format specific project
dotnet format MyProject.csproj
```

**StyleCop Analyzers**
```xml
<!-- Add to .csproj -->
<ItemGroup>
  <PackageReference Include="StyleCop.Analyzers" Version="1.1.118" />
</ItemGroup>
```

**Roslyn Analyzers**
```bash
# Enable in .csproj
<PropertyGroup>
  <EnableNETAnalyzers>true</EnableNETAnalyzers>
  <AnalysisLevel>latest</AnalysisLevel>
  <EnforceCodeStyleInBuild>true</EnforceCodeStyleInBuild>
</PropertyGroup>
```

### Quality Metrics

**dotnet-metrics (Complexity)**
```bash
# Install
dotnet tool install -g dotnet-metrics

# Analyze project
dotnet-metrics -p MyProject.csproj

# Generate JSON report
dotnet-metrics -p MyProject.csproj -o metrics.json
```

**Roslynator (Code Analysis)**
```bash
# Install
dotnet tool install -g roslynator.dotnet.cli

# Analyze solution
roslynator analyze MySolution.sln
```

### Parsing Coverage Data (.NET)

**Cobertura XML Format:**
```xml
<coverage line-rate="0.85" branch-rate="0.78">
  <packages>
    <package name="MyApp.Domain" line-rate="0.92">
      <classes>
        <class name="OrderService" filename="src/Domain/OrderService.cs" line-rate="0.90">
          <lines>
            <line number="10" hits="5" branch="false"/>
            <line number="15" hits="0" branch="false"/>
          </lines>
        </class>
      </classes>
    </package>
  </packages>
</coverage>
```

**Parsing Strategy:**
```bash
# Extract overall coverage using xmllint
xmllint --xpath "string(//coverage/@line-rate)" coverage.cobertura.xml

# Extract per-package coverage
xmllint --xpath "//package/@name | //package/@line-rate" coverage.cobertura.xml
```

---

## Python

### Testing Frameworks

**Primary: pytest**
```bash
# Run tests
pytest

# Run specific test file
pytest tests/test_order_service.py

# Run specific test method
pytest tests/test_order_service.py::test_calculate_discount

# Run tests matching pattern
pytest -k "order"

# Show print statements
pytest -s

# Stop at first failure
pytest -x

# Run last failed tests
pytest --lf
```

**Alternative: unittest (Standard Library)**
```bash
# Discover and run tests
python -m unittest discover

# Run specific test
python -m unittest tests.test_order_service.TestOrderService.test_calculate_discount
```

### Coverage Tools

**coverage.py (pytest-cov)**
```bash
# Install
pip install pytest-cov

# Run tests with coverage
pytest --cov=src --cov-report=term --cov-report=html --cov-report=json

# Coverage for specific modules
pytest --cov=src.services --cov=src.domain

# Show missing lines
pytest --cov=src --cov-report=term-missing

# Fail if coverage below threshold
pytest --cov=src --cov-fail-under=80
```

**Configuration (.coveragerc)**
```ini
[run]
source = src
omit =
    */tests/*
    */migrations/*
    */__init__.py

[report]
exclude_lines =
    pragma: no cover
    def __repr__
    raise AssertionError
    raise NotImplementedError
    if __name__ == .__main__.:
    if TYPE_CHECKING:
    @abstractmethod
```

### Linting & Formatting

**Black (Formatter)**
```bash
# Install
pip install black

# Format code
black src/

# Check without modifying
black --check src/

# Show diffs
black --diff src/
```

**isort (Import Sorting)**
```bash
# Install
pip install isort

# Sort imports
isort src/

# Check only
isort --check-only src/
```

**pylint (Linter)**
```bash
# Install
pip install pylint

# Lint code
pylint src/

# Lint with specific rcfile
pylint --rcfile=.pylintrc src/

# Generate config
pylint --generate-rcfile > .pylintrc
```

**flake8 (Style Checker)**
```bash
# Install
pip install flake8

# Check code
flake8 src/

# Ignore specific errors
flake8 --ignore=E501,W503 src/
```

**mypy (Type Checking)**
```bash
# Install
pip install mypy

# Check types
mypy src/

# Strict mode
mypy --strict src/
```

### Quality Metrics

**radon (Complexity & Maintainability)**
```bash
# Install
pip install radon

# Cyclomatic complexity
radon cc src/ -a

# Show only complex functions (> threshold)
radon cc src/ -n C

# Maintainability index
radon mi src/ -s

# Raw metrics
radon raw src/
```

**bandit (Security Scanner)**
```bash
# Install
pip install bandit

# Scan for security issues
bandit -r src/

# Generate JSON report
bandit -r src/ -f json -o bandit-report.json

# Ignore specific issues
bandit -r src/ -s B101,B601
```

### Parsing Coverage Data (Python)

**JSON Format (coverage.json):**
```json
{
  "meta": {
    "version": "5.5",
    "timestamp": "2025-01-15T10:30:00"
  },
  "files": {
    "src/services/order_service.py": {
      "executed_lines": [1, 2, 5, 8, 10],
      "missing_lines": [15, 20],
      "summary": {
        "covered_lines": 5,
        "num_statements": 7,
        "percent_covered": 71.43,
        "missing_lines": 2
      }
    }
  },
  "totals": {
    "covered_lines": 850,
    "num_statements": 1000,
    "percent_covered": 85.0
  }
}
```

**Parsing Strategy:**
```python
import json

with open('coverage.json') as f:
    data = json.load(f)

overall_coverage = data['totals']['percent_covered']
print(f"Overall: {overall_coverage}%")

for file, metrics in data['files'].items():
    coverage = metrics['summary']['percent_covered']
    print(f"{file}: {coverage}%")
```

---

## JavaScript / TypeScript / Node.js

### Testing Frameworks

**Primary: Jest**
```bash
# Run tests
npm test

# Run with coverage
npm test -- --coverage

# Run specific test file
npm test -- order.test.js

# Run tests matching pattern
npm test -- --testNamePattern="calculate discount"

# Watch mode
npm test -- --watch

# Update snapshots
npm test -- -u
```

**Alternative: Vitest**
```bash
# Run tests
npm run test

# Coverage
npm run test -- --coverage

# UI mode
npm run test -- --ui
```

**Alternative: Mocha + Chai**
```bash
# Install
npm install --save-dev mocha chai

# Run tests
npx mocha tests/**/*.test.js

# With coverage (nyc)
npx nyc mocha tests/**/*.test.js
```

### Coverage Tools

**Jest Built-in Coverage**
```json
{
  "jest": {
    "collectCoverage": true,
    "coverageDirectory": "coverage",
    "coverageReporters": ["json", "html", "text", "lcov"],
    "coverageThreshold": {
      "global": {
        "branches": 80,
        "functions": 80,
        "lines": 80,
        "statements": 80
      }
    },
    "collectCoverageFrom": [
      "src/**/*.{js,jsx,ts,tsx}",
      "!src/**/*.test.{js,jsx,ts,tsx}",
      "!src/**/*.spec.{js,jsx,ts,tsx}"
    ]
  }
}
```

**Istanbul (nyc)**
```bash
# Install
npm install --save-dev nyc

# Run with coverage
nyc npm test

# Generate HTML report
nyc --reporter=html --reporter=text npm test

# Check thresholds
nyc --check-coverage --lines 80 --functions 80 npm test
```

**c8 (V8 Coverage)**
```bash
# Install
npm install --save-dev c8

# Run with coverage
c8 npm test

# With thresholds
c8 --check-coverage --lines 80 npm test
```

### Linting & Formatting

**ESLint**
```bash
# Install
npm install --save-dev eslint

# Initialize
npx eslint --init

# Lint code
npx eslint src/

# Fix automatically
npx eslint src/ --fix

# Lint TypeScript
npx eslint src/ --ext .ts,.tsx
```

**Prettier**
```bash
# Install
npm install --save-dev prettier

# Format code
npx prettier --write src/

# Check formatting
npx prettier --check src/

# With ESLint integration
npm install --save-dev eslint-config-prettier eslint-plugin-prettier
```

**TypeScript Compiler**
```bash
# Type check (no emit)
npx tsc --noEmit

# Watch mode
npx tsc --noEmit --watch
```

### Quality Metrics

**complexity-report**
```bash
# Install
npm install --save-dev complexity-report

# Analyze complexity
npx cr src/ --output=complexity.json
```

**jscpd (Copy-Paste Detector)**
```bash
# Install
npm install -g jscpd

# Detect duplicates
jscpd src/

# With threshold
jscpd src/ --threshold 5
```

**dependency-cruiser (Circular Dependencies)**
```bash
# Install
npm install --save-dev dependency-cruiser

# Detect circular dependencies
npx depcruise --validate -- src/

# Generate graph
npx depcruise --output-type dot src/ | dot -T svg > dependencies.svg
```

### Parsing Coverage Data (JavaScript)

**JSON Summary Format:**
```json
{
  "total": {
    "lines": { "total": 1000, "covered": 850, "skipped": 0, "pct": 85 },
    "statements": { "total": 1200, "covered": 1020, "skipped": 0, "pct": 85 },
    "functions": { "total": 150, "covered": 135, "skipped": 0, "pct": 90 },
    "branches": { "total": 200, "covered": 160, "skipped": 0, "pct": 80 }
  },
  "src/services/order-service.ts": {
    "lines": { "total": 50, "covered": 45, "pct": 90 }
  }
}
```

**Parsing Strategy:**
```javascript
const coverage = require('./coverage/coverage-summary.json');
const overall = coverage.total.lines.pct;
console.log(`Overall: ${overall}%`);

for (const [file, metrics] of Object.entries(coverage)) {
  if (file !== 'total') {
    console.log(`${file}: ${metrics.lines.pct}%`);
  }
}
```

---

## Go

### Testing Framework

**Standard Library (testing)**
```bash
# Run tests
go test ./...

# Run with coverage
go test ./... -coverprofile=coverage.out

# Run specific test
go test -run TestOrderService_CalculateDiscount

# Verbose output
go test -v ./...

# Show coverage percentage
go test ./... -cover

# Generate HTML coverage report
go tool cover -html=coverage.out -o coverage.html
```

### Coverage Tools

**Built-in Coverage**
```bash
# Generate coverage profile
go test ./... -coverprofile=coverage.out -covermode=count

# View coverage by function
go tool cover -func=coverage.out

# Coverage for specific package
go test ./services -coverprofile=coverage.out

# Combine coverage from multiple packages
go test ./... -coverprofile=coverage.out -coverpkg=./...
```

### Linting & Formatting

**gofmt (Standard Formatter)**
```bash
# Format code
gofmt -w .

# Check formatting
gofmt -l .

# Show diffs
gofmt -d .
```

**goimports (Auto-import Management)**
```bash
# Install
go install golang.org/x/tools/cmd/goimports@latest

# Format with imports
goimports -w .
```

**golint (Linter)**
```bash
# Install
go install golang.org/x/lint/golint@latest

# Lint code
golint ./...
```

**staticcheck (Advanced Linter)**
```bash
# Install
go install honnef.co/go/tools/cmd/staticcheck@latest

# Check code
staticcheck ./...
```

**golangci-lint (Meta-linter)**
```bash
# Install
go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest

# Run all linters
golangci-lint run

# Run specific linters
golangci-lint run --enable=gofmt,golint,staticcheck
```

### Quality Metrics

**gocyclo (Complexity)**
```bash
# Install
go install github.com/fzipp/gocyclo/cmd/gocyclo@latest

# Check complexity
gocyclo .

# Show functions over threshold
gocyclo -over 10 .
```

**golines (Line Length)**
```bash
# Install
go install github.com/segmentio/golines@latest

# Shorten long lines
golines -w .
```

### Parsing Coverage Data (Go)

**Coverage Output Format:**
```
mode: count
github.com/myapp/services/order.go:10.50,12.2 1 5
github.com/myapp/services/order.go:15.60,17.2 1 0
github.com/myapp/services/order.go:20.30,25.3 4 10
```

**Parsing Strategy:**
```bash
# Extract overall coverage percentage
go test ./... -coverprofile=coverage.out
go tool cover -func=coverage.out | grep total | awk '{print $3}'

# Parse coverage.out for per-file metrics
awk '{print $1}' coverage.out | cut -d: -f1 | sort | uniq
```

---

## Java

### Testing Frameworks

**Primary: JUnit 5**
```bash
# Maven
mvn test

# Run specific test
mvn test -Dtest=OrderServiceTest

# Run specific method
mvn test -Dtest=OrderServiceTest#testCalculateDiscount

# Gradle
./gradlew test

# Run specific test
./gradlew test --tests OrderServiceTest
```

**Alternative: TestNG**
```xml
<!-- Maven pom.xml -->
<dependency>
    <groupId>org.testng</groupId>
    <artifactId>testng</artifactId>
    <version>7.8.0</version>
    <scope>test</scope>
</dependency>
```

### Coverage Tools

**JaCoCo (Primary)**
```xml
<!-- Maven pom.xml -->
<plugin>
    <groupId>org.jacoco</groupId>
    <artifactId>jacoco-maven-plugin</artifactId>
    <version>0.8.10</version>
    <executions>
        <execution>
            <goals>
                <goal>prepare-agent</goal>
            </goals>
        </execution>
        <execution>
            <id>report</id>
            <phase>test</phase>
            <goals>
                <goal>report</goal>
            </goals>
        </execution>
        <execution>
            <id>check</id>
            <goals>
                <goal>check</goal>
            </goals>
            <configuration>
                <rules>
                    <rule>
                        <element>PACKAGE</element>
                        <limits>
                            <limit>
                                <counter>LINE</counter>
                                <value>COVEREDRATIO</value>
                                <minimum>0.80</minimum>
                            </limit>
                        </limits>
                    </rule>
                </rules>
            </configuration>
        </execution>
    </executions>
</plugin>
```

```bash
# Maven - Generate coverage
mvn clean test jacoco:report

# Coverage report location
target/site/jacoco/index.html

# Gradle - Generate coverage
./gradlew test jacocoTestReport
```

### Linting & Formatting

**Checkstyle**
```bash
# Maven
mvn checkstyle:check

# Configuration in pom.xml or checkstyle.xml
```

**PMD (Static Analysis)**
```bash
# Maven
mvn pmd:check

# Generate report
mvn pmd:pmd
```

**SpotBugs (Bug Detector)**
```bash
# Maven
mvn spotbugs:check

# GUI
mvn spotbugs:gui
```

**Google Java Format**
```bash
# Install
curl -o google-java-format.jar https://github.com/google/google-java-format/releases/download/v1.17.0/google-java-format-1.17.0-all-deps.jar

# Format code
java -jar google-java-format.jar --replace $(find src -name "*.java")
```

### Quality Metrics

**SonarQube**
```bash
# Maven
mvn sonar:sonar -Dsonar.host.url=http://localhost:9000

# Gradle
./gradlew sonarqube
```

### Parsing Coverage Data (Java)

**JaCoCo XML Format:**
```xml
<report name="MyApp">
  <counter type="LINE" missed="150" covered="850"/>
  <counter type="BRANCH" missed="44" covered="156"/>
  <package name="com/myapp/services">
    <class name="OrderService">
      <counter type="LINE" missed="5" covered="45"/>
    </class>
  </package>
</report>
```

**Parsing Strategy:**
```bash
# Extract overall coverage using xmllint
xmllint --xpath "//report/counter[@type='LINE']/@covered | //report/counter[@type='LINE']/@missed" target/site/jacoco/jacoco.xml
```

---

## Rust

### Testing Framework

**Standard Library (built-in)**
```bash
# Run tests
cargo test

# Run specific test
cargo test test_calculate_discount

# Show print output
cargo test -- --nocapture

# Run tests matching pattern
cargo test order

# Run ignored tests
cargo test -- --ignored

# Run tests in parallel (default)
cargo test -- --test-threads=4
```

### Coverage Tools

**cargo-tarpaulin**
```bash
# Install
cargo install cargo-tarpaulin

# Run with coverage
cargo tarpaulin --out Html --out Json

# With specific threshold
cargo tarpaulin --fail-under 80

# Coverage for workspace
cargo tarpaulin --workspace
```

**cargo-llvm-cov**
```bash
# Install
cargo install cargo-llvm-cov

# Run with coverage
cargo llvm-cov

# Generate HTML report
cargo llvm-cov --html

# Generate JSON
cargo llvm-cov --json --output-path coverage.json
```

### Linting & Formatting

**rustfmt (Standard Formatter)**
```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

**clippy (Linter)**
```bash
# Install
rustup component add clippy

# Lint code
cargo clippy

# Fail on warnings
cargo clippy -- -D warnings

# Show all lints
cargo clippy -- -W clippy::all
```

### Quality Metrics

**cargo-audit (Security)**
```bash
# Install
cargo install cargo-audit

# Check for vulnerabilities
cargo audit

# Fix vulnerabilities
cargo audit fix
```

**cargo-outdated (Dependencies)**
```bash
# Install
cargo install cargo-outdated

# Check outdated dependencies
cargo outdated
```

### Parsing Coverage Data (Rust)

**Tarpaulin JSON Format:**
```json
{
  "files": {
    "src/services/order.rs": {
      "lines": {
        "10": 5,
        "15": 0,
        "20": 10
      }
    }
  },
  "coverage": 85.5
}
```

---

## Quick Reference Matrix

| Language | Test Command | Coverage Command | Coverage File | Lint Command |
|----------|--------------|------------------|---------------|--------------|
| **.NET** | `dotnet test` | `dotnet test --collect:"XPlat Code Coverage"` | `TestResults/*/coverage.cobertura.xml` | `dotnet format --verify-no-changes` |
| **Python** | `pytest` | `pytest --cov=src --cov-report=json` | `coverage.json` | `pylint src/` |
| **Node.js** | `npm test` | `npm test -- --coverage` | `coverage/coverage-summary.json` | `npx eslint src/` |
| **Go** | `go test ./...` | `go test ./... -coverprofile=coverage.out` | `coverage.out` | `golangci-lint run` |
| **Java** | `mvn test` | `mvn test jacoco:report` | `target/site/jacoco/jacoco.xml` | `mvn checkstyle:check` |
| **Rust** | `cargo test` | `cargo tarpaulin --out Json` | `tarpaulin-report.json` | `cargo clippy` |

---

## Story-Scoped Coverage Commands (STORY-092)

When running `/qa STORY-NNN` or `/dev STORY-NNN`, use story-scoped output paths to enable concurrent test executions without data corruption.

**Variables:**
- `{STORY_ID}` - Story identifier (e.g., STORY-092)
- `results_dir` = `tests/results/{STORY_ID}`
- `coverage_dir` = `tests/coverage/{STORY_ID}`

| Language | Story-Scoped Coverage Command | Coverage File |
|----------|------------------------------|---------------|
| **.NET** | `dotnet test --collect:"XPlat Code Coverage" --results-directory=tests/results/{STORY_ID}` | `tests/results/{STORY_ID}/*/coverage.cobertura.xml` |
| **Python** | `pytest --cov=src --cov-report=json:tests/coverage/{STORY_ID}/coverage.json --junitxml=tests/results/{STORY_ID}/test-results.xml` | `tests/coverage/{STORY_ID}/coverage.json` |
| **Node.js** | `npm test -- --coverage --coverageDirectory=tests/coverage/{STORY_ID}` | `tests/coverage/{STORY_ID}/coverage-summary.json` |
| **Go** | `go test ./... -coverprofile=tests/coverage/{STORY_ID}/coverage.out -json > tests/results/{STORY_ID}/test-results.json` | `tests/coverage/{STORY_ID}/coverage.out` |
| **Java** | `mvn test jacoco:report -Djacoco.destFile=tests/coverage/{STORY_ID}/jacoco.exec` | `tests/coverage/{STORY_ID}/jacoco.xml` |
| **Rust** | `cargo tarpaulin --out Json --output-dir tests/coverage/{STORY_ID}` | `tests/coverage/{STORY_ID}/tarpaulin-report.json` |

**Configuration:** See `devforgeai/config/test-isolation.yaml` for customizing base paths
**Reference:** `test-isolation-service.md` for path resolution algorithm

---

**Reference**: Load this file when determining language-specific commands during QA validation.
