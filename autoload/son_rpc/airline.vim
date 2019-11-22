function! son_rpc#airline#_a(...)
    return b:son_buf.connected ? 'Online' : 'Offline'
endfunction

function! son_rpc#airline#_(...)
  if exists('b:son_buf')
      let w:airline_section_a = '%{son_rpc#airline#_a()}'
  endif
endfunction
