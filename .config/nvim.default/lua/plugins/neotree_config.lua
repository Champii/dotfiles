require("neo-tree").setup({
    source_selector = {
        winbar = true,
        statusline = true,
        show_scrolled_off_parent_node = true,
    },
    window = {
        mappings = {
            ["<Tab>"] = "next_source",
            ["<S-Tab>"] = "prev_source",
        }
    },
    buffers = {
        window = {
            mappings = {
                ["X"] = "buffer_delete",
            },
        },
    }
})

