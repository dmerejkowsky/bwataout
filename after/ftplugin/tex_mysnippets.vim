if !exists('loaded_snippet') || &cp
    finish
endif

Snippet s1 \section{<{}>}<CR>
Snippet s2 \subsection{<{}>}<CR>
Snippet s3 \subsubsection{<{}>}<CR>

Snippet it \begin{itemize}<CR><TAB>\item <{}><CR><BS>\end{itemize}<CR>
Snippet env  \begin{<{env}>}<CR><TAB><{}><CR><BS>\end{<{env}>}<CR>

Snippet fr \begin{frame}[fragile]<CR><TAB>\frametitle{<{}>}<CR><{}><CR><BS>\end{frame}<CR>
Snippet lst \begin{lstlisting}<CR><CR><BS><{}><CR>\end{lstlisting}<CR>
Snippet fig \begin{figure}[htb]<CR><TAB>\centering<CR><TAB>\includegraphics[height=.7\textheight]{<{}>}<CR><BS><BS>\end{figure}
Snippet col \begin{columns}[t]<CR><TAB>\column{.5\textwidth}<CR>\column{.5\textwidth}<CR><BS><BS>\end{columns}
Snippet warn \begin{alertblock}{Warning}<CR><{}><CR>\end{alertblock}
Snippet note \begin{alertblock}{Note}<CR><{}><CR>\end{alertblock}
