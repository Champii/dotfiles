# Neovim Keybindings Cheat Sheet

Leader key: `Space`

Sources:
- `init.lua`
- `lua/keybindings/init.lua`
- `lua/plugins/gitsigns_config.lua`
- `lua/plugins/toggleterm_config.lua`
- `lua/plugins/nvim-treesitter_config.lua`
- `lua/packer-nvim.lua`

## Basics

| Mode | Key | Action |
| --- | --- | --- |
| Insert | `jk` | Exit insert mode |
| Insert | `zx` | Save and exit insert mode |
| Normal | `zx` | Save file |
| Normal | `<Esc>` | Clear search highlights |
| Normal | `K` | Show LSP hover documentation |

## Windows And Buffers

| Mode | Key | Action |
| --- | --- | --- |
| Normal | `gh` | Open vertical split and move right |
| Normal | `gb` | Open horizontal split and move down |
| Normal | `gx` | Close buffer, keep window |
| Normal | `gX` | Close window |
| Normal | `<C-h>` | Move to left window |
| Normal | `<C-j>` | Move to lower window |
| Normal | `<C-k>` | Move to upper window |
| Normal | `<C-l>` | Move to right window |
| Normal | `<C-S-h>` | Increase window width |
| Normal | `<C-S-j>` | Increase window height |
| Normal | `<C-S-k>` | Decrease window height |
| Normal | `<C-S-l>` | Decrease window width |

## Files, Search, And FzfLua

| Mode | Key | Action |
| --- | --- | --- |
| Normal | `<Leader><Leader>` | Find files |
| Normal | `<Leader>.` | Find files in current file directory |
| Normal | `<Leader>,` | Switch buffers |
| Normal | `<Leader>fr` | Open recent files |
| Normal | `<Leader>/` | Live grep |
| Normal | `<Leader>:` | Command history |
| Normal | `<Leader>?` | Open FzfLua builtin picker |
| Normal | `<Leader><Tab>` | Resume last FzfLua picker |

## LSP And Diagnostics

| Mode | Key | Action |
| --- | --- | --- |
| Normal | `<Leader>x` | Document diagnostics |
| Normal | `<Leader>X` | Workspace diagnostics |
| Normal | `<Leader>s` | Document symbols |
| Normal | `<Leader>S` | Workspace symbols |
| Normal | `<Leader>ca` | Code actions |
| Normal | `gd` | Go to definition |
| Normal | `gr` | References |
| Normal | `gI` | Implementations |
| Normal | `[e` | Previous diagnostic |
| Normal | `]e` | Next diagnostic |
| Normal | `<Leader>cr` | Rename symbol |

## Git And Gitsigns

| Mode | Key | Action |
| --- | --- | --- |
| Normal | `<Leader>gg` | Open Neogit in split |
| Normal | `]h` | Next git hunk |
| Normal | `[h` | Previous git hunk |
| Normal, Visual | `<Leader>gs` | Stage hunk |
| Normal, Visual | `<Leader>gr` | Reset hunk |
| Normal | `<Leader>gS` | Stage buffer |
| Normal | `<Leader>gu` | Undo stage hunk |
| Normal | `<Leader>gR` | Reset buffer |
| Normal | `<Leader>gh` | Preview hunk |
| Normal | `<Leader>gb` | Blame current line, full output |
| Normal | `<Leader>gtb` | Toggle current line blame |
| Normal | `<Leader>gd` | Diff current file |
| Normal | `<Leader>gD` | Diff current file against previous revision |
| Normal | `<Leader>gtd` | Toggle deleted lines |
| Operator, Visual | `ih` | Select git hunk text object |

## Sessions

| Mode | Key | Action |
| --- | --- | --- |
| Normal | `<Leader>ql` | Load session for current directory |
| Normal | `<Leader>qL` | Load last session |
| Normal | `<Leader>qd` | Stop Persistence session saving |

## Terminal And Build

