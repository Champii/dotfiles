local function map(mode, lhs, rhs, opts)
    vim.api.nvim_set_keymap(mode, lhs, rhs, opts)
end

local opts = { silent = true }

-- Basics
map("i", "jk", "<ESC>", opts)       -- Exit insert mode
map("i", "zx", "<ESC>:w<CR>", opts) -- Exit insert mode and save
map("n", "zx", ":w<CR>", opts)      -- Save

-- Window

---- splits
map('n', 'gh', '<C-w>v<C-l>', opts)  -- split vertical
map('n', 'gb', '<C-w>s<C-j>', opts)  -- split horizontal
map('n', 'gx', ':Bdelete<CR>', opts) -- Close buffer but let the window in place
map('n', 'gX', '<C-w>q', opts)       -- Close the window

---- Movements
map("n", "<C-h>", "<C-w>h", opts)
map("n", "<C-j>", "<C-w>j", opts)
map("n", "<C-k>", "<C-w>k", opts)
map("n", "<C-l>", "<C-w>l", opts)

---- Resize
map("n", "<C-S-h>", "3<C-w>>", opts)
map("n", "<C-S-j>", "3<C-w>+", opts)
map("n", "<C-S-k>", "3<C-w>-", opts)
map("n", "<C-S-l>", "3<C-w><", opts)

-- Fzf
map('n', '<Leader><Leader>', '<cmd>FzfLua files<CR>', opts)
map('n', '<Leader>.', '<cmd>FzfLua files cwd=%:p:h<CR>', opts)
map('n', '<Leader>,', '<cmd>FzfLua buffers<CR>', opts)
map('n', '<Leader>fr', '<cmd>FzfLua oldfiles<CR>', opts)
map('n', '<Leader>/', '<cmd>FzfLua live_grep<CR>', opts)
map('n', '<Leader>:', '<cmd>FzfLua command_history<CR>', opts)
map('n', '<Leader>?', '<cmd>FzfLua<CR>', opts)
map('n', '<Leader><Tab>', '<cmd>FzfLua resume<CR>', opts)
map('n', '<Leader>x', '<cmd>FzfLua lsp_document_diagnostics<CR>', opts)
map('n', '<Leader>X', '<cmd>FzfLua lsp_workspace_diagnostics<CR>', opts)
map('n', '<Leader>s', '<cmd>FzfLua lsp_document_symbols<CR>', opts)
map('n', '<Leader>S', '<cmd>FzfLua lsp_workspace_symbols<CR>', opts)

map('n', '<Leader>ca', '<cmd>FzfLua lsp_code_actions<CR>', opts)
map('n', 'gd', '<cmd>FzfLua lsp_definitions<CR>', opts)
map('n', 'gr', '<cmd>FzfLua lsp_references<CR>', opts)
map('n', 'gI', '<cmd>FzfLua lsp_implementations<CR>', opts)

-- Disable search highlights with ESC
map('n', '<ESC>', ':noh<CR>', opts)

-- Show hover doc
map('n', 'K', ':lua vim.lsp.buf.hover()<CR>', opts)

-- Diagnostics
map('n', '[e', ':lua vim.diagnostic.goto_prev()<CR>zz', opts)
map('n', ']e', ':lua vim.diagnostic.goto_next()<CR>zz', opts)

-- Neogit
map('n', '<Leader>gg', ':lua require("neogit").open({ kind = "split" })<CR>', opts)

-- Session

---- restore the session for the current directory
map("n", "<leader>ql", '<cmd>lua require("persistence").load()<cr>', opts)

---- restore the last session
map("n", "<leader>qL", '<cmd>lua require("persistence").load({ last = true })<cr>', opts)

---- stop Persistence => session won't be saved on exit
map("n", "<leader>qd", '<cmd>lua require("persistence").stop()<cr>', opts)

-- Terminals
map('n', '<Leader>ot', ':ToggleTerm<CR>', opts)
map('n', '<Leader>to', ':ToggleTerm<CR>', opts)

-- PWD
map('n', '<Leader>cd', '<cmd>cd %:p:h<CR>', opts)

-- LSP
---- lsp rename
map('n', '<Leader>cr', ':lua vim.lsp.buf.rename()<CR>', opts)

-- NeoTree
map('n', '<Leader>tn', ':Neotree toggle<CR>', opts)
map('n', '<Leader>tf', ':Neotree focus<CR>', opts)

-- Quit
map('n', '<Leader>qq', ':qa<CR>', opts)


map('n', '<M-j>', 'ddp', opts)
map('n', '<M-k>', 'ddkP', opts)

-- Tabs

map('n', '<Leader>n', ':tabnew<CR>', opts)
map('n', '<Tab>', ':tabnext<CR>', opts)
-- map('n', '<Leader><Tab>', ':tabprevious<CR>', opts)

-- Projects

map('n', '<Leader>pp', ": lua require'telescope'.extensions.projects.projects{}<CR>", opts)

-- mememto

map('n', '<Leader>gx', ": lua require('memento').toggle()<CR>", opts)

local whichkey = require "which-key"
--[[ local keymap = {
    d = {
        name = "Debug",
        R = { "<cmd>lua require'dap'.run_to_cursor()<cr>", "Run to Cursor" },
        E = { "<cmd>lua require'dapui'.eval(vim.fn.input '[Expression] > ')<cr>", "Evaluate Input" },
        C = { "<cmd>lua require'dap'.set_breakpoint(vim.fn.input '[Condition] > ')<cr>", "Conditional Breakpoint" },
        U = { "<cmd>lua require'dapui'.toggle()<cr>", "Toggle UI" },
        b = { "<cmd>lua require'dap'.step_back()<cr>", "Step Back" },
        c = { "<cmd>lua require'dap'.continue()<cr>", "Continue" },
        d = { "<cmd>lua require'dap'.disconnect()<cr>", "Disconnect" },
        e = { "<cmd>lua require'dapui'.eval()<cr>", "Evaluate" },
        g = { "<cmd>lua require'dap'.session()<cr>", "Get Session" },
        h = { "<cmd>lua require'dap.ui.widgets'.hover()<cr>", "Hover Variables" },
        S = { "<cmd>lua require'dap.ui.widgets'.scopes()<cr>", "Scopes" },
        i = { "<cmd>lua require'dap'.step_into()<cr>", "Step Into" },
        o = { "<cmd>lua require'dap'.step_over()<cr>", "Step Over" },
        p = { "<cmd>lua require'dap'.pause.toggle()<cr>", "Pause" },
        q = { "<cmd>lua require'dap'.close()<cr>", "Quit" },
        r = { "<cmd>lua require'dap'.repl.toggle()<cr>", "Toggle Repl" },
        s = { "<cmd>lua require'dap'.continue()<cr>", "Start" },
        t = { "<cmd>lua require'dap'.toggle_breakpoint()<cr>", "Toggle Breakpoint" },
        x = { "<cmd>lua require'dap'.terminate()<cr>", "Terminate" },
        u = { "<cmd>lua require'dap'.step_out()<cr>", "Step Out" },
    },
}

whichkey.register(keymap, {
    mode = "n",
    prefix = "<leader>",
    buffer = nil,
    silent = true,
    noremap = true,
    nowait = false,
})

local keymap_v = {
    name = "Debug",
    e = { "<cmd>lua require'dapui'.eval()<cr>", "Evaluate" },
}
whichkey.register(keymap_v, {
    mode = "v",
    prefix = "<leader>",
    buffer = nil,
    silent = true,
    noremap = true,
    nowait = false,
}) ]]
