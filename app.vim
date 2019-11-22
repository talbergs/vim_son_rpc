" Extend the app with optional dependencies:
" set rtp+=path_to_airline_plugin
" packadd lalala

set rtp+=$PWD

source $PWD/plugin/son_rpc.vim

silent call son_rpc#start_service()
silent call son_rpc#set_session($URL, $USERNAME, $PASSWORD)
silent call son_rpc#layout#view1()

nnoremap <c-s> :call son_rpc#submit()<cr>
inoremap <c-s> <esc>:call son_rpc#submit()<cr>a

" An idea to keep al buffers unlisted and allow to each rotate
" inoremap <c-n> :call son_rpc#buf_rotate_next()<cr>
" inoremap <c-p> :call son_rpc#buf_rotate_prev()<cr>

" au UserGettingBored echomsg 'use <c-s> to send request'

" Usage: nvim -u app.vim --noplugin
