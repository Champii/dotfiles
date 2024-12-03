-- Only required if you have packer configured as `opt`
-- vim.cmd [[packadd packer.nvim]]

-- Lazy init
local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"
if not vim.loop.fs_stat(lazypath) then
    vim.fn.system({
        "git",
        "clone",
        "--filter=blob:none",
        "https://github.com/folke/lazy.nvim.git",
        "--branch=stable", -- latest stable release
        lazypath,
    })
end
vim.opt.rtp:prepend(lazypath)

local plugins = {
    'wbthomason/packer.nvim',
    -- 'glepnir/dashboard-nvim',
    'j-hui/fidget.nvim',
    'ibhagwan/fzf-lua',
    'junegunn/fzf.vim',
    { 'glepnir/galaxyline.nvim',             branch = 'main' },
    'lewis6991/gitsigns.nvim',
    'b3nj5m1n/kommentary',
    'ggandor/lightspeed.nvim',
    'lukas-reineke/lsp-format.nvim',
    'ray-x/lsp_signature.nvim',
    'L3MON4D3/LuaSnip',
    'nvim-neo-tree/neo-tree.nvim',
    'TimUntersberger/neogit',
    'MunifTanjim/nui.nvim',
    'windwp/nvim-autopairs',
    -- 'williamboman/nvim-lsp-installer',
    'nvim-treesitter/nvim-treesitter',
    'nvim-treesitter/nvim-treesitter-context',
    -- 'nvim-treesitter/nvim-treesitter-refactor',
    'nvim-treesitter/nvim-treesitter-textobjects',
    -- 'p00f/nvim-ts-rainbow',
    'hiphish/rainbow-delimiters.nvim',
    'nvim-tree/nvim-web-devicons',
    'folke/persistence.nvim',
    'nvim-lua/popup.nvim',
    'ahmedkhalf/project.nvim',
    'luukvbaal/stabilize.nvim',
    'lambdalisue/suda.vim',
    'akinsho/toggleterm.nvim',
    'mbbill/undotree',
    -- 'glepnir/vim-nix',
    'folke/which-key.nvim',
    'hrsh7th/nvim-cmp',
    -- # 'glepnir/copilot-lua',
    -- 'github/copilot.vim',
    'hrsh7th/cmp-buffer',
    'hrsh7th/cmp-nvim-lsp',
    'hrsh7th/cmp-nvim-lua',
    'saadparwaiz1/cmp_luasnip',
    'hrsh7th/cmp-path',
    'ray-x/cmp-treesitter',
    -- # 'glepnir/copilot-cmp',
    'm-demare/hlargs.nvim',
    'famiu/bufdelete.nvim',
    -- 'lukas-reineke/indent-blankline.nvim',
    { "lukas-reineke/indent-blankline.nvim", main = "ibl",   opts = {} },
    'stevearc/dressing.nvim',
    -- 'glepnir/vim-gluon',
    -- 'glepnir/orgmode',
    -- 'glepnir/nvim-dap',
    -- 'glepnir/nvim-dap-ui',
    -- 'glepnir/nvim-dap-virtual-text',
    -- 'glepnir/dap-buddy',
    -- 'glepnir/firenvim',
    'jackMort/ChatGPT.nvim',
    'nvim-lua/plenary.nvim',
    'nvim-telescope/telescope.nvim',
    {
        "folke/trouble.nvim",
        dependencies = "nvim-tree/nvim-web-devicons",
        config = function()
            require("trouble").setup {
                -- your configuration comes here
                -- or leave it empty to use the default settings
                -- refer to the configuration section below
            }
        end
    },
    {
        'rmagatti/goto-preview',
        config = function()
            require('goto-preview').setup {}
        end
    },
    {
        "rcarriga/nvim-notify",
        config = function() require("notify").setup {} end
    },
    "jose-elias-alvarez/null-ls.nvim",
    {
        'lvimuser/lsp-inlayhints.nvim',
        branch = 'anticonceal',
        config = function()
            require("lsp-inlayhints").setup {}
        end
    },
    {
        "williamboman/mason.nvim",
        config = function() require('mason').setup {} end
    },
    {
        "williamboman/mason-lspconfig.nvim",
        config = function()
            require('mason-lspconfig').setup {}
            require('mason-lspconfig').setup_handlers {
                function(server_name) -- default handler (optional)
                    require("lspconfig")[server_name].setup {
                        on_attach = require("lsp-format").on_attach,
                    }
                end,
                ["rust_analyzer"] = function() -- rust-analyzer handler
                    require("lspconfig").rust_analyzer.setup {
                        on_attach = function(c, b)
                            require "lsp-format".on_attach(c, b)
                            require("lsp-inlayhints").on_attach(c, b)
                        end,
                        settings = {
                            ["rust-analyzer"] = {
                                cargo = {
                                    features = "all",
                                    extraEnv = {
                                        CARGO_TARGET_DIR = "/home/champii/.cache/rust-analyzer"
                                    },
                                },
                                procMacro = {
                                    enable = true,
                                },

                            },
                        },
                    }
                    --[[ require("lspconfig").rust_analyzer.setup {
                        on_attach = require("lsp-format").on_attach,
                        settings = {
                            ["rust-analyzer"] = {
                                inlayHints = {
                                    typeHints = {
                                        enabled = true,
                                    },
                                },
                            },
                        },
                    } ]]
                end,
            }
        end
    },
    'neovim/nvim-lspconfig',
    'simrat39/rust-tools.nvim',
    "rktjmp/highlight-current-n.nvim",
    { 'michaelb/sniprun', run = 'bash ./install.sh' },
    "nvim-treesitter/playground",
    {
        "aserowy/tmux.nvim",
        config = function() return require("tmux").setup() end
    },
    {
        's1n7ax/nvim-window-picker',
        name = 'window-picker',
        event = 'VeryLazy',
        version = '2.*',
        config = function()
            require 'window-picker'.setup({
                hint = 'floating-big-letter',
            })
        end,
    },
    {
        'saecki/crates.nvim',
        tag = 'stable',
        dependencies = { 'nvim-lua/plenary.nvim' },
        config = function()
            require('crates').setup{
                lsp = {
                    enabled = true,
                    on_attach = function(client, bufnr)
                        -- the same on_attach function as for your other lsp's
                    end,
                    actions = true,
                    completion = true,
                    hover = true,
                },
            }
        end,
    },
    {
       "amitds1997/remote-nvim.nvim",
       version = "*", -- Pin to GitHub releases
       dependencies = {
           "nvim-lua/plenary.nvim", -- For standard functions
           "MunifTanjim/nui.nvim", -- To build the plugin UI
           "nvim-telescope/telescope.nvim", -- For picking b/w different remote methods
       },
       config = true,
    },
    {
        "nosduco/remote-sshfs.nvim",
        dependencies = { "nvim-telescope/telescope.nvim" },
        opts = {
            -- Refer to the configuration section below
            -- or leave empty for defaults
        },
    },
    {
        "OXY2DEV/markview.nvim",
        lazy = false,      -- Recommended
        -- ft = "markdown" -- If you decide to lazy-load anyway

        dependencies = {
            -- You will not need this if you installed the
            -- parsers manually
            -- Or if the parsers are in your $RUNTIMEPATH
            "nvim-treesitter/nvim-treesitter",
            "nvim-tree/nvim-web-devicons"
        }
    },
    {
        "desdic/macrothis.nvim",
        opts = {},
        keys = {
            { "<Leader>mmd", function() require('macrothis').delete() end, desc = "delete" },
            { "<Leader>mme", function() require('macrothis').edit() end, desc = "edit" },
            { "<Leader>mml", function() require('macrothis').load() end, desc = "load" },
            { "<Leader>mmn", function() require('macrothis').rename() end, desc = "rename" },
            { "<Leader>mmq", function() require('macrothis').quickfix() end, desc = "run macro on all files in quickfix" },
            { "<Leader>mmr", function() require('macrothis').run() end, desc = "run macro" },
            { "<Leader>mms", function() require('macrothis').save() end, desc = "save" },
            { "<Leader>mmx", function() require('macrothis').register() end, desc = "edit register" },
            { "<Leader>mmp", function() require('macrothis').copy_register_printable() end, desc = "Copy register as printable" },
            { "<Leader>mmm", function() require('macrothis').copy_macro_printable() end, desc = "Copy macro as printable" },
        }
    },
    'gaborvecsei/memento.nvim',
    {
        'nvimdev/dashboard-nvim',
        event = 'VimEnter',
        config = function()
            require('dashboard').setup {
                -- config
                theme = 'hyper',
                config = {
                    header = {
                    },
                    footer = {
                        "",
                        "Remember to stay hydrated !",
                    },
                    week_header = {
                        enable = false,
                    },
                    shortcut = {
                        { desc = '󰊳 Update', group = '@property', action = 'Lazy update', key = 'u' },
                        {
                            icon = ' ',
                            icon_hl = '@variable',
                            desc = 'Files',
                            group = 'Label',
                            action = 'FzfLua files',
                            key = 'f',
                        },
                        --[[ {
                            desc = ' Apps',
                            group = 'DiagnosticHint',
                            action = 'Telescope app',
                            key = 'a',
                        }, ]]
                        {
                            desc = ' dotfiles',
                            group = 'Number',
                            action = 'FzfLua files cwd=~/.config/nvim/lua/',
                            key = 'd',
                        },
                    },
                    project = { enable = true, limit = 8, icon = 'your icon', label = '', action = 'FzfLua files cwd=' },
                },
               
            }
        end,
        dependencies = { {'nvim-tree/nvim-web-devicons'}}
    },
    {
        "folke/noice.nvim",
        event = "VeryLazy",
        opts = {
            -- add any options here
        },
        dependencies = {
            -- if you lazy-load any plugin below, make sure to add proper `module="..."` entries
            "MunifTanjim/nui.nvim",
            -- OPTIONAL:
            --   `nvim-notify` is only needed, if you want to use the notification view.
            --   If not available, we use `mini` as the fallback
            "rcarriga/nvim-notify",
        },
        config = function()
            require("noice").setup({
                lsp = {
                    progress = {
                        enabled = false,
                    },
                    signature = {
                        enabled = false,
                    },
                    -- override markdown rendering so that **cmp** and other plugins use **Treesitter**
                    override = {
                        ["vim.lsp.util.convert_input_to_markdown_lines"] = true,
                        ["vim.lsp.util.stylize_markdown"] = true,
                        ["cmp.entry.get_documentation"] = true, -- requires hrsh7th/nvim-cmp
                    },
                },
                -- you can enable a preset for easier configuration
                presets = {
                    -- bottom_search = true, -- use a classic bottom cmdline for search
                    command_palette = true, -- position the cmdline and popupmenu together
                    long_message_to_split = true, -- long messages will be sent to a split
                    inc_rename = false, -- enables an input dialog for inc-rename.nvim
                    lsp_doc_border = false, -- add a border to hover docs and signature help
                },
                cmdline = {
                    enabled = true, -- enables the Noice cmdline UI
                    view = "cmdline",
                },
                popupmenu = {
                    enabled = true, -- enables the Noice popupmenu UI
                    ---@type 'nui'|'cmp'
                    backend = "nui", -- backend to use to show regular cmdline completions
                },
            })
        end,
    },
    "https://github.com/ron-rs/ron.vim",
    {
        'Exafunction/codeium.vim',
        event = 'BufEnter'
    },
}

require("lazy").setup(plugins)
