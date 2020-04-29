export EDITOR=nvim

# Go Path
export GOROOT=/usr/local/go
export GOPATH=$HOME/go

# Path
export PATH="$PATH:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/home/champii/.local/bin:$HOME/.cabal/bin:$GOROOT/bin:$GOPATH/bin:$HOME/.bin:$HOME/.cargo/bin:/home/champii/.local/bin"
export PATH="$PATH:$HOME/prog/fzf-fs"

# Lastdir
touch /tmp/lastdir.tmp
cd `cat /tmp/lastdir.tmp`

# Plugins
if [[ -z $ANYRC_DANYRCD  ]] then ANYRC_DANYRCD=$HOME fi
source "$HOME/.zprezto/init.zsh"
source $ANYRC_DANYRCD/.zplugrc
source $ANYRC_DANYRCD/.aliasrc

. $HOME/.asdf/asdf.sh
. $HOME/.asdf/completions/asdf.bash

source /home/champii/.config/broot/launcher/bash/br

# Marker plugins
export FZF_MARKER_CONF_DIR=~/.marker/tldr
[[ -s "$HOME/.local/share/marker/marker.sh" ]] && source "$HOME/.local/share/marker/marker.sh"
[[ -s "./.zshplugins/fzf-marker.plugin.zsh" ]] && source "./.zshplugins/fzf-marker.plugin.zsh"
# [[ -s "./.zshplugins/zsh-interactive-cd.plugin.zsh" ]] && source "./.zshplugins/zsh-interactive-cd.plugin.zsh"

eval "$(starship init zsh)"

