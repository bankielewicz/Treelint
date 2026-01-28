# OS-Specific Installer Generation Reference

This reference documents the InstallerGenerator module for generating platform-specific installer configurations.

---

## Installer Format Matrix

| Platform | Format | File Extension | Tool Required | Build Command | Config Generated |
|----------|--------|----------------|---------------|---------------|------------------|
| Windows | MSI | .msi | WiX Toolset | `candle *.wxs && light *.wixobj` | .wxs (XML) |
| Windows | EXE | .exe | NSIS | `makensis installer.nsi` | .nsi (Script) |
| Linux (Debian) | DEB | .deb | dpkg-deb | `dpkg-deb --build package` | DEBIAN/ directory |
| Linux (RHEL) | RPM | .rpm | rpmbuild | `rpmbuild -bb package.spec` | .spec (Spec file) |
| macOS | PKG | .pkg | pkgbuild | `pkgbuild ... && productbuild ...` | distribution.xml + scripts |

---

## InstallerConfig Dataclass

```python
@dataclass
class InstallerConfig:
    platform: str          # windows, linux_deb, linux_rpm, macos
    format: str            # msi, nsis, deb, rpm, pkg
    config_path: str       # Path to generated configuration
    build_command: str     # Shell command to build installer (optional)
    tool_required: str     # Tool needed (wix, nsis, dpkg-deb, rpmbuild, pkgbuild)
    tool_available: bool   # True if tool is installed
    metadata: Dict         # Platform-specific metadata (GUIDs, dependencies)
```

---

## Platform-Specific Configuration Details

### Windows MSI (WiX)

**Generated File:** `{name}.wxs`

**Template Structure:**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<Wix xmlns="http://schemas.microsoft.com/wix/2006/wi">
    <Product Id="{PRODUCT_GUID}" UpgradeCode="{UPGRADE_GUID}" ...>
        <Package ... />
        <Directory Id="TARGETDIR">
            <Directory Id="ProgramFilesFolder">
                <Directory Id="INSTALLFOLDER">
                    <Component Id="...">
                        <File Id="..." Source="..." />
                    </Component>
                </Directory>
            </Directory>
        </Directory>
        <Feature Id="ProductFeature">
            <ComponentRef Id="..." />
        </Feature>
    </Product>
</Wix>
```

**Customization Variables:**
| Variable | Description | Default |
|----------|-------------|---------|
| `name` | Product name | Package name |
| `version` | Product version | 1.0.0 |
| `manufacturer` | Company name | From package_info |
| `description` | Product description | Empty |
| `files` | List of files to install | From package contents |

---

### Windows NSIS

**Generated File:** `{name}.nsi`

**Template Structure:**
```nsis
!include "MUI2.nsh"

Name "{name}"
OutFile "{name}-{version}-setup.exe"
InstallDir "$PROGRAMFILES\{name}"

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES

Section "Install"
    SetOutPath "$INSTDIR"
    File "{files}"
    WriteUninstaller "$INSTDIR\uninstall.exe"
SectionEnd

Section "Uninstall"
    Delete "$INSTDIR\*"
    RMDir "$INSTDIR"
SectionEnd
```

**Customization Variables:**
| Variable | Description | Default |
|----------|-------------|---------|
| `name` | Installer name | Package name |
| `version` | Version string | 1.0.0 |
| `publisher` | Publisher name | From package_info |
| `files` | Files to include | From package contents |

---

### Linux Debian

**Generated Directory:** `DEBIAN/`

**Files Generated:**
1. `control` - Package metadata
2. `postinst` - Post-installation script
3. `prerm` - Pre-removal script

**control Template:**
```
Package: {name}
Version: {version}
Section: misc
Priority: optional
Architecture: {architecture}
Maintainer: {maintainer}
Description: {description}
Depends: {dependencies}
```

**Customization Variables:**
| Variable | Description | Default |
|----------|-------------|---------|
| `name` | Package name (lowercase, no spaces) | Sanitized package name |
| `version` | Version string | 1.0.0 |
| `architecture` | Target arch | all |
| `maintainer` | Maintainer email | Unknown |
| `description` | Package description | No description |
| `dependencies` | Comma-separated deps | Empty |

---

### Linux RPM

**Generated File:** `{name}.spec`

**Template Structure:**
```spec
Name:           {name}
Version:        {version}
Release:        {release}%{?dist}
Summary:        {summary}

License:        {license}
URL:            {url}

%description
{description}

%prep
%autosetup

%build
%configure
%make_build

%install
%make_install

%files
%license LICENSE
{files}

%post
# Post-installation script

