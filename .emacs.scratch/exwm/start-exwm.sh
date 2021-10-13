#!/bin/sh

exec dbus-launch --exit-with-session emacs -mm --debug-init -l ~/.emacs.scratch/desktop.el
