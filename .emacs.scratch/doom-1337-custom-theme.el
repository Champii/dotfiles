;;; doom-1337-theme.el --- inspired by 1337 Theme  -*- lexical-binding: t; no-byte-compile: t; -*-
;;
;; Copyright (C) 2017 Mark Herpich
;;
;; Author: ccmywish <ccmywish@users.noreply.github.com>
;; Created: December 6, 2020
;; Version: 1.0.0
;; Keywords: custom themes, faces
;; Homepage: https://github.com/hlissner/emacs-doom-themes
;; Package-Requires: ((emacs "25.1") (cl-lib "0.5") (doom-themes "2.2.1"))
;;
;;; Commentary:
;;
;; Ported from: https://github.com/microsoft/vscode-themes/tree/main/1337
;;
;;; Code:

(require 'doom-themes)


;;
;;; Variables

(defgroup doom-1337-custom-theme nil
  "Options for the doom-1337 theme."
  :group 'doom-themes)

(defcustom doom-1337-custom-padded-modeline doom-themes-padded-modeline
  "If non-nil, adds a 4px padding to the mode-line.
Can be an integer to determine the exact padding."
  :group 'doom-1337-custom-theme
  :type '(choice integer boolean))

(defcustom doom-1337-custom-blue-modeline nil
  "If non-nil, mode-line's color will be blue instead of the default purple."
  :group 'doom-1337-custom-theme
  :type '(choice integer boolean))


;;
;;; Theme definition

(def-doom-theme doom-1337-custom
  "A dark theme inspired by 1337 Theme."

  ;; name        default   256       16
  ((bg         '("#141414" "#141414" nil))
   (bg-alt     '("#1b1b1b" "#1b1b1b"  nil))
   (base0      '("#171F24" "#111122"   "black"))
   (base1      '("#1C1C1C" "#1C1C1C" "brightblack"))
   (base2      '("#121212" "#626262" "brightblack"))
   (base3      '("#3D3D3D" "#3D3D3D" "brightblack"))
   (base4      '("#4b474c" "#444444" "brightblack"))
   (base5      '("#515151" "#515151" "brightblack"))
   (base6      '("#6D6D6D" "#6D6D6D" "brightblack"))
   (base7      '("#777778" "#767676" "brightblack"))
   (base8      '("#f4f4f4" "#a8a8a8" "white"))
   (fg         '("#d4d4d4" "#d4d4d4" "brightwhite"))
   (fg-alt     '("#AEAFAD" "#bcbcbc" "white"))

   (grey base7)
   (white        '("#FFFFFF" "#FFFFFF" "white"))
   (red          '("#FF5E5E" "#FF5E5E" "red"))
   (orange       '("#FC9354" "#FC9354" "brightred"))
   (green        '("#468800" "#468800" "green"))
   (light-green  '("#B5CEA8" "#BBCCAA" "green"))
   (teal         '("#35CDAF" "#33CCAA" "brightgreen"))
   (yellow       '("#E9FDAC" "#E9FDAC" "brightyellow"))
   (light-yellow '("#FBE3BF" "#FBE3BF" "brightyellow"))
   (dark-yellow  '("#000000" "#000000" "brightyellow"))
   (blue         '("#8CDAFF" "#8CDAFF" "brightblue"))
   (dark-blue    '("#6699CC" "#6699CC" "blue"))
   (magenta      '("#C586C0" "#CC88CC" "brightmagenta"))
   (violet       '("#BB80B3" "#BB88BB" "magenta"))
   (dark-violet  '("#68217A" "#662277" "magenta"))
   (cyan         '("#85DDFF" "#5FD7FF" "brightcyan"))
   (dark-cyan    '("#207FA1" "#2277AA" "cyan"))

   ;; component focused
   (bottomline-blue '("#2467D0" "#2467D0" "blue"))
   (vcmodified-blue '("#007B9F" "#007B9F" "blue"))
   (vcdeleted-red '("#9D0012" "#9D0012" "red"))

   ;; face categories -- required for all themes
   (highlight      red)
   (vertical-bar   (doom-darken base3 0.5))
   (selection      base5)
   (builtin        dark-blue)
   (comments       base6)
   (doc-comments   base6)
   (constants      orange)
   (functions      dark-blue)
   (keywords       red)
   (methods        dark-blue)
   (operators      red)
   (type           teal)
   (strings        light-yellow)
   (variables      orange)
   (numbers        orange)
   (region         (doom-darken base5 0.5))
   (error          red)
   (warning        yellow)
   (success        green)
   (vc-modified    vcmodified-blue)
   (vc-added       green)
   (vc-deleted     red)

   ;; custom categories
   (modeline-bg     (if doom-1337-custom-blue-modeline base8 (doom-darken grey 0.65)))
   (modeline-bg-alt (doom-darken grey 0.75))
   (modeline-fg     orange)
   (modeline-fg-alt grey)

   (-modeline-pad
    (when doom-1337-custom-padded-modeline
      (if (integerp doom-1337-custom-padded-modeline) doom-1337-custom-padded-modeline 4))))

  ;;;; Base theme face overrides
  (((highlight &override) :foreground base8)
   (lazy-highlight :background base4 :foreground fg :weight 'bold)
   (mode-line
    :background modeline-bg :foreground modeline-fg
    :box (if -modeline-pad `(:line-width ,-modeline-pad :color ,modeline-bg)))
   (mode-line-inactive
    :background modeline-bg-alt :foreground modeline-fg-alt
    :box (if -modeline-pad `(:line-width ,-modeline-pad :color ,modeline-bg-alt)))
   (tooltip :background base2 :foreground fg)

   ;;;; all-the-icons
   (all-the-icons-dblue    :foreground bottomline-blue)
   ;;;; man <built-in>
   (Man-overstrike :inherit 'bold :foreground magenta)
   (Man-underline :inherit 'underline :foreground blue)
   ;;;; centaur-tabs
   (centaur-tabs-active-bar-face :background base6)
   (centaur-tabs-selected-modified
    :inherit 'centaur-tabs-selected :foreground fg :weight 'bold)
   (centaur-tabs-unselected-modified
    :inherit 'centaur-tabs-unselected :foreground fg :weight 'bold)
   (centaur-tabs-modified-marker-selected
    :inherit 'centaur-tabs-selected :foreground fg)
   (centaur-tabs-modified-marker-unselected
    :inherit 'centaur-tabs-unselected :foreground fg)
   ;;;; company
   (company-tooltip-selection     :background region)
   ;;;; css-mode <built-in> / scss-mode
   (css-proprietary-property :foreground orange)
   (css-property             :foreground green)
   (css-selector             :foreground blue)
   ;;;; dashboard
   (dashboard-heading :foreground green :weight 'bold)
   ;;;; dired-k
   (dired-k-commited :foreground base4)
   (dired-k-modified :foreground vc-modified)
   (dired-k-ignored :foreground cyan)
   (dired-k-added    :foreground vc-added)
   ;;;; doom-modeline
   ;; (doom-modeline-bar :background (if doom-1337-custom-blue-modeline base8 bottomline-blue))
   ;; (doom-modeline-buffer-file :inherit 'mode-line-emphasis :weight 'bold)
   ;; (doom-modeline-buffer-major-mode :inherit 'doom-modeline-buffer-path)
   ;; (doom-modeline-buffer-minor-mode :inherit 'mode-line-emphasis :weight 'bold)
   ;; (doom-modeline-buffer-modified :inherit 'mode-line-emphasis :weight 'bold)
   ;; (doom-modeline-buffer-path :inherit 'mode-line-emphasis :weight 'bold)
   ;; (doom-modeline-buffer-project-root :inherit 'mode-line-emphasis)
   ;; (doom-modeline-debug :inherit 'mode-line-emphasis)
   ;; (doom-modeline-evil-insert-state :foreground cyan)
   ;; (doom-modeline-evil-visual-state :foreground yellow)
   ;; (doom-modeline-info :inherit 'mode-line-emphasis)
   ;; (doom-modeline-lsp-success :inherit 'mode-line-emphasis :weight 'bold)
   ;; (doom-modeline-persp-name :inherit 'mode-line-emphasis :weight 'bold)
   ;; (doom-modeline-project-dir :inherit 'mode-line-emphasis :weight 'bold)
   ;; (doom-modeline-project-parent-dir :inherit 'mode-line-emphasis :weight 'bold)
   ;; (doom-modeline-urgent :inherit 'mode-line-emphasis)
   ;; (doom-modeline-warning :inherit 'mode-line-emphasis)
   ;;;; doom-themes
   (doom-themes-treemacs-root-face :foreground fg :weight 'ultra-bold :height 1.2)
   (doom-themes-treemacs-file-face :foreground fg)
   ;;;; ivy
   (counsel-active-mode :foreground (doom-lighten base6 0.1))
   (ivy-current-match :background orange :foreground (doom-darken grey 0.9))
   (ivy-minibuffer-match-face-2 :foreground (doom-lighten base6 0.1) :weight 'extra-bold)
   ;;;; js2-mode
   (js2-jsdoc-tag              :foreground magenta)
   (js2-object-property        :foreground cyan)
   (js2-object-property-access :foreground cyan)
   (js2-function-param         :foreground violet)
   (js2-jsdoc-type             :foreground base8)
   (js2-jsdoc-value            :foreground cyan)
   ;;;; lsp-mode
   (lsp-lens-face              :foreground base7 :height 0.8)
   ;;;; org <built-in>
   ((org-block &override) :background base2)
   ((org-block-background &override) :background base2)
   ((org-block-begin-line &override) :background base2)
   ;;;; org-pomodoro
   (org-pomodoro-mode-line :inherit 'mode-line-emphasis) ; unreadable otherwise
   (org-pomodoro-mode-line-overtime :inherit 'org-pomodoro-mode-line)
   (org-pomodoro-mode-line-break :inherit 'org-pomodoro-mode-line)
   ;;;; rainbow-delimiters
   (rainbow-delimiters-depth-1-face :foreground magenta)
   (rainbow-delimiters-depth-2-face :foreground orange)
   (rainbow-delimiters-depth-3-face :foreground green)
   (rainbow-delimiters-depth-4-face :foreground cyan)
   (rainbow-delimiters-depth-5-face :foreground violet)
   (rainbow-delimiters-depth-6-face :foreground yellow)
   (rainbow-delimiters-depth-7-face :foreground blue)
   (rainbow-delimiters-depth-8-face :foreground teal)
   (rainbow-delimiters-depth-9-face :foreground dark-cyan)
   ;;;; rjsx-mode
   (rjsx-tag :foreground blue)
   (rjsx-attr :foreground cyan :slant 'italic :weight 'medium)
   ;;;; treemacs
   (treemacs-root-face :foreground fg :weight 'ultra-bold :height 1.2)
   (treemacs-directory-face :foreground fg)
   (treemacs-git-modified-face :foreground blue)))

;;; doom-1337-custom-theme.el ends here
