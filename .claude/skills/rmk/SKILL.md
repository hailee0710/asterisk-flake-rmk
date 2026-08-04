---
name: rmk
description: >-
  Comprehensive assistant for RMK (Rust Matrix Keyboard) firmware development.
  Use this skill when the user invokes the /rmk slash command, mentions RMK,
  keyboard firmware in Rust, keyboard.toml, vial.json, matrix pins, keymaps,
  split keyboard configuration, BLE keyboard, nRF52840 keyboard firmware,
  or needs help with keyboard firmware features like macros, tap-dance,
  combos, encoders, pointing devices, or Vial support.
  Also use when debugging RMK build errors, configuring Embassy/nRF SDK,
  or setting up new RMK keyboard projects.
compatibility: Requires RMK 0.8+ (current stable), Rust 2024 edition, Embassy 0.5+
---

# RMK Keyboard Firmware Assistant

You are an expert in the RMK (Rust Matrix Keyboard) firmware framework. Your role is to help users develop, configure, and debug RMK-based keyboard firmware. Always **explain your proposed changes and reasoning first**, then ask for confirmation before editing any files.

## Core Principles

- RMK is a feature-rich keyboard firmware written in Rust, built on the Embassy async runtime
- Configuration lives primarily in `keyboard.toml` (keyboard definition, layout, keymap, behavior) and `vial.json` (Vial layout editor definition)
- Entry points use `#[rmk_central]` and `#[rmk_peripheral]` proc macros (RMK 0.8+)
- The build system uses `build.rs` to generate Vial config and link `memory.x`
- RMK official docs: https://rmk.rs/docs/configuration.html

## Project Anatomy

A typical RMK project structure:
```
project/
├── Cargo.toml              # Dependencies: rmk, embassy-nrf, nrf-sdc, etc.
├── keyboard.toml           # Main configuration file
├── vial.json               # Vial layout editor definition
├── build.rs                # Vial config generation + memory.x linking
├── memory.x                # Flash/RAM layout for the MCU
├── .cargo/config.toml      # Build target + env vars
└── src/
    ├── central.rs          # #[rmk_central] entry point (split: central/left)
    └── peripheral.rs       # #[rmk_peripheral(id = N)] entry point (split: right half)
```

### Cargo.toml — RMK Feature Flags (0.8+)

Common feature combinations for nRF52840 BLE split:
```toml
rmk = { version = "0.8", default-features = false, features = [
    "nrf52840_ble",    # BLE on nRF52840
    "split",           # Split keyboard support
    "async_matrix",    # Async matrix scanning
    "adafruit_bl",     # Adafruit nRF52 bootloader
    "defmt",           # defmt logging
    "storage",         # Flash storage for keymaps/settings
    "vial",            # Vial app support
] }
```

Other useful features: `"col2row"` (diode direction), `"direct_pin"` (direct pin matrix), `"usb"` (USB HID).

## keyboard.toml — Configuration Reference

This is the main configuration file driving the RMK firmware. All sections are documented below.

### `[keyboard]` — Identity
```toml
[keyboard]
name = "My Keyboard"
product_name = "My Keyboard"
vendor_id = 0x4c4b
product_id = 0x4643
manufacturer = "YourName"
board = "nice!nano_v2"       # Pre-defined board (or use "chip" instead)
# chip = "nrf52840"          # Alternative: specify chip directly
serial_number = "vial:f64c2b3c:000001"
usb_enable = true            # USB HID (default: true for most chips)
```

### `[layout]` — Matrix Dimensions & Physical Layout
```toml
[layout]
rows = 5          # Total rows (all splits combined)
cols = 12         # Total cols (all splits combined)
layers = 2        # Number of layers (more = more flash/RAM)
matrix_map = """
(0,0) (0,1) (0,2) ...   # Map each physical key to (row, col)
(1,0) (1,1) (1,2) ...   # Use _ to skip positions
"""
```
Matrix map entries format: `(row, col)` or `(row, col, hand)` for split keyboards. Use `_` or just whitespace for gaps.

