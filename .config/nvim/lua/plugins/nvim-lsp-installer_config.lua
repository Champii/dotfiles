--[[ local lsp_installer = require("nvim-lsp-installer")

lsp_installer.on_server_ready(function(server)
  local opts = {}
  -- Refer to https://github.com/neovim/nvim-lspconfig/blob/master/doc/server_configurations.md
  server:setup(opts)

  require "lspconfig".rust_analyzer.setup { on_attach = require "lsp-format".on_attach }
    require "lspconfig".sumneko_lua.setup { on_attach = require "lsp-format".on_attach }

end) ]]
require("lspconfig").lua_ls.setup {
    -- format on save
    on_attach = require("lsp-format").on_attach,
}

require("lspconfig").rust_analyzer.setup {
    -- format on save
    on_attach = require("lsp-format").on_attach,
}

require("lspconfig").omnisharp.setup {
    -- format on save
    on_attach = require("lsp-format").on_attach,
}
