# GitHub Math Rendering Demo

This file tests some practical ways to show the matrix [ [1  2], [3  4] ] in GitHub Markdown.

---

## 1. Math Fencing

```math
\begin{bmatrix}
1 & 2 \\
3 & 4
\end{bmatrix}
```

---

## 2. Latex
### Inline
$\begin{bmatrix}1 & 2 \\ 3 & 4\end{bmatrix}$

### Block
$$
\begin{bmatrix}1 & 2 \\ 3 & 4\end{bmatrix}
$$


## 4. Unicode pseudo‑matrix
```
⎡1  2⎤
⎣3  4⎦
```

Inline version (compact):  
`⟦1 2; 3 4⟧`



## 5. Code block (preserved LaTeX)
```latex
\begin{bmatrix}1 & 2 \\ 3 & 4\end{bmatrix}
```

