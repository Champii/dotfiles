-- doom_userplugins - Doom nvim custom plugins
--
-- This file contains all the custom plugins that are not in Doom nvim but that
-- the user requires. All the available fields can be found here
-- https://github.com/wbthomason/packer.nvim#specifying-plugins
--
-- By example, for including a plugin with a dependency on telescope:
-- M.plugins {
--   {
--     'user/repository',
--     requires = { 'nvim-lua/telescope.nvim' },
--   },
-- }

local M = {}

M.source = debug.getinfo(1, "S").source:sub(2)

M.plugins = {
	--[[ {
		'sunjon/shade.nvim',
		config = function()
			require'shade'.setup({
  			overlay_opacity = 70,
  			opacity_step = 1,
			})
		end
	}, ]]
	{
		'petertriho/nvim-scrollbar',
		config = function()
			require'scrollbar'.setup({
    		handle = {
        	color = "#202020",
    		},
			})
		end
	},
	--[[ {
		'nyngwang/NeoRoot.lua',
		config = function()
			-- require'neo-root'.setup()
		end
	}, ]]
	--[[ {
		'simrat39/rust-tools.nvim',
		config = function()
			require'rust-tools'.setup({
 				tools = {
 					autoSetHints = true,
        	hover_with_actions = true,
        	inlay_hints = {
          	show_parameter_hints = false,
          	parameter_hints_prefix = "",
          	other_hints_prefix = "",
        	},
 				},
  			server = {
        	-- on_attach is a callback called when the language server attachs to the buffer
        	-- on_attach = on_attach,
        	settings = {
            -- to enable rust-analyzer settings visit:
            -- https://github.com/rust-analyzer/rust-analyzer/blob/master/docs/user/generated_config.adoc
            ["rust-analyzer"] = {
              -- enable clippy on save
              checkOnSave = {
                command = "clippy"
              },
            }
        	}
    		},
			})
			vim.diagnostic.config({
  			virtual_text = true,
  			signs = true,
  			underline = true,
  			update_in_insert = true,
  			severity_sort = false,
			})
			vim.o.updatetime = 250
			-- vim.cmd [[autocmd CursorHold,CursorHoldI * lua vim.diagnostic.open_float(nil, {focus=false})]]
			-- vim.diagnostic.show()
			-- vim.lsp.buf.signature_help()
		--[[ end
	}, ]]
	{
  	'ahmedkhalf/project.nvim',
  	-- after = "telescope",
  	config = function()
    	require("project_nvim").setup {
      	-- your configuration comes here
      	-- or leave it empty to use the default settings
      	-- refer to the configuration section below
    	}
			require('telescope').load_extension('projects')
  	end
	},
	--[[ {
'luochen1990/rainbow',
-- after = "telescope",
config = function()
vim.cmd("let g:rainbow_active = 1")
end
}, ]]
	{
  	'p00f/nvim-ts-rainbow',
 		after = "nvim-treesitter",
  	config = function()
  	end
	},
	{
 		'weilbith/nvim-code-action-menu',
  	cmd = 'CodeActionMenu',
	},
	--[[ {
'glacambre/firenvim',
run = function() vim.fn['firenvim#install'](0) end
}, ]]
	{
  	'romgrk/nvim-treesitter-context',
 		after = "nvim-treesitter",
  	config = function()
			require'treesitter-context'.setup({
 				enable = true, -- Enable this plugin (Can be enabled/disabled later via commands)
    		-- throttle = true, -- Throttles plugin updates (may improve performance)
    		max_lines = 0,
				patterns = { -- Match patterns for TS nodes. These get wrapped to match at word boundaries.
        	-- For all filetypes
        	-- Note that setting an entry here replaces all other patterns for this entry.
        	-- By setting the 'default' entry below, you can control which nodes you want to
        	-- appear in the context window.
        	default = {
            'class',
            'function',
            'method',
            'for', -- These won't appear in the context
            'while',
            'if',
            'switch',
            'case',
            'match',
        	},
        },
			})
  	end
	},
	{
  	'AllenDang/nvim-expand-expr',
  	config = function()
  	end
	},
	{
		'sudormrfbin/cheatsheet.nvim',

  	requires = {
    	{'nvim-telescope/telescope.nvim'},
    	{'nvim-lua/popup.nvim'},
    	{'nvim-lua/plenary.nvim'},
  	}
	},

	{
		"luukvbaal/stabilize.nvim",
		config = function() require("stabilize").setup() end
	},
	{
		'ggandor/lightspeed.nvim',
	},
	{
		'mbbill/undotree'
	},
	{
    'j-hui/fidget.nvim',
    config = function()
local mappings = require('doom.utils.mappings')
mappings.map(
  "n",
  "K",
  ":lua vim.lsp.buf.hover()<CR>",
  {},
  "LSP",
  "hover_doc",
  "Hover documentation"
)
			require('fidget').setup()
		end
	},
	--[[ {
		'rust-lang/rust.vim',
		config = function()
			vim.cmd('let g:rustfmt_autosave = 1')
			-- require('rust.vim').setup()
		end
	}, ]]
	{
		"junegunn/fzf",
	},
	{
		"ibhagwan/fzf-lua",
		requires = { 'kyazdani42/nvim-web-devicons' },
		config = function()
			local actions = require "fzf-lua.actions"

			require('fzf-lua').setup({
    		winopts = {
      		-- split         = "botright new",-- open in a split instead?
					border = false,
					-- win_row = 1,
					height = 0.2,
					width = 0.8,
					row = 0.8,
					col = 0.5,
					preview = {
						default = 'bat',
						-- border = 'noborder',
						vertical = "down:80%",
						horizontal = "right:40%",
					},
  				on_create = function()
  					-- vim.cmd("resize 20")

    				-- called once upon creation of the fzf main window
    				-- can be used to add custom fzf-lua mappings, e.g:
    				--   vim.api.nvim_buf_set_keymap(0, "t", "<C-j>", "<Down>",
    				--     { silent = true, noremap = true })
  				end,
    		},
				-- actions = {
					-- files = {
						-- ["default"]     = actions.buf_edit,
					-- },
				-- },
			})
		end
	},
	--[[ {
    	"nvim-neorg/neorg",
    	-- tag = "latest", (see below)
    	config = function()
        	require('neorg').setup {
            	-- ... -- check out setup part...
        	}
    	end,
    	requires = "nvim-lua/plenary.nvim"
	}, ]]

}

return M

-- vim: sw=2 sts=2 ts=2 noexpandtab
