local actions = require "fzf-lua.actions"

require('fzf-lua').setup({
    winopts = {
        border = false,
        height = 0.3,
        width = 0.8,
        row = 0.8,
        col = 0.5,
        preview = {
            default = 'bat',
            vertical = "down:80%",
            horizontal = "right:40%",
        },
        on_create = function()
        end,
    },
})
