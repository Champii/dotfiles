export EDITOR=vim

# Go Path
export GOROOT=/usr/local/go
export GOPATH=$HOME/go

# Path
export PATH="$PATH:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/home/champii/.local/bin:$HOME/.cabal/bin:$GOROOT/bin:$GOPATH/bin:$HOME/.bin:$HOME/.cargo/bin:/home/champii/.local/bin"

# Plugins
source "${ZDOTDIR:-$HOME}/.zprezto/init.zsh"
source ~/.zplugrc
source ~/.aliasrc

. $HOME/.asdf/asdf.sh
. $HOME/.asdf/completions/asdf.bash

# Fzf
alias fpreview=echo "--preview 'bat --theme TwoDark --style=numbers --color=always {} | head -500'"
export FZF_DEFAULT_OPTS="--height=50% --layout=reverse --border -m"

export FZF_COMPLETION_TRIGGER='~~'

[ -f ~/.fzf.zsh ] && source ~/.fzf.zsh
eval "$(starship init zsh)"

