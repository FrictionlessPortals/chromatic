" Test Scheme -- A Vim color scheme.
" Name: Test Scheme
" Author: Test Author
" Description: Test Scheme
" License: MIT

hi clear

if exists("syntax_on")
  syntax reset
endif

let g:colors_name = "Test Scheme"

if ($TERM =~ '256' || &t_Co >= 256) || has("gui_running")
    hi LineNr ctermbg=#ffffff guibg=#ffffff ctermfg=#808080 guifg=#808080 cterm=none gui=none guisp=none 
    hi SpellBad ctermbg=#ffffff guibg=#ffffff ctermfg=#808080 guifg=#808080 cterm=undercurl gui=undercurl guisp=#ff0000 
    hi Cursor ctermbg=#ffffff guibg=#ffffff ctermfg=#808080 guifg=#808080 cterm=none gui=none guisp=none 
    hi CursorLine ctermbg=#ffffff guibg=#ffffff ctermfg=#808080 guifg=#808080 cterm=none gui=none guisp=none 
    hi CursorLineNr ctermbg=#ffffff guibg=#ffffff ctermfg=#808080 guifg=#808080 cterm=none gui=none guisp=none 
    hi Normal ctermbg=#ffffff guibg=#ffffff ctermfg=#808080 guifg=#808080 cterm=none gui=none guisp=none set background=light
elseif &t_Co == 8 || $TERM !~# '^linux' || &t_Co == 16
    set t_Co=16

    hi LineNr ctermbg=#ffffff ctermfg=#808080 cterm=none 
    hi SpellBad ctermbg=#ffffff ctermfg=#808080 cterm=undercurl 
    hi Cursor ctermbg=#ffffff ctermfg=#808080 cterm=none 
    hi CursorLine ctermbg=#ffffff ctermfg=#808080 cterm=none 
    hi CursorLineNr ctermbg=#ffffff ctermfg=#808080 cterm=none 
    hi Normal ctermbg=#ffffff ctermfg=#808080 cterm=none set background=light
endif

" Generated with chromatic (https://github.com/FrictionlessPortals/chromatic).