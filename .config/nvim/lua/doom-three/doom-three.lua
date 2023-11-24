local M = {}

-- Highlight Function And Color definitons {{{
local util = {
    bg = '#171717'
}

local function hexToRgb(hex_str)
    local hex = "[abcdef0-9][abcdef0-9]"
    local pat = "^#(" .. hex .. ")(" .. hex .. ")(" .. hex .. ")$"
    hex_str = string.lower(hex_str)

    assert(string.find(hex_str, pat) ~= nil, "hex_to_rgb: invalid hex_str: " .. tostring(hex_str))

    local r, g, b = string.match(hex_str, pat)
    return { tonumber(r, 16), tonumber(g, 16), tonumber(b, 16) }
end

function util.blend(fg, bg, alpha)
    bg = hexToRgb(bg)
    fg = hexToRgb(fg)

    local blendChannel = function(i)
        local ret = (alpha * fg[i] + ((1 - alpha) * bg[i]))
        return math.floor(math.min(math.max(0, ret), 255) + 0.5)
    end

    return string.format("#%02X%02X%02X", blendChannel(1), blendChannel(2), blendChannel(3))
end

function util.darken(hex, amount, bg)
    return util.blend(hex, bg or util.bg, math.abs(amount))
end

local function highlight(group, opts)
    vim.api.nvim_set_hl(0, group, opts)
end

vim.highlight.priorities.semantic_tokens = 1000

local bg_dark                            = '#141414'
local bg_darker                          = '#111111'
local bg                                 = '#171717'
local bg_light                           = '#202020'
local bg_lighter                         = '#232323'
local grey                               = '#6d6d6d'
local grey_dark                          = '#606060'
local grey_light                         = '#858585'
local grey_lighter                       = '#abb2bf'
local red                                = '#d34444'
local heavy_red                          = '#e61f44'
local green                              = '#4baf1b'
local blue                               = '#5eadfc'
local blue_dark                          = '#1674d3'
local yellow                             = '#cfcfbf'
local yellow_light                       = '#ced331'
local orange                             = '#ff7000'
local orange_light                       = '#ff8800'
local purple                             = '#c11971'
local cyan_dark                          = '#89bbdd'
local cyan                               = '#89ddff'
local fg                                 = '#a6accd'
local fg_light                           = '#fbfbfb'
local fg_dark                            = '#676e96'
local fg_darker                          = '#575e86'
local hollow                             = '#424760'
local hollow_lighter                     = '#30354e'
local white                              = '#ffffff'

local green_blue                         = '#35cdaf'
local orange_dark                        = '#a1561c'
local beige                              = '#b78181'
local purple_light                       = '#cf81cf'

local black                              = '#000000'



-- }}}