%changelog
* {date} {maintainer} - {version}-{release}
- Initial package
```

**Customization Variables:**
| Variable | Description | Default |
|----------|-------------|---------|
| `name` | Package name | Package name |
| `version` | Version string | 1.0.0 |
| `release` | Release number | 1 |
| `summary` | Short description | First 80 chars of description |
| `license` | License type | MIT |
| `files` | File list | %{_bindir}/* |

---

### macOS PKG

**Generated Files:**
1. `distribution.xml` - Product distribution definition
2. `scripts/postinstall` - Post-installation script
3. `build.sh` - Build helper script

**distribution.xml Template:**
```xml
<?xml version="1.0" encoding="utf-8"?>
<installer-gui-script minSpecVersion="1">
    <title>{title}</title>
    <organization>{identifier}</organization>
    <domains enable_localSystem="true"/>
    <options customize="never" require-scripts="true"/>

    <pkg-ref id="{identifier}"/>

    <choices-outline>
        <line choice="default">
            <line choice="{identifier}"/>
        </line>
    </choices-outline>

    <choice id="default"/>
    <choice id="{identifier}" visible="false">
        <pkg-ref id="{identifier}"/>
    </choice>

    <pkg-ref id="{identifier}" version="{version}">{name}.pkg</pkg-ref>
</installer-gui-script>
```

**Customization Variables:**
| Variable | Description | Default |
|----------|-------------|---------|
| `title` | Installer title | Package name |
| `identifier` | Bundle identifier | com.example.{name} |
| `version` | Version string | 1.0.0 |

---

## Template Customization Guide

### How to Customize Templates

The InstallerGenerator uses string formatting with `{variable}` placeholders. To customize:

1. **Modify package_info dict** - Pass custom values when calling `generate()`:
   ```python
   package_info = {
       "name": "MyApp",
       "version": "2.0.0",
       "manufacturer": "My Company",
       "description": "My awesome application",
       "files": ["app.exe", "data.dll"],
       "dependencies": ["python3", "libssl"],
   }
   generator.generate("windows", "msi", package_info)
   ```

2. **Override specific fields** - Any field in package_info takes precedence:
   ```python
   # Custom GUID for Windows installer
   package_info["product_guid"] = "YOUR-CUSTOM-GUID"
   ```

3. **Platform-specific metadata** - Use metadata dict for advanced options:
   ```python
   package_info["metadata"] = {
       "architecture": "amd64",
       "license": "Apache-2.0",
   }
   ```

### Common Customization Scenarios

#### Add Custom Dependencies (Linux)
```python
package_info["dependencies"] = ["python3", "nodejs", "libssl-dev"]
```

#### Custom Installation Directory (Windows)
The default is `$PROGRAMFILES\{name}`. To change, modify the WiX INSTALLFOLDER or NSIS InstallDir in the generated config before building.

#### Add Pre/Post Scripts
Post-installation scripts are auto-generated with basic templates. To customize:
1. Generate the config
2. Edit the generated script files
3. Build the installer

### Tool Detection

InstallerGenerator detects tools using `shutil.which()`:

| Tool | Executables Checked |
|------|---------------------|
| wix | candle, light, wix |
| nsis | makensis |
| dpkg-deb | dpkg-deb |
| rpmbuild | rpmbuild |
| pkgbuild | pkgbuild |

If a tool is not found, `tool_available=False` is set but the config is still generated (BR-001).

---

## Business Rules

| ID | Rule | Behavior |
|----|------|----------|
| BR-001 | Config generated even if tool missing | `tool_available=False`, log info |
| BR-002 | Unique GUIDs per Windows installer | `uuid.uuid4()` for each generation |
| BR-003 | Linux packages declare dependencies | Parse from `package_info["dependencies"]` |
| BR-004 | Scripts are platform-appropriate | Bash for Linux/macOS, batch for Windows |

---

## API Reference

### InstallerGenerator

```python
class InstallerGenerator:
    def __init__(self, output_dir: Optional[str] = None):
        """Initialize with output directory (defaults to cwd)."""

    def generate(
        self,
        platform: str,
        format: str,
        package_info: Dict[str, Any],
        output_subdir: Optional[str] = None,
    ) -> InstallerConfig:
        """Generate installer config for single platform."""

    def generate_all(
        self,
        package_info: Dict[str, Any],
        platforms: Optional[List[str]] = None,
    ) -> List[InstallerConfig]:
        """Generate configs for all platforms (or specified subset)."""

    def detect_tool(self, tool_name: str) -> bool:
        """Check if build tool is installed."""

    def extract_file_list(self, package_path: str) -> List[str]:
        """Extract file list from ZIP archive."""
```

---

## Related Stories

- **STORY-241**: Language-Specific Package Creation Module (dependency)
- **STORY-242**: OS-Specific Installer Generation Module (this feature)
- **EPIC-037**: Release Skill Package & Installer Generation (parent epic)

---

## Change Log

| Date | Change |
|------|--------|
| 2026-01-07 | Initial creation (STORY-242) |
