# urine

User Region Identification and Naming Exosystem

## Purpose

`urine` is a Windows-only utility for interactively selecting a rectangular region on the screen (typically the letter grid in Word Play) and optionally naming it. The selected region will be used by downstream tools for screen capture and AI processing.

## Implementation Plan

### 1. Argument Parsing
- Use `clap` derive for argument parsing.
- Supported arguments:
  - `--debug`: Enable debug-level tracing/logging.
  - `--prompt <TEXT>`: Optional prompt text to display in the center of the main monitor.

### 2. Tracing Initialization
- Use a shared helper (`word_player_shared::init_tracing`) to initialize tracing based on the `--debug` flag.

### 3. Bevy App for Region Selection
- Use `bevy` to create transparent, borderless windows on each monitor.
- Darken the entire screen except for the region being selected (scissored/excluded rectangle).
- Display the prompt text (if provided) in large font on the center monitor.
- Handle mouse input:
  - On left mouse button down: start region selection.
  - On mouse move: update the selection rectangle.
  - On left mouse button up: finalize the region.
- Draw a visible rectangle for the selected region.
- Allow the user to cancel (e.g., with `Esc`).

### 4. Output
- Print the selected region's coordinates (x, y, width, height) to stdout in a machine-readable format (e.g., JSON or CSV).
- Optionally, allow naming the region for future reference (future feature).

### 5. Coding Style
- Follow idioms from `aura-exporter` and `youre-muted-btw` (argument parsing, automata-style state management, modularity).
- Use clear, concise logging and error handling.

### 6. Future Enhancements
- Support saving/loading named regions.
- Add more user feedback (sound, animation).

---

## Dependencies
- `bevy` (for windowing, rendering, input)
- `clap` (for argument parsing)
- `tracing`/`tracing-subscriber` (for logging)
- `word_player_shared` (for shared helpers)

---

## Usage Example

```pwsh
urine.exe --prompt "Select the letter grid" --debug
```

---

## Status
- [x] Project scaffolded
- [ ] Bevy region selection implemented
- [ ] Output format finalized
- [ ] User feedback and polish
