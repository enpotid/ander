$$
\begin{align}
[\text{Prog}] &\to [\text{Stmt}]^* \\
[\text{Stmt}] &\to
\begin{cases}
\text{exit([\text{Expr}]);}\\
\text{let \space\text{ident} = [\text{Expr}];}
\end{cases} \\
[\text{Expr}] &\to
\begin{cases}
[\text{Term}] \\
[\text{BinExpr}]\\
\end{cases} \\
[\text{BinExpr}] &\to
\begin{cases}
[\text{Expr}] + [\text{Expr}] \space prec = 0 \\
[\text{Expr}] - [\text{Expr}] \space prec = 0 \\
[\text{Expr}] \space * \space [\text{Expr}] \space prec = 1 \\
[\text{Expr}] \space \space / \space \space [\text{Expr}] \space prec = 1 \\
\end{cases} \\
[\text{Term}] &\to
\begin{cases}
\text{int\_lit} \\
\text{ident} \\
\end{cases}
\end{align}
$$
