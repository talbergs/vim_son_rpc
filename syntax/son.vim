syntax match sonField "^.\{-}:" contained
syntax match sonNumber "\v<\d+>" contained
syntax match sonNumber "\v<\d+\.\d+>" contained
syntax match sonHeader "\%1l.\{-}$" contained
syntax match sonComment "^\s\{-}#.\{-}$" contained contains=ALLBUT,sonNumber,sonField

syntax region sonInput start="\%1l" end="^---$" fold transparent contains=son.*

highlight default link sonHeader Title
highlight default link sonField MoreMsg
highlight default link sonNumber Number
highlight default link sonComment Comment
