## Context

The CLI accepts paths for input PDFs, TOML configuration, optional SVG maps, and generated SVG output. CodeQL traces those values to file-system APIs and reports path injection because they have no containment boundary.

## Goals / Non-Goals

**Goals:**
- Allow files only within the current project directory.
- Canonicalize existing paths before reading them, including symlink resolution.
- Create output only in a relative project-local directory.
- Preserve normal project-relative CLI usage.

**Non-Goals:**
- Sandboxing arbitrary library consumers.
- Supporting inputs outside the current project directory.
- Preventing a privileged local user from changing files they already control.

## Decisions

- Provide a shared `path_safety` module rather than duplicate checks at each sink. This keeps read/write containment rules consistent.
- Canonicalize existing files and their parent directories, then require `starts_with(current_directory)`. Canonicalization prevents `..` and symlink escapes; string filtering alone would not.
- Require relative output directories and create them below the current directory. This safely supports the documented `--output output/` workflow while rejecting absolute and traversal paths.
- Enforce the checks both at CLI ingress and core renderer/map file access. CLI validation provides clear errors and core checks protect direct library use.

## Risks / Trade-offs

- [Breaking path restriction] → Document that CLI paths must be project-local and provide a clear invalid-path error.
- [Time-of-check/time-of-use race] → Canonicalize immediately before file operations; fully eliminating races requires platform-specific descriptor APIs and is out of scope.
- [Tests need writable project-local paths] → Use unique, cleaned-up files under the test working directory.
