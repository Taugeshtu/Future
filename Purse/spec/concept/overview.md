# Purse — Overview

Purse is a floating, workspace-local context accumulator.

It holds a set of files at **preview fidelity**: enough to identify and assess each one, not enough to act on them. It is the *noun half* of a noun/verb interaction model — you assemble context in Purse, then hand it off to something that acts.

## Fidelity ladder

Files can be examined at multiple levels. Purse occupies the middle:

```
filename only
filename + icon / thumbnail
→ Purse: enough to assess
full viewer / editor
```

## What Purse is not

- Not a file manager. It does not navigate.
- Not a full viewer. It shows enough to decide, not enough to do.
- Not a launcher. It holds nouns; something else handles verbs.
- Not persistent across sessions. Purse is transient working memory.