### `[matrix]` — Pin Configuration (Non-Split)
```toml
[matrix]
matrix_type = "normal"       # or "direct_pin"
row_pins = ["P0_31", "P0_29", "P0_02", "P0_09", "P1_06"]
col_pins = ["P0_11", "P0_10", "P1_13", "P1_11", "P1_15", "P1_04"]
row2col = false              # Default: col2row. Set true for row2col.
# bootmagic = [0, 0]         # Optional: hold key at (row,col) on boot to enter bootloader
```
For direct pin matrix: use `direct_pins = [["PIN_0", "PIN_1"], ["PIN_2", "_"]]` instead.

### `[split]` — Split Keyboard Configuration
```toml
[split]
connection = "ble"           # "ble" or "serial"

[split.central]
rows = 0                     # Central's own matrix rows (0 if no keys)
cols = 0                     # Central's own matrix cols
row_offset = 0
col_offset = 0
# ble_addr = [0x18, 0xe2, ...]  # Optional: override auto-generated BLE addr
# battery_adc_pin = "vddh"
# adc_divider_measured = 2000
# adc_divider_total = 2806

[split.central.matrix]
matrix_type = "normal"
row_pins = []
col_pins = []

[[split.peripheral]]
rows = 5                     # This peripheral's matrix rows
cols = 6                     # This peripheral's matrix cols
row_offset = 0               # Offset into the global layout
col_offset = 0               # Offset into the global layout
# ble_addr = [0x7e, ...]     # Optional: override auto-generated BLE addr

[split.peripheral.matrix]
matrix_type = "normal"
row_pins = ["P0_31", "P0_29", ...]
col_pins = ["P0_11", "P0_10", ...]
```
Each `[[split.peripheral]]` uses **double brackets** — it's a TOML array of tables. Each peripheral gets its own matrix pins, offsets, and optional input devices.

### `[[layer]]` — Keymaps
```toml
[[layer]]
name = "base"                # Optional layer name
keys = """
esc   Kc1   Kc2   Kc3   Kc4   Kc5   Kc6   Kc7   Kc8   Kc9   Kc0   Minus
Tab   Q     W     E     R     T     Y     U     I     O     P     bsls
MO(1) A     S     D     F     G     H     J     K     L     scln  quot
lsft  Z     X     C     V     B     N     M     Comma Dot   slsh  Equal
      LAlt  LGui  lctl  bspc  Enter  del  Space             lbrc  rbrc
"""
```
Keycodes are case-insensitive. Use `_` or `___` for transparent (passthrough to lower layer). Use `a!(No)` equivalent is just whitespace/empty.

### Common Keycodes
| Category | Examples |
|----------|----------|
| **Alphanumeric** | `A`-`Z`, `Kc0`-`Kc9` |
| **Modifiers** | `LCtrl`, `LAlt`, `LShift`, `LGui`, `RCtrl`, `RAlt`, `RShift`, `RGui` |
| **Navigation** | `Up`, `Down`, `Left`, `Right`, `Home`, `End`, `PageUp`, `PageDown` |
| **Editing** | `Enter`, `Tab`, `Space`, `bspc`, `del`, `Esc`, `pscr`, `Insert` |
| **Punctuation** | `Minus`, `Equal`, `lbrc`, `rbrc`, `bsls`, `scln`, `quot`, `Comma`, `Dot`, `slsh`, `Grave` |
| **Function** | `F1`-`F12` |
| **Media** | `AudioMute`, `AudioVolUp`, `AudioVolDown`, `mply`, `mprv`, `mnxt`, `mstop` |
| **Mouse** | `MouseUp`, `MouseDown`, `MouseLeft`, `MouseRight`, `MouseBtn1`-`MouseBtn5`, `MouseWheelUp`, `MouseWheelDown` |
| **Mod+Key** | `WM(X, LCtrl)` = Ctrl+X, `WM(V, LGui)` = Win+V |
| **System** | `Power`, `Sleep`, `Wake` |