| Mode | Key | Action |
| --- | --- | --- |
| Normal | `<Leader>to` | Toggle terminal |
| Normal | `<Leader>tt` | Toggle vertical `cargo test` terminal |
| Normal | `<Leader>tb` | Toggle vertical `cargo build` terminal |
| Terminal | `<Esc>` | Leave terminal mode |
| Terminal | `jk` | Leave terminal mode |
| Terminal | `<C-h>` | Leave terminal mode and move left |
| Terminal | `<C-j>` | Leave terminal mode and move down |
| Terminal | `<C-k>` | Leave terminal mode and move up |
| Terminal | `<C-l>` | Leave terminal mode and move right |

## Project, Directory, Tabs, And Quit

| Mode | Key | Action |
| --- | --- | --- |
| Normal | `<Leader>cd` | Change working directory to current file directory |
| Normal | `<Leader>pp` | Open Telescope projects |
| Normal | `<Leader>n` | New tab |
| Normal | `<Tab>` | Next tab |
| Normal | `<Leader>qq` | Quit all |

## Neo-tree

| Mode | Key | Action |
| --- | --- | --- |
| Normal | `<Leader>tn` | Toggle Neo-tree |
| Normal | `<Leader>tf` | Focus Neo-tree |

## Editing Helpers

| Mode | Key | Action |
| --- | --- | --- |
| Normal | `<M-j>` | Move current line down |
| Normal | `<M-k>` | Move current line up |
| Normal | `<Leader>gx` | Toggle Memento |

## Treesitter

### Incremental Selection

| Mode | Key | Action |
| --- | --- | --- |
| Normal | `gnn` | Start Treesitter selection |
| Visual | `grn` | Increment selection to next node |
| Visual | `grc` | Increment selection to scope |
| Visual | `grm` | Decrement selection |

### Text Objects

| Mode | Key | Action |
| --- | --- | --- |
| Visual, Operator | `af` | Select outer function |
| Visual, Operator | `if` | Select inner function |
| Visual, Operator | `ac` | Select outer class |
| Visual, Operator | `ic` | Select inner class |
| Normal | `]f` | Next function start |
| Normal | `]F` | Next function end |
| Normal | `[f` | Previous function start |
| Normal | `[F` | Previous function end |
| Normal | `]]` | Next class start |
| Normal | `][` | Next class end |
| Normal | `[[` | Previous class start |
| Normal | `[]` | Previous class end |
| Normal | `gD` | Peek definition code |
| Normal, Visual | `g.` | Smart text subject selection |
| Normal, Visual | `g;` | Outer container text subject selection |

### Swaps And Refactors

| Mode | Key | Action |
| --- | --- | --- |
| Normal | `<Leader>>` | Swap parameter with next parameter |
| Normal | `<Leader><` | Swap parameter with previous parameter |
| Normal | `<M-J>` | Swap statement with next statement |
| Normal | `<M-K>` | Swap statement with previous statement |
| Normal | `<Leader>rn` | Treesitter smart rename |

## Macrothis

| Mode | Key | Action |
| --- | --- | --- |
| Normal | `<Leader>mmd` | Delete macro |
| Normal | `<Leader>mme` | Edit macro |
| Normal | `<Leader>mml` | Load macro |
| Normal | `<Leader>mmn` | Rename macro |
| Normal | `<Leader>mmq` | Run macro on all files in quickfix |
| Normal | `<Leader>mmr` | Run macro |
| Normal | `<Leader>mms` | Save macro |
| Normal | `<Leader>mmx` | Edit register |
| Normal | `<Leader>mmp` | Copy register as printable |
| Normal | `<Leader>mmm` | Copy macro as printable |

## Opencode

| Mode | Key | Action |
| --- | --- | --- |
| Normal | `<Leader>o...` | Opencode.nvim keymap prefix, plugin-managed mappings |

## Dashboard Shortcuts

These are dashboard-local shortcut keys shown by `dashboard-nvim` on startup.

| Key | Action |
| --- | --- |
| `u` | `Lazy update` |
| `f` | `FzfLua files` |
| `d` | `FzfLua files cwd=~/.config/nvim/lua/` |

## Notes

- Commented-out mappings are excluded.
- Some plugins may define their own default mappings at runtime; this sheet lists mappings configured in this repository.
- `<Leader>` means `Space` in this config.
