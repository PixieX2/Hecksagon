import sys
from collections import deque


TARGET = "HELLO WORLD"

# Command mapping: (delta_cell, flip_nose, delta_shadow, delta_ghost)
mapping = {
    '!': (3, 0, 1, 0),
    '@': (-7, 1, 2, 1),
    '#': (5, 0, -1, 3),
    '~': (0, 1, 0, 0),
    '%': (1, 0, 3, -2),
    '^': (-3, 0, 0, 5),
    '&': (2, 1, -2, 0),
    '*': (4, 0, 1, 1),
    '(': (-2, 1, 0, -1),
    ')': (0, 0, 4, 0),
    '_': (0, 0, 0, 1),
    '+': (7, 0, -3, 2),
}



OPS = list(mapping.keys())

def step(state, cmd):
    """Apply a single command to the state."""
    cell, nose, shadow, ghost = state
    dC, flip, dS, dG = mapping[cmd]
    cell = (cell + dC) % 256
    shadow = (shadow + dS) % 256
    ghost = (ghost + dG) % 256
    if flip:
        nose ^= 1
    return (cell, nose, shadow, ghost)

state = (0, 0, 0, 0)
program = ""

for i, ch in enumerate(TARGET, 1):
    goal = ord(ch)

    # BFS search for shortest command sequence producing goal
    q = deque()
    q.append((state, ""))

    seen = set([state])
    found = None

    while q:
        cur, seq = q.popleft()
        cell, nose, shadow, ghost = cur

        # Check if output matches target character
        if (cell + shadow + ghost) % 256 == goal:
            found = seq
            state = cur
            break

        # Explore all operations
        for op in OPS:
            nxt = step(cur, op)
            if nxt not in seen:
                seen.add(nxt)
                q.append((nxt, seq + op))

    if found is None:
        raise RuntimeError(f"Cannot reach target '{ch}' 💀")

    # Add found command sequence + output command
    program += found + "$"

    # Progress feedback
    sys.stdout.write(f"\r[{i}/{len(TARGET)}] chars generated")
    sys.stdout.flush()

print("\nGenerated hw.h2")

with open("hw.h2", "w") as f:
    f.write(program)


text = """
******************************************************
*                                                    *
*   DONE! BIG SHOUTOUT TO THE HOT POTATOES           *
*   FOR MAKING THIS POSSIBLE !!!                     *
*                                                    *
******************************************************
"""

print(text)