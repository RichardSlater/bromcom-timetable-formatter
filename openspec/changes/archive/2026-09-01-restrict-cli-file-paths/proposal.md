## Why

CodeQL reports seven high-severity path-injection alerts because CLI-controlled paths reach file-system reads and writes without a containment check. Restricting file access to the working project prevents traversal and unintended access outside the user's project.

## What Changes

- Add reusable path validation that canonicalizes existing files and restricts read/write targets to the current project directory.
- Validate CLI input, configuration, map, and output paths before processing them.
- **BREAKING**: CLI input/config/map paths must be inside the current project directory; `--output` must be a relative directory within it.
- Document the path constraint and update tests to use safe project-local files.

## Capabilities

### New Capabilities
- `project-file-boundaries`: Validate file-system paths so timetable processing cannot access files outside the current project directory.

### Modified Capabilities
- None.

## Impact

- Affects `timetable_core` file access and `timetable_cli` argument handling.
- No dependency or configuration-format changes.
