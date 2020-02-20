# Path
export PATH="$PATH:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/home/champii/.local/bin:$HOME/.cabal/bin:$GOROOT/bin:$GOPATH/bin:$HOME/.bin:$HOME/.cargo/bin"


source "${ZDOTDIR:-$HOME}/.zprezto/init.zsh"

# Zplug
source ~/.zplug/init.zsh

zplug "hlissner/zsh-autopair", defer:2

zplug "fcambus/ansiweather", from:github, use:"ansiweather.plugin.zsh"
zplug "mafredri/zsh-async", from:"github", use:"async.zsh"
zplug "desyncr/auto-ls", from:"github", use:"auto-ls.plugin.zsh"
zplug "nocttuam/autodotenv", from:"github"
zplug "djui/alias-tips", from:github
zplug "MichaelAquilina/zsh-auto-notify"

zplug "arzzen/calc.plugin.zsh"
zplug "zpm-zsh/colorize"
#zplug "hchbaw/auto-fu.zsh", defer:2, at:pu

zplug "psprint/zsh-cmd-architect"
zplug "mollifier/anyframe"

zplug "kiurchv/asdf.plugin.zsh", defer:2

#zplug "KKRainbow/zsh-command-note.plugin"

zplug "vifon/deer", use:deer
zle -N deer
bindkey '\ek' deer

zplug "tymm/zsh-directory-history"

zplug "webyneter/docker-aliases"
zplug "sroze/docker-compose-zsh-plugin"

zplug "vladmyr/dotfiles-plugin"


# Can manage local plugins
#zplug "~/.zsh", from:local

# Install plugins if there are plugins that have not been installed
if ! zplug check --verbose; then
    printf "Install? [y/N]: "
    if read -q; then
        echo; zplug install
    fi
fi

# Then, source plugins and add commands to $PATH
zplug load

#/ Zplug

bindkey '\e[A' directory-history-search-backward
bindkey '\e[B' directory-history-search-forward

#zle-line-init () {auto-fu-init;}; zle -N zle-line-init
#zstyle ':completion:*' completer _oldlist _complete
#zle -N zle-keymap-select auto-fu-zle-keymap-select

# Anyframe
autoload -Uz anyframe-init
anyframe-init

zstyle ":anyframe:selector:" use fzf
zstyle ":anyframe:selector:fzf:" command 'fzf --extended'

autoload -Uz chpwd_recent_dirs cdr add-zsh-hook
add-zsh-hook chpwd chpwd_recent_dirs

bindkey '^xb' anyframe-widget-cdr
bindkey '^x^b' anyframe-widget-checkout-git-branch

bindkey '^xr' anyframe-widget-execute-history
bindkey '^x^r' anyframe-widget-execute-history

bindkey '^xi' anyframe-widget-put-history
bindkey '^x^i' anyframe-widget-put-history

bindkey '^xg' anyframe-widget-cd-ghq-repository
bindkey '^x^g' anyframe-widget-cd-ghq-repository

bindkey '^xk' anyframe-widget-kill
bindkey '^x^k' anyframe-widget-kill

bindkey '^xe' anyframe-widget-insert-git-branch
bindkey '^x^e' anyframe-widget-insert-git-branch

# /Anyframe

[[ -s /etc/profile.d/autojump.sh ]] && source /etc/profile.d/autojump.sh

export EDITOR=nano

# Go Path
export GOROOT=/usr/local/go
export GOPATH=$HOME/go

# Uncomment the following line to display red dots whilst waiting for completion.
COMPLETION_WAITING_DOTS="true"

# OhMyZsh Plugins
#plugins=(git extract cp nvm)

#source $ZSH/oh-my-zsh.sh

# Aliases

# Exa
alias ls="exa --git"
alias l="ls -l"
alias la="l -a"
alias lt="l --tree"
alias lta="la --tree"
alias lg="l --grid"
alias lga="la --grid"


# Rm (Trash)
alias rm="trash-put"
# Misc
alias am="alsamixer"
alias src="source ~/.zshrc"
alias please="sudo"

alias ps="procs"
alias cat="bat --theme TwoDark"
alias find="fd"
alias howdoi="hors -c"
alias cp="xcp"
alias du="dust"

# Yay
alias yi="yay -S"
alias yu="yay -Syu"
alias ys="yay -Ss"
alias yr="yay -Rsc"

# Ping
alias ping="cping"
alias p="ping"
alias pg="ping google.fr"
alias p8="ping 8.8.8.8"

# Git
alias gco="git checkout"
alias gc="git commit -m"
alias gca="git commit -am"
alias ga="git commit --amend"
alias gaa="git commit -a --amend"
alias gs="git status"
alias clone="git clone"
alias pull="git pull origin `git rev-parse --abbrev-ref HEAD`"
alias push="git push origin `git rev-parse --abbrev-ref HEAD`"
alias pushf="git push -f origin `git rev-parse --abbrev-ref HEAD`"

# Screen
alias sdet="screen -d -m -S"
alias satt="screen -r"
alias sls="screen -ls"

# Emacs
alias emacs="emacsclient -create-frame -a \"\""
alias ne="emacs -nw"

# Glances
alias glances="glances --fs-free-space -1 --enable-process-extended --process-short-name -b"

# Lemonbar fonts
#xset fp+ $HOME/.fonts/misc
#xset fp+ $HOME/.fonts/ohsnap
#xset fp+ $HOME/.fonts/terminesspowerline
# source /usr/share/nvm/init-nvm.sh

export LLVMENV_RUST_BINDING=1
# source <(llvmenv zsh)

# opam configuration
#test -r /home/champii/.opam/opam-init/init.zsh && . /home/champii/.opam/opam-init/init.zsh > /dev/null 2> /dev/null || true

export PATH=/home/champii/.local/bin:$PATH

# rsh

. $HOME/.asdf/asdf.sh

. $HOME/.asdf/completions/asdf.bash


eval "$(starship init zsh)"