### Layer Actions
| Action | Syntax | Description |
|--------|--------|-------------|
| **Momentary** | `MO(n)` | Activate layer n while held |
| **Toggle** | `TG(n)` | Toggle layer n on/off |
| **Switch To** | `TO(n)` | Switch to layer n, deactivate others |
| **Default** | `DF(n)` | Set default layer to n |
| **One-Shot Layer** | `OSL(n)` | Activate layer n for next keypress |
| **Layer Tap** | `LT(n, key)` | Key on tap, layer n on hold |
| **Layer Mod** | `LM(n, mod)` | Layer n + modifier while held |
| **Tri-Layer** | (config) | Activate adjust layer when both upper+lower held |

### `[aliases]` — Custom Keycode Shortcuts
```toml
[aliases]
MyCut = "WM(X, LCtrl)"
MyCopy = "WM(C, LCtrl)"
MyPaste = "WM(V, LCtrl)"
```
Reference in keymaps with `@MyCut`.

### `[behavior]` — Advanced Behaviors
```toml
[behavior]
# Tri-layer: holding both layer 1 and 2 activates layer 3
tri_layer = { upper = 1, lower = 2, adjust = 3 }

# One-shot: layer/modifier stays active for one keypress
one_shot = { timeout = "1s" }

# One-shot modifiers
one_shot_modifiers = { activate_on_keypress = false, quick_release = false }

# Combos: press keys simultaneously for custom output
[behavior.combo]
timeout = "150ms"
prior_idle_time = "130ms"
combos = [
    { actions = ["J", "K"], output = "Escape" },
]

# Macros
[[behavior.macro.macros]]
operations = [
    { operation = "text", text = "Hello, world!" }
]

[[behavior.macro.macros]]
operations = [
    { operation = "down", keycode = "LShift" },
    { operation = "tap", keycode = "H" },
    { operation = "up", keycode = "LShift" },
    { operation = "delay", duration = "100ms" },
    { operation = "tap", keycode = "I" },
]

# Morse / Tap-Dance
[behavior.morse]
enable_flow_tap = true
prior_idle_time = "120ms"
hold_timeout = "250ms"
gap_timeout = "250ms"

morses = [
    # Single-tap = F1, double-tap = F2, hold = layer 1
    { tap = "F1", double_tap = "F2", hold = "MO(1)" },
    # Extended form
    { tap_actions = ["F1", "F2", "F3"], hold_actions = ["MO(1)", "MO(2)", "MO(3)"] },
]

# Custom tap-hold profiles
[behavior.morse.profiles]
HRM = { unilateral_tap = true, permissive_hold = true, hold_timeout = "250ms", gap_timeout = "250ms" }
```
Reference a custom profile in keymap with: `LT(1, A, HRM)`.

### `[ble]` — Bluetooth Configuration
```toml
[ble]
enabled = true
# ble_profiles_num = 3        # Number of BLE profiles (in [rmk] section)
# battery_adc_pin = "vddh"    # Battery voltage sensing
# adc_divider_measured = 2000  # For voltage divider (e.g., nice!nano)
# adc_divider_total = 2806
```
For nice!nano: measured=2000, total=2806 (806K + 2M divider, measuring on 2M). Battery can also be set per-peripheral in `[split.central]` and `[[split.peripheral]]`.

### `[storage]` — Flash Storage
```toml
[storage]
enabled = true
num_sectors = 2               # Flash sectors for storage (≥2)
# start_addr = 0x00000000     # Auto-allocated from end if not set
clear_storage = false          # Set true to wipe storage each boot (testing)
```

### `[rmk]` — Internal Parameters
```toml
[rmk]
mouse_key_interval = 20
mouse_wheel_interval = 80
debounce_time = 20             # ms
combo_max_num = 8
combo_max_length = 4
morse_max_num = 8
max_patterns_per_key = 36
macro_space_size = 256
report_channel_size = 16
vial_channel_size = 4
flash_channel_size = 4
ble_profiles_num = 3
split_peripherals_num = 0      # Auto-detected, but can override
split_central_sleep_timeout_seconds = 0  # 0 = never sleep
```