-- Editor Highlight Groups {{{

local editor_syntax = {
    ColorColumn              = { bg = bg_light },
    CursorLine               = { bg = bg_lighter },
    Cursor                   = { fg = bg_dark, bg = yellow },
    Directory                = { fg = blue, bold = true },
    DiffAdd                  = { fg = green },
    DiffChange               = { fg = blue },
    DiffDelete               = { fg = red },
    DiffText                 = { fg = yellow },
    EndOfBuffer              = { fg = bg },
    ErrorMsg                 = { fg = red, bold = true },
    VertSplit                = { bg = bg, fg = grey_dark },
    Folded                   = { fg = fg_dark, italic = true },
    FoldColumn               = { fg = yellow },
    SignColumn               = { fg = yellow },
    IncSearch                = { bg = yellow, fg = bg },
    Substitute               = { bg = blue, fg = bg },
    LineNr                   = { fg = grey_light },
    CursorLineNr             = { fg = grey_lighter },
    MatchParen               = { bold = true, undercurl = true },
    Normal                   = { fg = beige, bg = bg },
    NormalNC                 = { fg = beige, bg = bg_dark },
    NormalFloat              = { bg = bg_dark },
    FloatBorder              = { fg = bg },
    FloatTitle               = { bg = bg_dark, fg = fg_light },
    Pmenu                    = { bg = bg, fg = grey },
    PmenuSel                 = { bg = bg_lighter, bold = true },
    PmenuSbar                = { bg = fg_darker },
    PmenuThumb               = { bg = grey },
    Search                   = { bg = hollow },
    SpecialKey               = { bg = bg_light },
    SpellBad                 = { undercurl = true, sp = red },
    SpellCap                 = { undercurl = true, sp = yellow },
    SpellLocal               = { undercurl = true, sp = orange },
    SpellRare                = { undercurl = true, sp = blue },
    TabLine                  = { bg = bg_dark, fg = fg_light },
    TabLineFill              = { bg = bg_dark, fg = fg_light },
    TabLineSel               = { bg = cyan, fg = bg_dark, bold = true },
    Title                    = { fg = green },
    Visual                   = { bg = hollow_lighter },
    VisualNOS                = { bg = hollow_lighter },
    WarningMsg               = { fg = yellow, italic = true },
    Whitespace               = { bg = bg, fg = fg_darker }, -- TODO: i don't know where this is

    -- lsp
    DiagnosticError          = { fg = red, bold = true },
    DiagnosticWarn           = { fg = orange, bold = true },
    DiagnosticInfo           = { fg = yellow, bold = true },
    DiagnosticHint           = { fg = grey, bold = true },
    DiagnosticOk             = { fg = grey_light, bold = true },

    DiagnosticUnderlineError = { undercurl = true, sp = red },
    DiagnosticUnderlineWarn  = { undercurl = true, sp = orange },
    DiagnosticUnderlineInfo  = { undercurl = true, sp = yellow },
    DiagnosticUnderlineHint  = { undercurl = true, sp = green },
    DiagnosticUnderlineOk    = { undercurl = true, sp = yellow_light },

    -- git highlighting
    gitcommitComment         = { fg = fg_dark, italic = true },
    gitcommitUntracked       = { fg = fg_dark, italic = true },
    gitcommitDiscarded       = { fg = fg_dark, italic = true },
    gitcommitSelected        = { fg = fg_dark, italic = true },
    gitcommitUnmerged        = { fg = green },
    gitcommitBranch          = { fg = purple },
    gitcommitNoBranch        = { fg = purple },
    gitcommitDiscardedType   = { fg = red },
    gitcommitSelectedType    = { fg = green },
    gitcommitUntrackedFile   = { fg = cyan },
    gitcommitDiscardedFile   = { fg = red },
    gitcommitDiscardedArrow  = { fg = red },
    gitcommitSelectedFile    = { fg = green },
    gitcommitSelectedArrow   = { fg = green },
    gitcommitUnmergedFile    = { fg = yellow },
    gitcommitUnmergedArrow   = { fg = yellow },
    gitcommitSummary         = { fg = fg_light },
    gitcommitOverflow        = { fg = red },
    gitcommitOnBranch        = {},
    gitcommitHeader          = {},
    gitcommitFile            = {},

    -- User dependent groups, probably useless to change the default:
    Conceal                  = {},
    ModeMsg                  = {},
    MsgArea                  = {},
    MsgSeparator             = { fg = bg_dark },
    MoreMsg                  = {},
    NonText                  = {},
    Question                 = {},
    QuickFixLine             = {},
    StatusLine               = { bg = "#000000" },
    StatusLineNC             = {},
    WildMenu                 = {}
}

-- }}}

-- Vim Default Code Syntax {{{

local code_syntax   = {
    Comment        = { fg = grey, italic = true },
    Constant       = { fg = orange_light },
    String         = { fg = orange_dark },
    Character      = { fg = red_light, bold = true },
    Number         = { fg = orange },
    Float          = { fg = orange },
    Boolean        = { fg = orange },

    Identifier     = { fg = beige },
    Function       = { fg = blue, italic = true },

    Statement      = { fg = blue_dark, italic = true },
    Conditional    = { fg = heavy_red, italic = true },
    Repeat         = { fg = heavy_red, italic = true },
    Label          = { fg = blue, italic = true },
    Exception      = { fg = blue, italic = true },
    Operator       = { fg = red },
    Keyword        = { fg = heavy_red },

    Include        = { fg = blue_dark },
    Define         = { fg = purple },
    Macro          = { fg = purple },
    PreProc        = { fg = yellow },
    PreCondit      = { fg = yellow },

    Type           = { fg = green },
    StorageClass   = { fg = yellow },
    Structure      = { fg = yellow },
    Typedef        = { fg = yellow },

    Special        = { fg = blue },
    SpecialChar    = {},
    Tag            = { fg = orange },
    SpecialComment = { fg = fg_dark, bold = true },
    Debug          = {},
    Delimiter      = {},

    Ignore         = {},
    Underlined     = { underline = true },
    Error          = { fg = heavy_red },
    Todo           = { fg = purple, bold = true },
}

-- }}}

