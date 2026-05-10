package.preload["galaxyline.provider_lsp"] = function()
    local function get_lsp_client(msg)
        msg = msg or "No Active Lsp"

        local bufnr = vim.api.nvim_get_current_buf()
        local clients = vim.lsp.get_clients({ bufnr = bufnr })
        if #clients == 0 then
            return msg
        end

        local buf_ft = vim.bo[bufnr].filetype
        for _, client in ipairs(clients) do
            local filetypes = client.config.filetypes
            if not filetypes or vim.tbl_contains(filetypes, buf_ft) then
                return client.name
            end
        end

        return msg
    end

    return { get_lsp_client = get_lsp_client }
end

package.preload["galaxyline.provider_diagnostic"] = function()
    local function get_coc_diagnostic(diag_type)
        local has_info, info = pcall(vim.api.nvim_buf_get_var, 0, "coc_diagnostic_info")
        if has_info and info[diag_type] and info[diag_type] > 0 then
            return info[diag_type]
        end
        return ""
    end

    local function get_nvim_lsp_diagnostic(severity)
        if #vim.lsp.get_clients({ bufnr = 0 }) == 0 then
            return ""
        end

        local result = vim.diagnostic.get(0, { severity = severity })
        if result and #result ~= 0 then
            return #result .. " "
        end

        return ""
    end

    local function get_diagnostic(diag_type, severity)
        if vim.fn.exists("*coc#rpc#start_server") == 1 then
            return get_coc_diagnostic(diag_type)
        end
        return get_nvim_lsp_diagnostic(severity)
    end

    return {
        get_diagnostic_error = function()
            return get_diagnostic("error", vim.diagnostic.severity.ERROR)
        end,
        get_diagnostic_warn = function()
            return get_diagnostic("warning", vim.diagnostic.severity.WARN)
        end,
        get_diagnostic_hint = function()
            return get_diagnostic("hint", vim.diagnostic.severity.HINT)
        end,
        get_diagnostic_info = function()
            return get_diagnostic("information", vim.diagnostic.severity.INFO)
        end,
    }
end

local colors2 = require("galaxyline.theme").default

local gl = require("galaxyline")
local lsp = require("galaxyline.provider_lsp")
local buffer = require("galaxyline.provider_buffer")
local condition = require("galaxyline.condition")

local gls = gl.section

local new_bg = "#101010"

local function colors(name)
    if name == "bg" then
        return new_bg
    end

    return colors2[name]
end

gl.short_line_list = {
    "NvimTree",
    "NeoTree",
    "packer",
    "minimap",
    "Outline",
    "toggleterm",
    "netrw",
}


