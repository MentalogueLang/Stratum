# Release Notes

## 0.1.3 - 2026-03-17

### Highlights

- Added automatic shim setup on first run, including PATH registration for `inscribe`.
- Added `stratum remove` for uninstalling installed layers.
- Avoided PATH truncation on Windows by persisting PATH via PowerShell.

### Artifacts

- `stratum-0.1.3-windows-x64.zip`
- `stratum-0.1.3-linux-x64.tar.gz`

## 0.1.2 - 2026-03-16

### Highlights

- Added Windows `inscribe.cmd` shim that targets the active Stratum version.

### Artifacts

- `stratum-0.1.2-windows-x64.zip`
- `stratum-0.1.2-linux-x64.tar.gz`

## 0.1.1 - 2026-03-16

### Highlights

- Added GitHub release workflow for packaging Stratum binaries.
- Added GitHub release download + unpack for `stratum install`.
- Added global version selection with `stratum use`.

### Artifacts

- `stratum-0.1.1-windows-x64.zip`
- `stratum-0.1.1-linux-x64.tar.gz`

## 0.1.0 - 2026-03-16

### Highlights

- Added pinfile support for `.stratum` with directory resolution.
- Added layer inventory helpers for installed versions.
- Added CLI wiring for `pin`, `list`, `which`, `use`, and `install`.

### Artifacts

- `stratum-0.1.0-windows-x64.zip`
- `stratum-0.1.0-linux-x64.tar.gz`