-- Plugin Highlight Groups {{{

local plugin_syntax = {
    GitGutterAdd               = { fg = green },
    GitGutterChange            = { fg = blue },
    GitGutterDelete            = { fg = red },
    GitGutterChangeDelete      = { fg = orange },

    diffAdded                  = { fg = green },
    diffRemoved                = { fg = heavy_red },

    ['@punctuation.delimiter'] = { fg = red },
    ['@punctuation.bracket']   = { fg = red },
    ['@punctuation.special']   = { fg = red },

    ['@constant']              = { fg = orange_light },
    ['@constant.builtin']      = { fg = orange_light },
    ['@constant.macro']        = { fg = yellow },

    ['@string']                = { fg = orange_dark },
    ['@string.regex']          = { fg = cyan_dark },
    ['@string.escape']         = { fg = cyan_dark },
    ['@string.special']        = { fg = cyan },

    ['@number']                = { fg = orange },
    ['@boolean']               = { fg = orange },
    ['@float']                 = { fg = orange },

    ['@function']              = { fg = blue },
    ['@function.call']         = { fg = blue },
    ['@function.builtin']      = { fg = orange },
    ['@function.macro']        = { fg = purple },

    ['@parameter']             = { fg = yellow },
    ['@constructor']           = { fg = yellow },

    ['@method']                = { fg = blue },
    ['@method.call']           = { fg = blue },

    ['@field']                 = { fg = green_blue },
    ['@property']              = { fg = green_blue },

    ['@conditional']           = { fg = red, italic = true },
    ['@repeat']                = { fg = red, italic = true },
    ['@exception']             = { fg = blue, italic = true },
    ['@label']                 = { fg = cyan_dark, italic = true },
    ['@debug']                 = { fg = cyan_dark, italic = true },
    ['@include']               = { fg = red },
    ['@namespace']             = { fg = purple_light },

    ['@operator']              = { fg = red },
    ['@comment']               = { fg = grey, italic = true },
    ['@error']                 = {},
    ['@preproc']               = { fg = yellow },
    ['@define']                = { fg = purple_light },

    ['@keyword']               = { fg = red },
    ['@keyword.function']      = { fg = orange },
    ['@keyword.operator']      = { fg = red },
    ['@keyword.return']        = { fg = red },

    ['@type']                  = { fg = green },
    ['@type.builtin']          = { fg = orange },
    ['@type.definition']       = { fg = yellow_light },
    ['@type.qualifier']        = { fg = green },

    ['@tag']                   = { fg = blue_dark },
    ['@tag.delimiter']         = { fg = cyan },
    ['@tag.attribute']         = { fg = blue_dark },

    ['@symbol']                = { fg = orange_light },
    ['@variable']              = { fg = beige },
    ['@variable.builtin']      = { fg = orange },

    ['@text.title']            = { bold = true, underline = true },
    ['@text.reference']        = { fg = cyan },
    ['@text.uri']              = { underline = true, fg = green },
    ['@text.warning']          = { fg = purple, bold = true },
    ['@text.literal']          = { fg = fg_light },


    -- TODO: maybe implement this at some point, disable for now
    ['@lsp.type.class']         = {},
    ['@lsp.type.decorator']     = {},
    ['@lsp.type.enumMember']    = {},
    ['@lsp.type.function']      = { fg = blue },
    ['@lsp.type.interface']     = { fg = cyan },
    ['@lsp.type.struct']        = { fg = green },
    ['@lsp.type.enum']          = { fg = yellow_light },
    ['@lsp.type.typeParameter'] = { fg = orange },
    ['@lsp.type.keyword']       = {},
    ['@lsp.type.label']         = {},
    ['@lsp.type.macro']         = {},
    ['@lsp.type.method']        = { fg = blue_dark },
    ['@lsp.type.builtinType']   = { fg = orange },
    ['@lsp.type.typeAlias']     = { fg = orange },
    ['@lsp.type.namespace']     = {},
    ['@lsp.type.parameter']     = {},
    ['@lsp.type.property']      = {},
    ['@lsp.type.type']          = {},
    ['@lsp.type.variable']      = {},


    -- nvim-cmp
    CmpItemAbbr           = { fg = beige },
    CmpItemAbbrDeprecated = { fg = grey_dark },
    CmpItemAbbrMatch      = { fg = blue },
    CmpItemAbbrMatchFuzzy = { fg = cyan },
    -- TODO: not sure where this goes
    CmpItemKind           = { fg = white, bold = true },
    CmpItemKindVariable   = { fg = beige, bold = true },
    CmpItemKindInterface  = { fg = cyan, bold = true },
    CmpItemKindText       = { fg = fg_darker, bold = true },
    CmpItemKindMethod     = { fg = blue, bold = true },
    CmpItemKindModule     = { fg = purple, bold = true },
    CmpItemKindReference  = { fg = red, bold = true },
    CmpItemKindFunction   = { fg = blue, bold = true },
    CmpItemKindStruct     = { fg = green, bold = true },
    CmpItemKindSnippet    = { fg = orange, bold = true },
    CmpItemKindKeyword    = { fg = red, bold = true },
    CmpItemKindProperty   = { fg = green_blue, bold = true },
    CmpItemKindField      = { fg = green_blue, bold = true },
    CmpItemKindUnit       = { fg = white, bold = true },
    CmpItemMenu           = { fg = purple },


    NeotestAdapterName                = { fg = blue_dark },
    NeotestBorder                     = { fg = '#ffa000' }, -- TODO
    NeotestDir                        = { fg = blue, bold = true },
    NeotestExpandMarker               = { fg = grey },
    NeotestFailed                     = { fg = heavy_red },
    NeotestFile                       = { fg = purple, bold = true, italic = true },
    NeotestFocused                    = { bold = true, underline = true },
    NeotestIndent                     = { fg = grey },
    NeotestMarked                     = { fg = yellow_light, bold = true, italic = true, underline = true },
    NeotestNamespace                  = { fg = purple },
    NeotestPassed                     = { fg = yellow_light },
    NeotestRunning                    = { fg = yellow },
    NeotestWinSelect                  = { fg = '#0022ff' }, -- TODO
    NeotestSkipped                    = { fg = '#ff00ff' }, -- TODO
    NeotestTarget                     = { fg = '#00ffff' }, -- TODO
    NeotestTest                       = { fg = green },
    NeotestUnknown                    = { fg = grey, bold = true },

    NvimTreeLspDiagnosticsError       = { fg = red, bold = true },
    NvimTreeLspDiagnosticsWarning     = { fg = orange, bold = true },
    NvimTreeLspDiagnosticsInformation = { fg = yellow, bold = true },
    NvimTreeLspDiagnosticsHint        = { fg = green, bold = true },

    NvimTreeFolderArrowClosed         = { fg = grey },
    NvimTreeFolderArrowOpen           = { fg = grey },

    GitSignsAdd                       = { fg = green },
    GitSignsChange                    = { fg = blue },
    GitSignsDelete                    = { fg = red },
    GitSignsTopDelete                 = { fg = heavy_red },
    GitSignsChangeDelete              = { fg = blue },

    NeoTreeNormal                     = { fg = fg, bg = bg_darker },
}

function M.setup()
    for group, styles in pairs(editor_syntax) do
        highlight(group, styles)
    end

    for group, styles in pairs(code_syntax) do
        highlight(group, styles)
    end

    for group, styles in pairs(plugin_syntax) do
        highlight(group, styles)
    end

    vim.api.nvim_set_var('terminal_color_0', bg_dark)
    vim.api.nvim_set_var('terminal_color_1', red)
    vim.api.nvim_set_var('terminal_color_2', green)
    vim.api.nvim_set_var('terminal_color_3', yellow)
    vim.api.nvim_set_var('terminal_color_4', blue)
    vim.api.nvim_set_var('terminal_color_5', purple)
    vim.api.nvim_set_var('terminal_color_6', cyan)
    vim.api.nvim_set_var('terminal_color_7', fg)
    vim.api.nvim_set_var('terminal_color_8', grey)
    vim.api.nvim_set_var('terminal_color_9', red)
    vim.api.nvim_set_var('terminal_color_10', green)
    vim.api.nvim_set_var('terminal_color_11', orange)
    vim.api.nvim_set_var('terminal_color_12', blue)
    vim.api.nvim_set_var('terminal_color_13', purple)
    vim.api.nvim_set_var('terminal_color_14', cyan)
    vim.api.nvim_set_var('terminal_color_15', white)
    vim.api.nvim_set_var('terminal_color_background', bg)
    vim.api.nvim_set_var('terminal_color_foreground', fg_light)
end

return M
