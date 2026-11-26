+++
title = "Hello, notes"
date = 2026-11-26
draft = true  # sample: hidden from the published site (see it with `zola serve --drafts`)
description = "A sample note showing what notes can do: Markdown, math, code and figures. Edit or delete this one."
[taxonomies]
tags = ["meta"]
[extra]
math = true
+++

This is a **sample note**. Notes are plain Markdown files in `content/notes/`.
Create one with `site new note "My title"` (see the README), write, and push.

## Math

Inline math like $x_\mathrm{HI}$ works. For display equations use the `math` block, which keeps
LaTeX safe from Markdown. The Gunn–Peterson optical depth:

{% <math> %}
\tau_\mathrm{GP} \simeq 1.8 \times 10^{5}\, h^{-1}\, \Omega_m^{-1/2}
\left(\frac{\Omega_b h^2}{0.02}\right)
\left(\frac{1+z}{7}\right)^{3/2} x_\mathrm{HI}
{% </math> %}

## Code

```python
import numpy as np

z = np.linspace(5, 13, 100)
print(f"{z.min():.1f} < z < {z.max():.1f}")
```

## Figures

Put an image next to `index.md` in the note's folder and link it: `![caption](figure.png)`.

> Short, informal, and allowed to be unfinished — that's the point of a note.
