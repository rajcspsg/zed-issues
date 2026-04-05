# Zed Issue #42994 - Debuggee Programs Remain Running After Closing Zed

## Issue Description
When debugging programs in Zed and closing the editor without manually terminating the debug sessions, the debuggee programs continue running in the background. This requires manual cleanup using `killall` or similar commands.

## Original Issue
https://github.com/zed-industries/zed/issues/42994

## Affected Version
- Zed v0.212.6
- OS: Linux Wayland Fedora 42 (likely affects other Linux distributions)
- Architecture: x86_64

## Prerequisites
- Zed editor installed
- C++ compiler (g++ or clang++)
- Debugger (lldb or gdb)
- Linux system (where the bug is reported)

## Setup

### 1. Build the test program
```bash
cd zed-issue-42994
make
```

Or manually:
```bash
g++ -g -std=c++11 -o test_program test_program.cpp
```

### 2. Verify the program works
```bash
./test_program
```

You should see output counting from 1 to 60 seconds, with the PID displayed.

## Steps to Reproduce

### Method 1: Using Zed's Debug Panel

1. Open the `zed-issue-42994` folder in Zed
2. Open the debug panel (View → Debug or keyboard shortcut)
3. Start a debug session:
   - Select the "Debug test_program" configuration
   - Click "Start Debugging" or press F5
4. Wait for the program to start (you should see output in the debug console)
5. **WITHOUT stopping the debugger**, close Zed completely
6. Open a terminal and check for running processes:
   ```bash
   ps aux | grep test_program
   ```

### Method 2: Multiple Debug Sessions

1. Open the `zed-issue-42994` folder in Zed
2. Start the first debug session (F5)
3. Start a second debug session (if Zed allows multiple sessions)
4. Close Zed without stopping either session
5. Check for orphaned processes:
   ```bash
   ps aux | grep test_program
   pgrep -a test_program
   ```

## Expected Behavior
When Zed is closed, all debuggee programs should be automatically terminated. No orphaned processes should remain running.

## Actual Behavior (Bug)
The `test_program` process continues running in the background after Zed is closed. The process must be manually killed:
```bash
killall test_program
# or
pkill test_program
# or find the PID and kill it
kill <PID>
```

## Why This Test Program?

The test program:
- Runs for 60 seconds (gives plenty of time to reproduce the bug)
- Prints its PID (makes it easy to identify in process lists)
- Outputs regular messages (helps verify it's still running)
- Is simple and has no external dependencies

## Technical Details

This bug occurs because:
- Zed doesn't properly send termination signals to debuggee processes on exit
- The debug adapter (lldb/gdb) loses connection but doesn't clean up child processes
- On Linux, orphaned debug processes don't receive SIGTERM when the parent closes

## Cleanup

If you have orphaned processes after testing:
```bash
# Find all test_program instances
ps aux | grep test_program

# Kill all instances
killall test_program

# Or be more specific
pkill -f test_program
```

## Testing on macOS

While this bug is reported on Linux, you can test similar behavior on macOS:
1. Follow the same reproduction steps
2. Check for orphaned processes:
   ```bash
   ps aux | grep test_program
   ```
3. The behavior may differ due to different process management between Linux and macOS

## Verification

To verify the bug:
1. Note the PID when the program starts
2. Close Zed
3. Check if that PID is still running: `ps -p <PID>`
4. If the process exists, the bug is present
5. If the process doesn't exist, the bug is fixed

## Additional Notes

- This issue affects any debugging session, not just C++
- It's particularly problematic with long-running programs or servers
- Multiple debug sessions compound the problem
- Manual cleanup is required to prevent system resource usage
