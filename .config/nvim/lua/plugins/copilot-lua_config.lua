function mysplit (inputstr, sep)
    if sep == nil then
	sep = "%s"
    end
    local t={}
    for str in string.gmatch(inputstr, "([^"..sep.."]+)") do
	table.insert(t, str)
    end
    return t
end

local packpath = mysplit(vim.o.packpath, ',')[1]
local indexjs_path = packpath .. "/pack/home-manager/start/vimplugin-copilot.lua/copilot/index.js"
local home_path = packpath .. "/pack/home-manager"

-- require("copilot.copilot_handler").start({ copilot_path = indexjs_path })
require("copilot").setup {
    plugin_manager_path = home_path
}

--[[ local copilot_attach = vim.api.nvim_create_augroup("copilot_attach",
                                                   {clear = true})

vim.api.nvim_create_autocmd({"BufEnter"}, {
    pattern = "*",
    group = copilot_attach,
    callback = function()
        local client_id = require("copilot.util").find_copilot_client()
        if vim.api.nvim_get_current_buf() ~= nil and vim.bo.filetype ~= "" and
            client_id then vim.lsp.buf_attach_client(0, client_id) end
    end
})
 ]]
