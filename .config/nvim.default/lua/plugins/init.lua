require('plugins.nvim-treesitter_config')
require('plugins.gitsigns_config')
require('plugins.which-key_config')
require('plugins.lsp-format_config')
require('plugins.lsp_signature_config')
require('plugins.nvim-lsp-installer_config')
-- require('plugins.copilot-lua_config')
require('plugins.luasnip_config')
require('plugins.cmp_config')
require('plugins.fzf-lua_config')
require('plugins.persistence_config')
require('plugins.nvim-autopairs_config')
require('plugins.fidget_config')
require('plugins.stabilize_config')
require('plugins.nvim-treesitter-context_config')
require('plugins.toggleterm_config')
require('plugins.neotree_config')
require('plugins.neogit_config')
require('plugins.hlargs_config')
-- require('plugins.orgmode_config')
-- require('plugins.dap-install_config')
-- require('plugins.dap-ui_config')
require('plugins.indent-blankline-nvim_config')
require('plugins.chat-gpt_config')
require('plugins.code_fixer_test')

require('plugins.statusline_config')

-- require('plugins.rainbow_ident_test')

--[[ local queries = require("nvim-treesitter.query")
require("nvim-treesitter").define_modules({
    rainbow_ident = {
        module_path = "plugins.rainbow_ident_test",
        is_supported = function(lang)
            return queries.get_query(lang, "parens") ~= nil
        end,
        extended_mode = true,
        colors = {
            "#cc241d",
            "#a89984",
            "#b16286",
            "#d79921",
            "#689d6a",
            "#d65d0e",
            "#458588",
        },
        termcolors = {
            "Red",
            "Green",
            "Yellow",
            "Blue",
            "Magenta",
            "Cyan",
            "White",
        },
    },
}) ]]
