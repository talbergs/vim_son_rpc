fun! son_rpc#layout#as_list()
    return [s:son_params, s:son_resp, s:son_detail]
endfun

fun! son_rpc#layout#view1()
    sp son_resp
    sp son_detail
endfun

fun! son_rpc#layout#ensure()
    if line2byte('$') == -1
        e son_params
    else
        badd son_params
    end

    badd son_resp
    badd son_detail

    let s:son_params = bufnr('son_params')
    call nvim_buf_set_option(s:son_params, 'filetype', 'son')
    call nvim_buf_set_option(s:son_params, 'buftype', 'nofile')
    call nvim_buf_set_option(s:son_params, 'omnifunc', 'son_rpc#complete#omni')

    let s:son_resp = bufnr('son_resp')
    call nvim_buf_set_option(s:son_resp, 'filetype', 'json')
    call nvim_buf_set_option(s:son_resp, 'buftype', 'nofile')

    let s:son_detail = bufnr('son_detail')
    call nvim_buf_set_option(s:son_detail, 'filetype', 'yaml')
    call nvim_buf_set_option(s:son_detail, 'buftype', 'nofile')

endfun


