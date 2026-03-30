# Thunar context-menu bug: zip-zero (and catch-all) broken when directories selected

When the Thunar context menu fires with "directories" as the action context (rather than files),
custom actions that expect file paths — including "send to catch-all" and "zip-zero" — don't
receive the file list. The action runs but gets nothing.

Possible relevance to Future: context menus on mixed selections (files + dirs) are a solved UX
problem we should handle gracefully. Don't inherit this bug.