### Pointing Devices (Trackball, Touchpad)
Added per-peripheral in `keyboard.toml`:
```toml
[[split.peripheral]]
# ... matrix config ...
[split.peripheral.input_device.pmw3610]
name = "trackball0"
spi.instance = "bitbang0"
spi.sck = "P1_00"
spi.mosi = "P0_24"
spi.miso = "P0_11"
spi.cs = "P0_22"
force_awake = true
smart_mode = true
cpi = 800
invert_x = true
invert_y = true
```
Then register the processor in `central.rs`:
```rust
#[rmk_central]
mod keyboard_central {
    use super::*;

    #[register_processor(event)]
    fn pointing_processor_controller() -> PointingProcessorController {
        PointingProcessorController::new()
    }
}
```

## vial.json — Vial Layout Editor

The `vial.json` defines how your keyboard appears in the Vial desktop app. Key requirements:
- **`matrix.rows`** and **`matrix.cols`** must match `keyboard.toml` layout
- **`layouts.keymap`** defines the visual key positions using Keyboard Layout Editor format
- `vendorId`/`productId` should match `keyboard.toml`
- Custom keycodes for BLE profiles/output switching go in `customKeycodes` array

The build.rs automatically compresses `vial.json` with XZ and embeds it in the firmware.

## Source Code Entry Points

### Central (BLE split master)
```rust
#![no_main]
#![no_std]

use rmk::macros::rmk_central;

#[rmk_central]
mod keyboard_central {}
```
The central module can register pointing device processors, custom handlers, etc.

### Peripheral (BLE split slave)
```rust
#![no_main]
#![no_std]

use rmk::macros::rmk_peripheral;

#[rmk_peripheral(id = 0)]
mod keyboard_peripheral {}
```
Each peripheral needs a unique `id`. The id corresponds to the order of `[[split.peripheral]]` entries in `keyboard.toml`.

## Build & Flash

### Configuration
`.cargo/config.toml`:
```toml
[build]
target = "thumbv7em-none-eabihf"

[env]
DEFMT_LOG = "debug"
KEYBOARD_TOML_PATH = { value = "keyboard.toml", relative = true }
```

### memory.x (nRF52840 with Adafruit bootloader)
```
MEMORY
{
  FLASH : ORIGIN = 0x00001000, LENGTH = 1020K
  RAM : ORIGIN = 0x20000008, LENGTH = 255K
}
```

### Build Commands
```shell
cargo build --release --bin central     # Build central firmware
cargo build --release --bin peripheral  # Build peripheral firmware
cargo run --release --bin central       # Build + flash central
```

## Common Tasks

### Adding a New Layer
1. Increment `layout.layers` in `keyboard.toml`
2. Add a new `[[layer]]` block with a `keys` string matching the `matrix_map` entries

### Changing Pin Assignments
1. Update `row_pins`/`col_pins` in the appropriate `[matrix]` or `[split.peripheral.matrix]` sections
2. Pin names use nRF port notation: `P0_31`, `P1_04`, etc.

### Adding BLE Profiles
1. Set `ble_profiles_num` in `[rmk]` section
2. Add custom keycodes in `vial.json` for profile switching (User0-UserN map to BT0-BTN)

### Converting Non-Split to Split
1. Replace `[matrix]` with `[split]` configuration
2. Add `"split"` feature to Cargo.toml
3. Create `src/central.rs` and `src/peripheral.rs`
4. Add `[[bin]]` entries for both binaries
5. Update `layout.rows`/`cols` to cover all splits
6. Adjust `matrix_map` with hand assignments as needed

### Troubleshooting Build Errors
- **Linker errors about memory**: Check `memory.x` ORIGIN/LENGTH match your bootloader
- **`#[rmk_central]`/`#[rmk_peripheral]` not found**: Ensure `rmk` 0.8+ with correct features
- **Pin not found**: Verify pin names match nRF52 datasheet (`P0_31`, not `P0.31`)
- **Vial not connecting**: Check `clear_storage = true` is NOT set, verify `vial.json` layout matches `keyboard.toml`
- **BLE not advertising**: Verify `[ble] enabled = true` and `[split] connection = "ble"`

## When You Don't Know

- RMK official docs: https://rmk.rs/docs/configuration.html
- RMK GitHub: https://github.com/haobogu/rmk
- Context7 RMK docs: use `/haobogu/rmk` library ID for detailed queries
- For nRF52840 hardware questions, consult the nRF52840 Product Specification
- For Embassy questions, consult Embassy docs at https://embassy.dev
