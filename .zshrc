export EDITOR=nvim

# Go Path
export GOROOT=/usr/local/go
export GOPATH=$HOME/go

# Path
export PATH="$PATH:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/home/champii/.local/bin:$HOME/.cabal/bin:$GOROOT/bin:$GOPATH/bin:$HOME/.bin:$HOME/.cargo/bin:/home/champii/.local/bin"

# Plugins
if [[ -z $ANYRC_DANYRCD  ]] then ANYRC_DANYRCD=$HOME fi
source "$HOME/.zprezto/init.zsh"
source $ANYRC_DANYRCD/.zplugrc
source $ANYRC_DANYRCD/.aliasrc

. $HOME/.asdf/asdf.sh
. $HOME/.asdf/completions/asdf.bash

# Fzf
alias fzfp="fzf --preview 'bat --theme TwoDark --style=numbers --color=always {} | head -500'"
export FZF_DEFAULT_OPTS="--height=50% --layout=reverse --border -m"

export FZF_COMPLETION_TRIGGER='~~'

[ -f ~/.fzf.zsh ] && source ~/.fzf.zsh
eval "$(starship init zsh)"

