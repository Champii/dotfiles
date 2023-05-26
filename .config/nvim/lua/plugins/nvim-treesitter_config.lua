--require 'nvim-treesitter.install'.compilers = { "clang++" }
--require('orgmode').setup_ts_grammar()

local parser_config = require "nvim-treesitter.parsers".get_parser_configs()
parser_config.rock = {
    install_info = {
        url = "~/prog/rust/treesitter/tree-sitter-rock/", -- local path or git repo
        files = { "src/parser.c", "src/scanner.cc" },
        -- optional entries:
        -- branch = "main", -- default branch in case of git repo if different from master
        -- generate_requires_npm = false, -- if stand-alone parser without npm dependencies
        -- requires_generate_from_grammar = false, -- if folder contains pre-generated src/parser.c
    },
    filetype = "rk", -- if filetype does not match the parser name
}
-- local ft_to_parser = require "nvim-treesitter.parsers".filetype_to_parsername
local ft_to_parser = vim.treesitter.language.register
ft_to_parser('rk', 'rock')

require("nvim-treesitter.configs").setup {
    incremental_selection = {
        enable = true,
        keymaps = {
            init_selection = "gnn",
            node_incremental = "grn",
            scope_incremental = "grc",
            node_decremental = "grm",
        },
    },
    indent = { enable = true },
    highlight = {
        enable = true,
        disable = { "nix", "org" },
        additional_vim_regex_highlighting = { 'org' }, -- Required since TS highlighter doesn't support all syntax features (conceal)
    },
    rainbow = {
        enable = true,
        extended_mode = true, -- Also highlight non-bracket delimiters like html tags, boolean or table: lang -> boolean
        max_file_lines = nil, -- Do not enable for files with more than n lines, int
        -- colors = {}, -- table of hex strings
        -- termcolors = {} -- table of colour name strings
    },
    --[[ rainbow_ident = {
        enable = true,
    }, ]]
    textobjects = {
        select = {
            enable = true,
            lookahead = true,
            keymaps = {
                -- You can use the capture groups defined in textobjects.scm
                ['af'] = '@function.outer',
                ['if'] = '@function.inner',
                ['ac'] = '@class.outer',
                ['ic'] = '@class.inner',
            },
        },
        move = {
            enable = true,
            set_jumps = true, -- whether to set jumps in the jumplist
            goto_next_start = { [']f'] = '@function.outer', [']]'] = '@class.outer' },
            goto_next_end = { [']F'] = '@function.outer', [']['] = '@class.outer' },
            goto_previous_start = { ['[f'] = '@function.outer', ['[['] = '@class.outer' },
            goto_previous_end = { ['[F'] = '@function.outer', ['[]'] = '@class.outer' },
        },
        swap = {
            enable = true,
            swap_next = {
                ['<leader>>'] = '@parameter.inner',
                ['<m-J>'] = '@statement.outer',
            },
            swap_previous = {
                ['<leader><'] = '@parameter.inner',
                ['<m-K>']     = '@statement.outer',
            },
        },
        lsp_interop = {
            enable = true,
            peek_definition_code = {
                ['gD'] = '@function.outer',
            },
        },
    },
    textsubjects = {
        enable = true,
        keymaps = {
            ['g.'] = 'textsubjects-smart',
            ['g;'] = 'textsubjects-container-outer',
        },
    },
    refactor = {
        highlight_definitions = {
            enable = true,
        },
        navigation = {
            enable = true,
        },
        smart_rename = {
            enable = true,
            keymaps = {
                smart_rename = '<leader>rn',
            },
        },
    },
    autopairs = { enable = true },
    autotag = { enable = true, disable = { 'markdown' } },
    context_commentstring = { enable = true, enable_autocmd = false },
}
