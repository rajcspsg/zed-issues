# Zed Issue #46348 - Task Names in Which-Key Popup Menu

**Issue URL:** https://github.com/zed-industries/zed/issues/46348

## Description
The which-key popup menu shows generic "task: spawn" description instead of the custom task label when using task keymaps.

## Expected Behavior
When triggering a multi-key sequence bound to a task, the which-key popup should display the task's custom label (e.g., "Build Test Program", "Run Test Program", "Clean Build").

## Actual Behavior
The popup shows generic "task::Spawn" instead of the specific task names.

## Setup

This reproduction case includes:

### 1. Tasks Configuration (`.zed/tasks.json`)
Three tasks with custom labels:
- "Build Test Program" - runs `make`
- "Run Test Program" - runs `./test_program`
- "Clean Build" - runs `make clean`

### 2. Keymap Configuration (`.zed/keymap.json`)
Current keybindings:
- `cmd-shift-r` → Run Test Program (single-key, working)
- `cmd-' b` → Build Test Program (multi-key sequence)
- `cmd-' r` → Run Test Program (multi-key sequence)
- `cmd-' c` → Clean Build (multi-key sequence)

## Reproduction Steps

1. Open this folder (`zed-issue-42994`) in Zed
2. Press `Cmd+'` (apostrophe key) and **wait** for 1-2 seconds (don't press the next key immediately)
3. The which-key popup should appear showing available next keys (b, r, c)

### Expected Result
The popup should show:
```
b - Build Test Program
r - Run Test Program
c - Clean Build
```

### Actual Result (Bug)
The popup shows:
```
b - task::Spawn
r - task::Spawn
c - task::Spawn
```

## Current Status

**Tested on:** Zed 0.230.1, macOS

**Findings:**
- Tasks are accessible and work from the command palette (Cmd+Shift+P → search "task")
- Single-key bindings work (`cmd-shift-r` successfully spawns tasks)
- Multi-key sequences don't appear to trigger the which-key popup
- The which-key feature may not be available in stable Zed releases yet

## Alternative Testing

If multi-key sequences aren't working, you can test with the single working binding:

1. Press `Cmd+Shift+R` - this should run the test program
2. Check if any popup or hint appears

## Notes

- The which-key feature was mentioned in issue #46348 filed with Zed v0.219.2+preview
- The feature may only be available in preview/nightly builds
- Multi-key sequences may need additional configuration or may not be fully implemented yet
- Original issue was reported on Windows; behavior may differ on macOS

## Manual Verification

To verify the tasks work (even without the popup):
- Open command palette: `Cmd+Shift+P`
- Type "task spawn" and select a task
- Or use `Cmd+Shift+R` to run the test program directly

Check the terminal output to confirm the commands are executing.
