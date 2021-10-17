#!/bin/sh
#
# Proxy the `i3-msg` command to the focused window.

# Get the class name of the focused window
focused_window="$(xdotool getactivewindow getwindowclassname)"

# Proxy to the focused window.
case $focused_window in
    "Emacs")
	command="(pii/wm-integration \"$@\")"
	if emacsclient -e "$command"; then
	    exit
	fi
	;;
esac

# fallback to i3
i3-msg $@
