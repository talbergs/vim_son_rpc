function! son_rpc#statusline#_(...)
  if exists('g:son_state')
    return join([
                \bufname('%'),
                \'| process:',
                \son_rpc#jobid() ? 'Online' : 'Offline',
                \'| session:',
                \g:son_state.connected ? 'Online' : 'Offline',
                \'| user:',
                \g:son_state.username,
                \'| url:',
\bufname('%'),
                \g:son_state.url])
  endif
  return 'Welcome to son buf. Connect using son_rpc#open(url, un, pw)'
endfunction

