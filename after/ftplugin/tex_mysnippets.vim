if !exists('loaded_snippet') || &cp
    finish
endif

Snippet s1 \section{<{}>}<CR><{}>
Snippet s2 \subsection{<{}>}<CR><{}>
Snippet s3 \subsubsection{<{}>}<CR><{}>

Snippet bi \begin{itemize}<CR><TAB>\item <{}><CR><BS>\end{itemize}<CR><{}>
Snippet be  \begin{<{env}>}<CR><TAB><{}><CR><BS>\end{<{env}>}<CR><{}>

Snippet fr \begin{frame}[fragile]<CR><TAB>\frametitle{<{}>}<CR><{}><CR><BS>\end{frame}<CR><{}>
Snippet lst \begin{lstlisting}<CR><CR><BS><{}><CR>\end{lstlisting}<CR><{}>
