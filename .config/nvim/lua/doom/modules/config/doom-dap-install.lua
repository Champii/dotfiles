return function()
  local log = require("doom.extras.logging")
  local utils = require("doom.utils")
  -- Init dap-install
  local dap_install = require("dap-install")
  dap_install.setup({
    installation_path = vim.fn.stdpath("data") .. "/dapinstall/",
  })

  local dap_lang_lookup = {
    cpp = { "ccppr_vsc" },
    c = { "ccppr_vsc" },
    rust = { "ccppr_vsc" },
    go = { "go_delve" },
    javascript = { "chrome", "jsnode" },
    typescript = { "chrome", "jsnode" },
    ruby = { "ruby_vsc" },
  }

  -- Iterates through langs and installs clients where possible
  local install_dap_clients = function()
    local modules = require("doom.core.config.modules").modules
    local langs = modules.langs

    local installed_clients = require("dap-install.api.debuggers").get_installed_debuggers()
    local available_clients = vim.tbl_keys(require("dap-install.api.debuggers").get_debuggers())

    local daps_to_install = {}

    -- Iterate over each lang, get the required daps and push it to daps_to_install (to avoid duplicates)
    for _, lang in ipairs(langs) do
      local lang_str = lang
      lang = lang:gsub("%s+%+lsp", ""):gsub("%s+%+debug", "")

      -- DAPInstall.nvim has different names for the DAPs so sometimes we need to lookup the correct DAP to install
      if dap_lang_lookup[lang] ~= nil then
        lang = dap_lang_lookup[lang]
      else
        lang = { lang }
      end

      -- Iterate over DAPs installing them one by one
      for _, dap_name in ipairs(lang) do
        -- If the +debug flag exists and the language client is not installed yet
        if lang_str:find("%+debug") and (not utils.has_value(installed_clients, dap_name)) then
          -- Try to install the client only if there is a client available for
          -- the language, oterwise raise a warning
          if utils.has_value(available_clients, dap_name) then
            -- Avoid installing duplicate daps
            if not utils.has_value(daps_to_install, dap_name) then
              table.insert(daps_to_install, dap_name)
            end
          else
            log.warn(
              "The language "
                .. dap_name
                .. ' does not have a DAP client, please remove the "+debug" flag'
            )
          end
        end
      end
    end

    -- Install the daps one by one
    for _, dap_name in ipairs(daps_to_install) do
      require("dap-install.core.install").install_debugger(dap_name)
    end
  end

  install_dap_clients()
  local dap = require("dap")

  dap.adapters.lldb = {
  type = 'executable',
  command = '/usr/bin/lldb-vscode', -- adjust as needed
  name = "lldb"
}
dap.configurations.cpp = {
  {
    name = "Launch",
    type = "lldb",
    request = "launch",
    program = "/home/champii/prog/rust/Rock/target/debug/rock",
    cwd = "/home/champii/prog/rust/Rock/project",
    stopOnEntry = false,
    args = { 'run' },

    -- if you change `runInTerminal` to true, you might need to change the yama/ptrace_scope setting:
    --
    --    echo 0 | sudo tee /proc/sys/kernel/yama/ptrace_scope
    --
    -- Otherwise you might get the following error:
    --
    --    Error on launch: Failed to attach to the target process
    --
    -- But you should be aware of the implications:
    -- https://www.kernel.org/doc/html/latest/admin-guide/LSM/Yama.html
    runInTerminal = false,
  },
}


-- If you want to use this for rust and c, add something like this:

dap.configurations.c = dap.configurations.cpp
dap.configurations.rust = dap.configurations.cpp

end
