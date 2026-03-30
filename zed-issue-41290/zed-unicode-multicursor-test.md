# Zed Unicode Multicursor Bug Reproduction

## Issue
Multicursor and Unicode Mode Malfunction on Linux
Issue: https://github.com/zed-industries/zed/issues/41290

## Environment
- Zed Version: v0.206.7
- OS: Linux Wayland Ubuntu 24.04
- Desktop Environment: Standard Ubuntu Gnome

## Test Cases

### Test Case 1: Basic Unicode Insertion
Line 1:
Line 2:
Line 3:

### Test Case 2: Text Before Unicode Character
Line 1: Quote here:
Line 2: Quote here:
Line 3: Quote here:

### Test Case 3: Mixed Content
Start
Middle
End

## Steps to Reproduce

1. Open this file in Zed editor
2. Navigate to Test Case 1
3. Click at the end of "Line 1: " while holding Alt
4. Continue holding Alt and click at the end of "Line 2: " and "Line 3: "
5. You should now have 3 cursors active
6. Press Ctrl+Shift+U to enter Unicode mode
7. Type: 00ab (this is the code for «)
8. Press Enter to insert the character

### Expected Result
All three lines should have the « character inserted:
```
Line 1: «
Line 2: «
Line 3: «
```

### Actual Result (Bug)
- Inconsistent character insertion
- Some locations get the correct character «
- Other locations get the literal text "00ab"
- Cursor positions may change unpredictably
- Sometimes characters are deleted

## Additional Unicode Characters to Test

- 00bb = » (right-pointing double angle quotation mark)
- 2022 = • (bullet)
- 2014 = — (em dash)
- 03B1 = α (Greek letter alpha)
- 2764 = ❤ (heart)

## Notes

- This issue only occurs with multicursor + Unicode mode
- Regular multicursor text insertion works fine
- Single cursor Unicode insertion works fine
- The bug is specific to the combination of both features
