## ADDED Requirements

### Requirement: Project-local file access
The system SHALL canonicalize every existing file path used to read a timetable input, configuration, or map and SHALL reject paths that resolve outside the current project directory.

#### Scenario: Project-local map is processed
- **WHEN** the user supplies an existing map file below the current project directory
- **THEN** the system processes the map using its canonical path

#### Scenario: Traversal or symlink escape is supplied
- **WHEN** a file path resolves outside the current project directory
- **THEN** the system rejects the path before reading the file

### Requirement: Contained output generation
The system SHALL create generated SVG files only under a relative output directory within the current project directory.

#### Scenario: Relative output directory is supplied
- **WHEN** the user supplies `output` as the output directory
- **THEN** the system creates the directory if needed and writes generated SVGs beneath it

#### Scenario: Absolute or parent-traversal output directory is supplied
- **WHEN** the user supplies an output directory containing a root, platform prefix, or parent traversal component
- **THEN** the system rejects the output directory before creating files
