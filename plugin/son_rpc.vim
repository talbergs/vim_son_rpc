if !exists('s:jobid')
    let s:jobid = 0
endif

let s:scriptdir = resolve(expand('<sfile>:p:h') . '/..')
let $RUST_BACKTRACE = 1
let s:bin = s:scriptdir . '/target/debug/rpc'
" let s:bin = s:scriptdir . '/target/release/rpc'

function! s:disconnect()
    call s:rpcnotify('Exit')
    let s:jobid = 0
endfunction

function! s:connect()
    if s:jobid > 0
        call s:disconnect()
    else
        let s:jobid = jobstart([s:bin], {
                    \'rpc': v:true,
                    \'on_stderr': function('s:on_stderr'),
                    \'on_exit': function('s:on_exit'),
                    \})
    endif
endfunction

fun! son_rpc#stop_service()
    call s:disconnect()
endfun

fun! son_rpc#start_service()
    call s:connect()
    if s:jobid > 0
        call son_rpc#util#log_init()
        call s:autocmd()
        call son_rpc#layout#ensure()
    else
        echoerr s:jobid == -1
                    \? 'Cannot start RPC_client process.'
                    \: 'RPC_client process is not executable.'
    endif
endfun

fun! son_rpc#submit()
    if s:jobid > 0
        call call(function('s:rpcnotify'), ['Submit'] + son_rpc#layout#as_list())
    endif
endfun

fun! son_rpc#set_session(url, un, pw)
    if s:jobid > 0
        call s:rpcnotify('SetSession', a:url, a:un, a:pw)
    endif
endfun

fun! son_rpc#jobid()
    return s:jobid > 0
endfun

function! s:on_exit(id, exitcode, event) dict
    if a:exitcode == 0
        return
    endif
    let s:jobid = 0
    call son_rpc#util#log(a:event.' code '.a:exitcode)
    echoerr 'Job exited unexpectedly. To see error, :call son_rpc#util#open_log()'
endfunction

function! s:on_stderr(id, data, event) dict
    call son_rpc#util#log(join(a:data, "\n"))
endfunction

function! s:autocmd()
    augroup son_rpc
        autocmd!
        autocmd VimLeavePre * :call s:disconnect()
    augroup END
endfunction

fun! CompleteMonths(findstart, base)
    if a:findstart
        " locate the start of the word
        let line = getline('.')
        let start = col('.') - 1
        while start > 0 && line[start - 1] =~ '\a'
            let start -= 1
        endwhile
        return start
    else
        " find months matching with "a:base"
        for m in split("Jan Feb Mar Apr May Jun Jul Aug Sep Oct Nov Dec")
            if m =~ '^' . a:base
                call complete_add(m)
            endif
            sleep 300m	" simulate searching for next match
            if complete_check()
                break
            endif
        endfor
        return []
    endif
endfun

fun! son_rpc#rpcnotify(...)
    if s:jobid > 0
        call call(function('rpcnotify'), [s:jobid, a:1] + a:000[1:])
    end
endfun

function! s:rpcnotify(...)
    if s:jobid > 0
        call call(function('rpcnotify'), [s:jobid, a:1] + a:000[1:])
    end
endfunction

if exists('airline#add_statusline_func')
    call airline#add_statusline_func('son_rpc#airline#_')
else
    set statusline=%{son_rpc#statusline#_()}
endif

fun! son_rpc#completefunc(findstart, base)
    " call s:rpcnotify('CompleteMethod')
    if a:findstart
        " locate the start of the word
        let line = getline('.')
        let start = col('.') - 1
        while start > 0 && line[start - 1] =~ '\a'
            let start -= 1
        endwhile
        return start
    else
        " find months matching with "a:base"
        let res = []
        for m in split("Jan Feb Mar Apr May Jun Jul Aug Sep Oct Nov Dec")
            if m =~ '^' . a:base
                call add(res, m)
            endif
        endfor
        return res
    endif
endfun