-- {{{ Utility functions
local function is_dashboard()
    --[[ local buftype = buffer.get_buffer_filetype()
    return buftype == "DASHBOARD" ]]
    return false
end

local function is_not_dashboard()
    --[[ local buftype = buffer.get_buffer_filetype()
    return buftype ~= "DASHBOARD" ]]
    return true
end


vim.api.nvim_command('autocmd WinLeave * lua require("galaxyline").load_galaxyline()')

-- Left side
gls.left[1] = {
    RainbowLeft = {
        provider = function()
            return "▊ "
        end,
        highlight = { colors("orange"), colors("bg") },
    },
}
gls.left[2] = {
    ViMode = {
        provider = function()
            -- auto change color according the vim mode
            -- TODO: find a less dirty way to set ViMode colors
            local mode_color = {
                n = colors("red"),
                i = colors("green"),
                v = colors("blue"),
                ["CTRL-V"] = colors("blue"),
                V = colors("blue"),
                c = colors("magenta"),
                no = colors("red"),
                s = colors("orange"),
                S = colors("orange"),
                [""] = colors("orange"),
                ic = colors("yellow"),
                R = colors("magenta"),
                Rv = colors("magenta"),
                cv = colors("red"),
                ce = colors("red"),
                r = colors("cyan"),
                rm = colors("cyan"),
                ["r?"] = colors("cyan"),
                ["!"] = colors("red"),
                t = colors("red"),
            }
            if mode_color[vim.fn.mode()] ~= nil then
                vim.api.nvim_command('hi GalaxyViMode guifg=' .. mode_color[vim.fn.mode()])
            end
            return "  "
        end,
        highlight = { colors("red"), colors("bg"), "bold" },
    },
}
gls.left[3] = {
    FileSize = {
        provider = "FileSize",
        condition = condition.buffer_not_empty and condition.hide_in_width,
        highlight = {
            colors("green"),
            colors("bg"),
        },
        -- separator = " ",
        --
        separator = " ",
        separator_highlight = { colors("green"), colors("bg") },
    },
}
gls.left[4] = {
    FileIcon = {
        provider = "FileIcon",
        condition = condition.buffer_not_empty and is_not_dashboard,
        highlight = {
            require("galaxyline.provider_fileinfo").get_file_icon_color,
            colors("bg"),
        },
    },
}
gls.left[5] = {
    FileName = {
        provider = "FilePath",
        condition = condition.buffer_not_empty and is_not_dashboard,
        highlight = { colors("yellow"), colors("bg"), "bold" },
        separator = " ",
        separator_highlight = { colors("bg"), colors("bg") },
    },
}
gls.left[6] = {
    LineInfo = {
        provider = function()
            local line = vim.fn.line(".")
            local column = vim.fn.col(".")
            return string.format("%3d:%2d  ", line, column)
        end,
        condition = is_not_dashboard,
        highlight = { colors("magenta"), colors("bg") },
    },
}
gls.left[7] = {
    LinePercent = {
        provider = "LinePercent",
        condition = is_not_dashboard,
        highlight = { colors("blue"), colors("bg") },
        separator = "  ",
        separator_highlight = { colors("bg"), colors("bg") },
    },
}
gls.left[8] = {
    DiagnosticError = {
        provider = "DiagnosticError",
        condition = is_not_dashboard,
        icon = "",
        highlight = { colors("red"), colors("bg") },
    },
}
gls.left[9] = {
    DiagnosticWarn = {
        provider = "DiagnosticWarn",
        condition = is_not_dashboard,
        icon = "",
        highlight = { colors("orange"), colors("bg") },
    },
}
gls.left[10] = {
    DiagnosticInfo = {
        provider = "DiagnosticInfo",
        condition = is_not_dashboard,
        icon = "",
        highlight = { colors("blue"), colors("bg") },
    },
}

-- Right side
-- alternate separator colors if the current buffer is a dashboard
gls.right[1] = {
    FileFormat = {
        provider = "FileFormat",
        condition = condition.hide_in_width and is_not_dashboard,
        highlight = { colors("red"), colors("bg") },
        separator = "  ",
        separator_highlight = { colors("bg"), colors("bg") },
    },
}

gls.right[2] = {
    FileEncode = {
        provider = "FileEncode",
        condition = condition.hide_in_width and is_not_dashboard,
        highlight = { colors("orange"), colors("bg") },
        separator = " ",
        separator_highlight = { colors("bg"), colors("bg") },
    },
}
gls.right[3] = {
    ShowLspClientOrFileType = {
        provider = function()
            -- Check if there's a LSP client running to avoid redundant
            -- statusline elements
            if lsp.get_lsp_client() ~= "No Active Lsp" then
                return string.format(" %s » %s ", vim.bo.filetype, lsp.get_lsp_client())
            else
                -- Use the filetype instead
                return string.format(" %s ", vim.bo.filetype)
            end
        end,
        condition = function()
            local tbl = { ["dashboard"] = true, [""] = true }
            if tbl[vim.bo.filetype] then
                return false
            end
            return true
        end,
        highlight = { colors("blue"), colors("bg") },
        separator = "  ",
        separator_highlight = { colors("bg"), colors("bg") },
    },
}
gls.right[4] = {
    GitIcon = {
        provider = function()
            return " "
        end,
        condition = condition.check_git_workspace,
        highlight = { colors("red"), colors("bg") },
        separator = " ",
        separator_highlight = { colors("bg"), colors("bg") },
    },
}
gls.right[5] = {
    GitBranch = {
        provider = "GitBranch",
        condition = condition.check_git_workspace,
        highlight = { colors("green"), colors("bg"), "bold" },
    },
}
gls.right[6] = {
    DiffSeparator = {
        provider = function()
            return "   "
        end,
        condition = condition.hide_in_width,
        highlight = { colors("bg"), colors("bg") },
    },
}
gls.right[7] = {
    DiffAdd = {
        provider = "DiffAdd",
        condition = condition.hide_in_width,
        icon = " ",
        highlight = { colors("green"), colors("bg") },
    },
}
gls.right[8] = {
    DiffModified = {
        provider = "DiffModified",
        condition = condition.hide_in_width,
        icon = " ",
        highlight = { colors("orange"), colors("bg") },
    },
}
gls.right[9] = {
    DiffRemove = {
        provider = "DiffRemove",
        condition = condition.hide_in_width,
        icon = " ",
        highlight = { colors("red"), colors("bg") },
    },
}

-- If the current buffer is the dashboard then show Doom Nvim version
if is_dashboard then
    gls.right[10] = {
        DoomVersion = {
            provider = function()
                return "Pii v0.1.0"
            end,
            condition = is_dashboard,
            highlight = { colors("green"), colors("bg"), "bold" },
            separator = "  ",
            separator_highlight = {
                colors("bg"),
                colors("bg"),
            },
        },
    }
end
gls.right[11]           = {
    RainbowRight = {
        provider = function()
            return " ▊"
        end,
        highlight = { colors("orange"), colors("bg") },
    },
}

-- Short status line
gls.short_line_left[1]  = {
    ShortRainbowLeft = {
        provider = function()
            return "▊ "
        end,
        highlight = { colors("blue"), colors("bg") },
    },
}
gls.short_line_left[2]  = gls.left[5]

gls.short_line_left[3]  = gls.left[8]
gls.short_line_left[4]  = gls.left[9]
gls.short_line_left[5]  = gls.left[10]

gls.short_line_right[1] = {
    BufferIcon = {
        provider = "BufferIcon",
        condition = is_not_dashboard,
        highlight = { colors("yellow"), colors("bg") },
    },
}
gls.short_line_right[2] = {
    BufferType = {
        provider = "FileTypeName",
        condition = is_not_dashboard,
        highlight = { colors("fg"), colors("bg") },
    },
}

gls.short_line_right[3] = {
    ShortRainbowRight = {
        provider = function()
            return " ▊"
        end,
        highlight = { colors("blue"), colors("bg") },
    },
}
