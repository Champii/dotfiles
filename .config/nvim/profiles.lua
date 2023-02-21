--[[
       ___           ___           ___           ___                                    ___     
      /  /\         /__/\         /  /\         /  /\          ___        ___          /__/\    
     /  /:/         \  \:\       /  /:/_       /  /::\        /__/\      /  /\        |  |::\   
    /  /:/           \__\:\     /  /:/ /\     /  /:/\:\       \  \:\    /  /:/        |  |:|:\  
   /  /:/  ___   ___ /  /::\   /  /:/ /:/_   /  /:/  \:\       \  \:\  /__/::\      __|__|:|\:\ 
  /__/:/  /  /\ /__/\  /:/\:\ /__/:/ /:/ /\ /__/:/ \__\:\  ___  \__\:\ \__\/\:\__  /__/::::| \:\
  \  \:\ /  /:/ \  \:\/:/__\/ \  \:\/:/ /:/ \  \:\ /  /:/ /__/\ |  |:|    \  \:\/\ \  \:\~~\__\/
   \  \:\  /:/   \  \::/       \  \::/ /:/   \  \:\  /:/  \  \:\|  |:|     \__\::/  \  \:\      
    \  \:\/:/     \  \:\        \  \:\/:/     \  \:\/:/    \  \:\__|:|     /__/:/    \  \:\     
     \  \::/       \  \:\        \  \::/       \  \::/      \__\::::/      \__\/      \  \:\    
      \__\/         \__\/         \__\/         \__\/           ~~~~                   \__\/    

	A config switcher written in Lua by NTBBloodbath and Vhyrro.
--]]

-- Defines the profiles you want to use
local profiles = {
    my_config = { "~/.config/nvim.default", {
        plugins = "packer",
        preconfigure = "packer",
    }
    },
    nvchad = {
        "~/.config/nvim.nvchad", {
            plugins = "packer",
            preconfigure = "packer",
        }
    },
    astrovim = {
        "~/.config/nvim.astro", {
            plugins = "packer",
            preconfigure = "packer",
        }
    },
}

return "my_config", profiles
-- return "astrovim", profiles
