$$
\begin{align}
[\text{prog}] &\to [\text{stmt}]^* \\
[\text{stmt}] &\to 
\begin{cases}
\text{exit([\text{expr}]);}\\
\text{let \space\text{index} = [\text{expr}];}
\end{cases} \\
[\text{expr}] &\to
\begin{cases} 
\text{int\_lit} \\
\text{index}
\end{cases}
\end{align}
$$
