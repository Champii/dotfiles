vim.cmd [[highlight IndentBlanklineIndent1 guifg=#b13740 gui=nocombine]]
vim.cmd [[highlight IndentBlanklineIndent2 guifg=#95702c gui=nocombine]]
vim.cmd [[highlight IndentBlanklineIndent3 guifg=#649f39 gui=nocombine]]
vim.cmd [[highlight IndentBlanklineIndent4 guifg=#56B6C2 gui=nocombine]]
vim.cmd [[highlight IndentBlanklineIndent5 guifg=#61AFEF gui=nocombine]]
vim.cmd [[highlight IndentBlanklineIndent6 guifg=#C678DD gui=nocombine]]

vim.opt.list = true
vim.opt.listchars:append("space:⋅")

require("indent_blankline").setup {
    -- for example, context is off by default, use this to turn it on
    --[[ show_current_context = true,
    show_current_context_start = true, ]]
    space_char_blankline = " ",
    char_highlight_list = {
        "IndentBlanklineIndent1",
        "IndentBlanklineIndent2",
        "IndentBlanklineIndent3",
        "IndentBlanklineIndent4",
        "IndentBlanklineIndent5",
        "IndentBlanklineIndent6",
    },
    filetype_exclude = { "dashboard" },
}
