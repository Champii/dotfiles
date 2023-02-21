-- Only required if you have packer configured as `opt`
vim.cmd [[packadd packer.nvim]]

return require('packer').startup(function(use)
    use 'glepnir/dashboard-nvim'
    use 'j-hui/fidget.nvim'
    use 'ibhagwan/fzf-lua'
    use 'junegunn/fzf.vim'
    use { 'glepnir/galaxyline.nvim', branch = 'main' }
    use 'lewis6991/gitsigns.nvim'
    use 'b3nj5m1n/kommentary'
    use 'ggandor/lightspeed.nvim'
    use 'lukas-reineke/lsp-format.nvim'
    use 'ray-x/lsp_signature.nvim'
    use 'L3MON4D3/LuaSnip'
    use 'nvim-neo-tree/neo-tree.nvim'
    use 'TimUntersberger/neogit'
    use 'MunifTanjim/nui.nvim'
    use 'windwp/nvim-autopairs'
    -- use 'williamboman/nvim-lsp-installer'
    use 'neovim/nvim-lspconfig'
    use 'nvim-treesitter/nvim-treesitter'
    use 'nvim-treesitter/nvim-treesitter-context'
    use 'nvim-treesitter/nvim-treesitter-refactor'
    use 'nvim-treesitter/nvim-treesitter-textobjects'
    use 'p00f/nvim-ts-rainbow'
    use 'nvim-tree/nvim-web-devicons'
    use 'folke/persistence.nvim'
    use 'nvim-lua/popup.nvim'
    use 'ahmedkhalf/project.nvim'
    use 'luukvbaal/stabilize.nvim'
    use 'lambdalisue/suda.vim'
    use 'akinsho/toggleterm.nvim'
    use 'mbbill/undotree'
    -- use 'glepnir/vim-nix'
    use 'folke/which-key.nvim'
    use 'hrsh7th/nvim-cmp'
    -- # use 'glepnir/copilot-lua'
    use 'github/copilot.vim'
    use 'hrsh7th/cmp-buffer'
    use 'hrsh7th/cmp-nvim-lsp'
    use 'hrsh7th/cmp-nvim-lua'
    use 'saadparwaiz1/cmp_luasnip'
    use 'hrsh7th/cmp-path'
    use 'ray-x/cmp-treesitter'
    -- # use 'glepnir/copilot-cmp'
    use 'm-demare/hlargs.nvim'
    use 'famiu/bufdelete.nvim'
    use 'lukas-reineke/indent-blankline.nvim'
    use 'stevearc/dressing.nvim'
    -- use 'glepnir/vim-gluon'
    -- use 'glepnir/orgmode'
    -- use 'glepnir/nvim-dap'
    -- use 'glepnir/nvim-dap-ui'
    -- use 'glepnir/nvim-dap-virtual-text'
    -- use 'glepnir/dap-buddy'
    -- use 'glepnir/firenvim'
    use 'jackMort/ChatGPT.nvim'
    use 'nvim-lua/plenary.nvim'
    use 'nvim-telescope/telescope.nvim'
    use {
        "folke/trouble.nvim",
        requires = "nvim-tree/nvim-web-devicons",
        config = function()
            require("trouble").setup {
                -- your configuration comes here
                -- or leave it empty to use the default settings
                -- refer to the configuration section below
            }
        end
    }
    use {
        'rmagatti/goto-preview',
        config = function()
            require('goto-preview').setup {}
        end
    }
    use {
        "rcarriga/nvim-notify",
        config = function() require("notify").setup {} end
    }
    use "jose-elias-alvarez/null-ls.nvim"
    use {
        "williamboman/mason.nvim",
        config = function() require('mason').setup {} end
    }
    use {
        "williamboman/mason-lspconfig.nvim",
        config = function() require('mason-lspconfig').setup {} end
    }
    use "rktjmp/highlight-current-n.nvim"
    use { 'michaelb/sniprun', run = 'bash ./install.sh' }
end)
