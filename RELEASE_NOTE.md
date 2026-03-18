# Stratum 0.1.4

## 0.1.4 - 2026-03-17

### Highlights

- Fixed shims to target the layer-root `inscribe-cli` binary instead of a nonexistent `bin` path.
- Removed stale shims when no active version is selected so old targets do not linger.
- Hardened Windows PATH updates by using PowerShell persistence with quieter permission fallback behavior.

### Artifacts

- `stratum-0.1.4-windows-x64.zip`
- `stratum-0.1.4-linux-x64.tar.gz`
