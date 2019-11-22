" :help ftplugin
" After our ftdetect file sets the file type as son Vim will source the this
" file.

setlocal commentstring=#\ %s
setlocal tabstop=2
setlocal softtabstop=2
setlocal shiftwidth=2
" TODO
setlocal completefunc=son_rpc#completefunc
setlocal omnifunc=son_rpc#omnifunc
