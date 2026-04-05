# Zed Issue #52163: Python Debugger justMyCode Not Working

This repository reproduces the issue where the Python debugger's `justMyCode` setting doesn't work as expected in Zed.

## Issue Description

When `justMyCode` is set to `false` in the debugger configuration, the debugger should allow stepping into external library code (like standard library modules). However, the debugger acts as if `justMyCode` is set to `true` and never enters external code when stepping into functions.

**Issue Link:** https://github.com/zed-industries/zed/issues/52163

## Setup

1. Open this directory in Zed
2. Ensure you have Python installed and Debugpy available
3. The debugger configuration is already set up in `.zed/tasks.json`

## Reproduction Steps

1. Open `test_debugger.py` in Zed
2. Set a breakpoint on line 31 (inside the `main()` function, on the line with `test_data = {`)
3. Start debugging using the "python debug" configuration
4. When the breakpoint hits, use "Step Into" (F11) to step into the `process_data()` function
5. Once inside `process_data()`, try to "Step Into" the `json.dumps()` call on line 18

### Expected Behavior

With `justMyCode: false`, the debugger should step into the `json.dumps()` implementation from the Python standard library.

### Actual Behavior

The debugger steps over `json.dumps()` as if `justMyCode` were set to `true`, treating it like a single step instead of allowing you to step into the external library code.

## Test File Details

The `test_debugger.py` file:
- Imports `json` and `pathlib.Path` from the standard library
- Contains a `process_data()` function that calls `json.dumps()` and `Path.cwd()`
- These are good test cases because they're common standard library functions that should be steppable when `justMyCode: false`

## Debugger Configuration

The configuration in `.zed/tasks.json`:

```json
{
  "label": "python debug",
  "program": "${file}",
  "type": "python",
  "request": "launch",
  "adapter": "Debugpy",
  "console": "integratedTerminal",
  "justMyCode": false
}
```

## Environment

Original issue reported on:
- Zed version: v0.228.0+stable
- OS: Linux Wayland pop 24.04
- Architecture: x86_64

## Notes

The `justMyCode` parameter is a standard Debugpy configuration option that should control whether the debugger restricts navigation to user-written code only. When set to `false`, it should allow debugging of library and framework code.


## Heelo
