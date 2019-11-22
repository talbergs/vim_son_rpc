if !exists('s:logfile')
  let s:logfile = '/tmp/son-rpc-stderr'
endif

fun! son_rpc#util#log_init()
    call system('touch '.s:logfile)
endfun

fun! son_rpc#util#open_log()
    tabnew s:logfile
endfun

fun! son_rpc#util#log(message)
  new
  setlocal buftype=nofile bufhidden=hide noswapfile nobuflisted
  put=a:message
  silent execute 'w >>' s:logfile
  q
endfun


