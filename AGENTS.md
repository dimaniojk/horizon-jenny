# Working on a native Horizon mod

Read README.md and compatibility.json before editing. Keep this repository
separate from Horizon and horizon-mixins. Only export committed pinned Horizon
sources into ignored target trees; never modify the original checkout or player
resources. Do not commit binaries, exported sources or Minecraft assets.

The Cargo library and injected runtime module share runtime.rs. Keep helper
functions small and gameplay logic testable there. All AST changes go through
horizon-mixins plan/apply; do not add string-based Rust source rewriting.

Validate with cargo fmt, clippy with warnings denied, cargo test and
python3 tools/test_template.py. For bundle/compatibility changes also run
python3 tools/test_stage.py and python3 tools/horizon.py check/test against the
pinned source export. Keep validation claims and supported revisions honest.
