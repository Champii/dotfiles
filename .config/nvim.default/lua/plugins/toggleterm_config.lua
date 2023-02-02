require("toggleterm").setup{
    shade_terminals = false,
    -- shading_factor = 3,
}

function _G.set_terminal_keymaps()
  local opts = {noremap = true}
  vim.api.nvim_buf_set_keymap(0, 't', '<esc>', [[<C-\><C-n>]], opts)
  vim.api.nvim_buf_set_keymap(0, 't', 'jk', [[<C-\><C-n>]], opts)
  vim.api.nvim_buf_set_keymap(0, 't', '<C-h>', [[<C-\><C-n><C-W>h]], opts)
  vim.api.nvim_buf_set_keymap(0, 't', '<C-j>', [[<C-\><C-n><C-W>j]], opts)
  vim.api.nvim_buf_set_keymap(0, 't', '<C-k>', [[<C-\><C-n><C-W>k]], opts)
  vim.api.nvim_buf_set_keymap(0, 't', '<C-l>', [[<C-\><C-n><C-W>l]], opts)
  _G.set_terminal_color()
  vim.api.nvim_command('startinsert')
end

-- if you only want these mappings for toggle term use term://*toggleterm#* instead
vim.cmd('autocmd! TermOpen term://*toggleterm#* lua set_terminal_keymaps()')

function _G.set_terminal_color()
    vim.cmd('hi ActiveTWindow guibg=#111111')
    vim.cmd('setlocal winhighlight=Normal:ActiveTWindow,NormalNC:ActiveTWindow,CursorLine:ActiveTWindow')
end



local Terminal  = require('toggleterm.terminal').Terminal
local cargoTest = Terminal:new({ cmd = "cargo test", hidden = true, direction = "vertical", close_on_exit = false })

function _cargo_test()
  cargoTest:toggle(100)
end

local cargoBuild = Terminal:new({ cmd = "cargo build", hidden = true, direction = "vertical", close_on_exit = false })

function _cargo_build()
  cargoBuild:toggle(80)
end

vim.api.nvim_set_keymap("n", "<leader>tt", "<cmd>lua _cargo_test()<CR>", {noremap = true, silent = true})
vim.api.nvim_set_keymap("n", "<leader>tb", "<cmd>lua _cargo_build()<CR>", {noremap = true, silent = true})


