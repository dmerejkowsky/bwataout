if !exists('loaded_snippet') || &cp
    finish
endif

Snippet s1 \section{<{}>}<CR>
Snippet s2 \subsection{<{}>}<CR>
Snippet s3 \subsubsection{<{}>}<CR>

Snippet it \begin{itemize}<CR>\item <{}><CR>\end{itemize}<CR>
Snippet env  \begin{<{env}>}<CR><{}><CR>\end{<{env}>}<CR>

Snippet fr \begin{frame}[fragile]<CR>\frametitle{<{}>}<CR><{}><CR>\end{frame}<CR>
Snippet fig \begin{figure}[htb]<CR>\centering<CR>\includegraphics[height=.7\textheight]{<{}>}<CR>\end{figure}
Snippet col \begin{columns}[t]<CR>\column{.5\textwidth}<CR>\column{.5\textwidth}<CR>\end{columns}
Snippet warn \begin{alertblock}{Warning}<CR><{}><CR>\end{alertblock}
Snippet note \begin{alertblock}{Note}<CR><{}><CR>\end{alertblock}
Snippet pycode \begin{minted}{python}<CR><{}><CR>\end{minted}
