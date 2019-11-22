let SessionLoad = 1
let s:so_save = &so | let s:siso_save = &siso | set so=0 siso=0
let v:this_session=expand("<sfile>:p")
silent only
cd ~/Dropbox/vim_son_rpc/src
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
set shortmess=aoO
badd +1 ~/Dropbox/vim-php-rpc/ftdetect/son.vim
badd +2 ~/Dropbox/vim-php-rpc/syntax/son.vim
badd +113 ~/Dropbox/vim-php-rpc/plugin/vim-plugin.vim
badd +1 ~/Dropbox/vim-php-rpc/doc/tags
badd +1 ~/Dropbox/vim-php-rpc/doc/vim-plugin.txt
badd +1 ~/Dropbox/vim-php-rpc/indent/son.vim
badd +53 ~/Dropbox/vim-php-rpc/src/core/eventhandler.rs
badd +1 ~/Dropbox/vim-php-rpc/autoload/vim-plugin.vim
badd +1 ~/Dropbox/vim_son_rpc/ftplugin/son.vim
badd +1 ~/Dropbox/vim_son_rpc/ftdetect/son.vim
badd +50 ~/Dropbox/vim_son_rpc/plugin/son_rpc.vim
badd +4 ~/Dropbox/vim_son_rpc/syntax/son.vim
badd +1 ~/Dropbox/vim_son_rpc/autoload/son_rpc/util.vim
badd +158 core/eventhandler.rs
badd +1 core/mod.rs
badd +13 ~/Dropbox/vim_son_rpc/Cargo.toml
badd +1 main.rs
badd +537 ~/.cargo/registry/src/github.com-1ecc6299db9ec823/neovim-lib-0.6.1/src/neovim_api.rs
badd +21 ~/Dropbox/vim_son_rpc/autoload/son_rpc/layout.vim
badd +1 core/client.rs
badd +30 core/req.rs
badd +2 ~/Dropbox/vim_son_rpc/autoload/son_rpc/airline.vim
badd +17 ~/Dropbox/vim_son_rpc/app.vim
badd +1 ~/Dropbox/vim_son_rpc/app.sh
badd +12 ~/Dropbox/vim_son_rpc/autoload/son_rpc/statusline.vim
badd +3 core/state.rs
badd +12 ~/Dropbox/vim_son_rpc/autoload/son_rpc/complete.vim
badd +91 core/resp.rs
badd +29 ~/.cargo/registry/src/github.com-1ecc6299db9ec823/http-0.1.18/src/version.rs
badd +1 ~/Dropbox/vim_son_rpc/readme.md
badd +1 ~/Dropbox/vim_son_rpc/Session.vim
argglobal
silent! argdel *
edit ~/Dropbox/vim_son_rpc/syntax/son.vim
set splitbelow splitright
set nosplitright
wincmd t
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
argglobal
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=99
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let s:l = 9 - ((8 * winheight(0) + 14) / 28)
if s:l < 1 | let s:l = 1 | endif
exe s:l
normal! zt
9
normal! 0
tabnext 1
if exists('s:wipebuf') && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20 winminheight=1 winminwidth=1 shortmess=filnxtToOFc
let s:sx = expand("<sfile>:p:r")."x.vim"
if file_readable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &so = s:so_save | let &siso = s:siso_save
let g:this_session = v:this_session
let g:this_obsession = v:this_session
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
