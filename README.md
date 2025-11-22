# Bromcom Timetable Formatter

A small Rust workspace that parses Bromcom-produced PDF timetables and renders a printable A4 SVG-style weekly timetable, with a color-coded timetable grid and an embedded school map highlighting departments.

> [!IMPORTANT]
> I built this for my daughter who has some additional needs and finds it easier to associate colours with rooms; it is based upon parsing the information out of Bromcom circa 2025; PDF parsing is notably brittle and so this may not work without significant fixes for other timetables. I'm happy to accept Pull Requests if you find an issue, ideally with some anonymized examples.

This repository contains two crates:
- `crates/core` — library with parsing, configuration, overrides, map processing and rendering code
- `crates/cli` — command-line utility which wires config + PDF + map -> SVG output

## Quick features

- Parse Bromcom timetable PDFs and reconstruct lessons (Subject, Room, Teacher, Class code)
- Configurable room-to-department mapping with separate background and foreground colors
- Per-week/day/period overrides via `config.toml` to correct parsing errors or make manual adjustments
- Render A4 sized SVGs containing a timetable grid and an embedded school map
- CLI flags to supply student name/form manually (fallback when PDF doesn't contain an extractable name)

```bash
cargo build --release
./target/release/timetable_cli \
  --input input/your_timetable.pdf \
  --config config.toml \
  --map resources/SchoolMap.svg \
  --output output/ \
  --student-name "Richard Slater" \
  --form "11RD"
```

## Output

- SVG files will be written to the `--output` directory (one file per week found in the PDF)

Configuration (`config.toml`)
- `[[mappings]]` — maps room code prefixes to a visual style and a map element id
  - `prefix` — e.g. `MA` (matches MA1, MA2)
  - `bg_color` — background color used for the room sidebar
  - `fg_color` — foreground text color used for the room label
  - `map_id` — the id attribute or data-name in your map SVG to highlight
  - `label` — human-readable department label

- `[[overrides]]` — manual corrections applied after parsing
  - `week` (1-based), `day` (`Monday`..`Friday`), `period` (`PD`, `L1`..`L5`)
  - Optional fields: `subject`, `room`, `teacher`, `class_code`

```toml
[[overrides]]
week = 2
day = "Wednesday"
period = "L3"
subject = "Geography"
room = "HU3"
teacher = "Mr Smith"
```

# Why this project exists

- Bromcom exports visually indistinct timetables as PDFs that are difficult to read for someone with a learning difficulty. This tool reconstructs the timetable grid and produces a clean, print-ready SVG with integrated map highlighting.

## Testing & development

- Core library unit tests live under `crates/core/src` (new tests will be added to ensure parser/config/renderer behavior)
- Run all tests: `cargo test`

### Pre-commit hooks

This repository includes a `.pre-commit-config.yaml` to run common checks before committing and pushing.

- Install pre-commit (requires Python):

```bash
pip install pre-commit
```

- Install hooks locally (one-time):

```bash
pre-commit install
pre-commit install --hook-type pre-push
```

- The hooks configured include:
  - YAML/formatting checks, whitespace trimming
  - `cargo fmt --all -- --check` on commit
  - `cargo clippy --all-targets --all-features -- -D warnings` on commit
  - `cargo test --workspace` on push

Note: Running clippy and tests on every commit/push can be slow; you can adjust or locally skip these checks via `git commit --no-verify` if necessary.

### Contributing & roadmap

- See `docs/TODO.md` for the current short-term roadmap (README, LICENSE, tests, CI, dependabot)
- Open issues for bugs or feature requests. Contributions welcome.

License
- MIT (see `LICENSE`)

Author / Contact
- [Richard Slater](https://richard-slater.co.uk)

