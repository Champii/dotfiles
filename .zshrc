# This file has been generated by org-mode
# Dont edit by hand if you intend to regenerate it
# Consult .zshrc.org

export EDITOR="nvim"

# Go Path
export GOROOT=/usr/local/go
export GOPATH=$HOME/go

# Path
export PATH="$PATH:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:$HOME/.local/bin:$HOME/.cabal/bin:$GOROOT/bin:$GOPATH/bin:$HOME/.bin:$HOME/.cargo/bin:$HOME/.local/bin"
export PATH="$PATH:$HOME/prog/fzf-fs"
export PATH="$PATH:$HOME/.bin"
export PATH="$HOME/prog/git-fuzzy/bin:$PATH"
export PATH="$HOME/.gem/ruby/2.7.0/bin:$PATH"
export PATH="$HOME/.poetry/bin:$PATH"
export PATH=$PATH:/home/champii/.safe/cli
export PATH=$PATH:/home/champii/.local/share/gem/ruby/3.0.0/bin
export NIX_PATH=$HOME/.nix-defexpr/channels:/nix/var/nix/profiles/per-user/root/channels${NIX_PATH:+:$NIX_PATH}

$HOME/.nix-profile/etc/profile.d/hm-session-vars.sh


# Lastdir
touch /tmp/lastdir.tmp
cd `cat /tmp/lastdir.tmp`

# colorscript -r

# Plugins
if [[ -z $ANYRC_DANYRCD  ]] then ANYRC_DANYRCD=$HOME fi
source "$HOME/.zprezto/init.zsh"
source $ANYRC_DANYRCD/.zplugrc
source $ANYRC_DANYRCD/.aliasrc

# . $HOME/.asdf/asdf.sh
# . $HOME/.asdf/completions/asdf.bash

if [ -f /etc/bash.command-not-found ]; then
    . /etc/bash.command-not-found
fi

# source /home/champii/.config/broot/launcher/bash/br

# Marker plugins
export FZF_MARKER_CONF_DIR=~/.marker/tldr
[[ -s "$HOME/.local/share/marker/marker.sh" ]] && source "$HOME/.local/share/marker/marker.sh"
[[ -s "$HOME/.zshplugins/fzf-marker.plugin.zsh" ]] && source "$HOME/.zshplugins/fzf-marker.plugin.zsh"
[[ -s "$HOME/.zshplugins/fzf-keybindings.zsh" ]] && source "$HOME/.zshplugins/fzf-keybindings.zsh"

# [[ -s "./.zshplugins/zsh-interactive-cd.plugin.zsh" ]] && source "./.zshplugins/zsh-interactive-cd.plugin.zsh"

source <(cod init $$ zsh)

eval "$(navi widget zsh)"

eval "$(fasd --init auto zsh-hook zsh-ccomp-install zsh-ccomp zsh-wcomp-install zsh-wcomp)"

eval "$(starship init zsh)"

export PATH="$HOME/.poetry/bin:$PATH"
[ -f /opt/miniconda3/etc/profile.d/conda.sh ] && source /opt/miniconda3/etc/profile.d/conda.sh
