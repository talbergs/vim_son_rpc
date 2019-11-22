" really bad omnicompletion here, by sending notid and waiting 20ms for global var to be written. awful
" else rpcrequest and sockconnect should be implemented, but there is no time for that right now
fun! son_rpc#complete#_param(findstart, base)
    let line = getline('.')
    if a:findstart
        " TODO findstart first char in line
        return 0
    else
        call son_rpc#rpcnotify('CompleteParam', a:base)
        sleep 20m
        return exists('g:son_param_completions') ? g:son_param_completions : []
    endif
endfun

fun! son_rpc#complete#_method(findstart, base)
    let line = getline('.')
    if a:findstart
        return 0
    else
        call son_rpc#rpcnotify('CompleteMethod', a:base)
        sleep 20m
        return exists('g:son_method_completions') ? g:son_method_completions : []
    endif
endfun

fun! son_rpc#complete#omni(findstart, base)
    return line('.') == 1
                \? son_rpc#complete#_method(a:findstart, a:base)
                \: son_rpc#complete#_param(a:findstart, a:base)
endfun
