require('gitsigns').setup({
  signs = {
    add = {
      text = "│",
    },
    change = {
      text = "│",
    },
    delete = {
      text = "_",
    },
    topdelete = {
      text = "‾",
    },
    changedelete = {
      text = "~",
    },
  },
  numhl = false,
  linehl = false,
  on_attach = function(bufnr)
    local gs = package.loaded.gitsigns

    local function map(mode, l, r, opts)
      opts = opts or {}
      opts.buffer = bufnr
      vim.keymap.set(mode, l, r, opts)
    end

    -- Navigation
    map('n', ']h', function()
      if vim.wo.diff then return ']c' end
      vim.schedule(function() gs.next_hunk() end)
      return '<Ignore>'
    end, {expr=true})

    map('n', '[h', function()
      if vim.wo.diff then return '[c' end
      vim.schedule(function() gs.prev_hunk() end)
      return '<Ignore>'
    end, {expr=true})

    -- Actions
    map({'n', 'v'}, '<leader>gs', ':Gitsigns stage_hunk<CR>')
    map({'n', 'v'}, '<leader>gr', ':Gitsigns reset_hunk<CR>')
    map('n', '<leader>gS', gs.stage_buffer)
    map('n', '<leader>gu', gs.undo_stage_hunk)
    map('n', '<leader>gR', gs.reset_buffer)
    map('n', '<leader>gh', gs.preview_hunk)
    map('n', '<leader>gb', function() gs.blame_line{full=true} end)
    map('n', '<leader>gtb', gs.toggle_current_line_blame)
    map('n', '<leader>gd', gs.diffthis)
    map('n', '<leader>gD', function() gs.diffthis('~') end)
    map('n', '<leader>gtd', gs.toggle_deleted)

    -- Text object
    map({'o', 'x'}, 'ih', ':<C-U>Gitsigns select_hunk<CR>')
  end,
  --[[ on_attach = function(bufnr)
    -- Default keymap options
    noremap = true,
    buffer = true,

    ["n ]c"] = {
      expr = true,
      "&diff ? ']h' : '<cmd>lua require\"gitsigns\".next_hunk()<CR>'",
    },
    ["n [c"] = {
      expr = true,
      "&diff ? '[h' : '<cmd>lua require\"gitsigns\".prev_hunk()<CR>'",
    },

    ["n <leader>gS"] = '<cmd>lua require"gitsigns".stage_hunk()<CR>',
    ["n <leader>gu"] = '<cmd>lua require"gitsigns".undo_stage_hunk()<CR>',
    ["n <leader>gr"] = '<cmd>lua require"gitsigns".reset_hunk()<CR>',
    ["n <leader>gR"] = '<cmd>lua require"gitsigns".reset_buffer()<CR>',
    ["n <leader>gh"] = '<cmd>lua require"gitsigns".preview_hunk()<CR>',
    ["n <leader>gb"] = '<cmd>lua require"gitsigns".blame_line()<CR>',

    -- Text objects
    ["o ih"] = ':<C-U>lua require"gitsigns".select_hunk()<CR>',
    ["x ih"] = ':<C-U>lua require"gitsigns".select_hunk()<CR>',
  }, ]]
  watch_gitdir = { interval = 1000, follow_files = true },
  current_line_blame_opts = {
    delay = 1000,
    position = "eol",
  },
  sign_priority = 6,
  update_debounce = 100,
  status_formatter = nil, -- Use default
  diff_opts = {
    internal = true, -- If luajit is present
  },
})
