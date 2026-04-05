# Zed Issue #46602 - Multicursor Collapse Bug

## Issue Description
Multicursor behavior issue where selecting multiple sequential identical characters and typing causes cursors to collapse incorrectly.

## Original Issue
https://github.com/zed-industries/zed/issues/46602#issuecomment-4140742057

## Affected Version
- Zed v0.217.3
- OS: Linux Wayland Ubuntu 24.04 (but may affect other platforms)

## Steps to Reproduce

1. Open `test.txt` in Zed
2. The file contains: `miks on piiiiks parem kui prääääks?`
3. Select the first "i" character
4. Use `Cmd+Shift+L` (macOS) or `Ctrl+Shift+L` (Linux/Windows) to select all "i" occurrences
5. Type "e" to replace all selected "i" characters

## Expected Behavior
The result should be:
```
meks on peeeeks parem kue prääääks?
```

All "i" characters should be replaced with "e", maintaining the multiple sequential "e"s.

## Actual Behavior (Bug)
The result is:
```
meks on peks parem kue prääääks?
```

The sequential "i"s in "piiiiks" collapse to a single character instead of maintaining multiple replacements.

## Technical Details
The bug occurs because of incorrect anchor bias logic when handling adjacent selections. When typing with multicursors on adjacent positions, the cursors collapse instead of maintaining their independent positions.

## Status
This issue was fixed in a later commit that adjusted the anchor bias logic to prevent cursor collapse.

---

## EDGE CASE: Combined with Issue #41290 (Unicode Composition)

### Description
While both issues #46602 and #41290 seem to work fine separately in recent versions, there is an **edge case where they fail together**:

When trying to insert a **Unicode sequence** (via Ctrl+Shift+U) into **multiple sequential cursor positions**, the cursors still collapse incorrectly.

### Related Issue
- Issue #41290: https://github.com/zed-industries/zed/issues/41290
- Unicode multicursor insertion bug

### Edge Case Reproduction Steps

**IMPORTANT**: This edge case is specific to **Linux** with the `Ctrl+Shift+U` Unicode input method. The bug manifests in how Zed handles Unicode composition on Linux.

#### On Linux (Ubuntu/Wayland):
1. Open `test-combined-edge-case.txt` in Zed
2. In Test Case 1, select the first "a" in "aaaaa"
3. Use `Ctrl+Shift+L` to select all "a" occurrences in that line
4. You should now have 5 sequential cursors
5. Press `Ctrl+Shift+U` to enter Unicode mode
6. Type a Unicode codepoint: `00ab` (for «)
7. Press Enter to insert the character

#### On macOS:

**IMPORTANT**: This bug is **Linux-specific** and relates to the `Ctrl+Shift+U` Unicode composition method that doesn't exist on macOS. You cannot fully reproduce this exact bug on macOS.

**Simple test to verify multicursor behavior with Unicode (not the exact bug):**
1. Open `test-combined-edge-case.txt` in Zed
2. Select the first "a" in "aaaaa"
3. Press `Cmd+Shift+L` to select all 5 "a" characters (creating 5 sequential cursors)
4. Copy this character: `«`
5. Paste it
6. Verify if all 5 positions get replaced with `«` or if cursors collapse

**To properly test this bug, you need a Linux environment** (Ubuntu, Fedora, etc.) where:
- `Ctrl+Shift+U` triggers Unicode composition mode
- You can type hex codes like `00ab` and press Enter
- The bug occurs during this composition sequence with sequential multicursors

**Why macOS can't reproduce this:**
- macOS doesn't have the `Ctrl+Shift+U` composition mechanism
- The bug is specific to how Linux's input method framework (IBus/fcitx) handles composition events
- It's an interaction between Linux's Unicode input method and Zed's multicursor implementation

### Expected Behavior (Edge Case)
All 5 sequential "a" characters should be replaced with «:
```
«««««
```

### Actual Behavior (Edge Case Bug)
The sequential cursors collapse, resulting in inconsistent behavior:
- May insert only one or two « characters
- May insert literal "00ab" text at some positions
- Cursors positions may shift unexpectedly
- Some characters may be deleted

### Why This Edge Case Exists
This is a **composition problem** between two issues:
1. **Issue #46602**: Sequential multicursor collapse (fixed for regular typing)
2. **Issue #41290**: Unicode insertion with multicursors (fixed for non-sequential cursors)

The fix for regular typing doesn't account for Unicode composition sequences, which have a different insertion mechanism in the editor.

### Test Unicode Characters
- `00ab` = « (left double angle quote)
- `00bb` = » (right double angle quote)
- `2022` = • (bullet)
- `2014` = — (em dash)
- `03B1` = α (Greek alpha)

### Testing the Edge Case
1. Test with regular text replacement (should work)
2. Test with Unicode insertion on non-sequential multicursors (should work)
3. Test with Unicode insertion on **sequential** multicursors (may still fail)

---

## Testing
To test this issue:
1. Open the test files in different versions of Zed
2. Follow the reproduction steps above
3. Test both the original case and the Unicode edge case
4. Verify whether the bugs still exist in your version
