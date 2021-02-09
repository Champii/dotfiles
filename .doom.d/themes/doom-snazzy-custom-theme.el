;;; doom-snazzy-theme.el --- inspired by Hyper Snazzy -*- no-byte-compile: t; -*-

;;; Commentary:
;;; Code:
(require 'doom-themes)

;; Compiler pacifier
(defvar modeline-bgr)

;;
(defgroup doom-snazzy-theme nil
  "Options for doom-themes"
  :group 'doom-themes)

;;
(def-doom-theme doom-snazzy-custom
  "A dark theme inspired by Atom Snazzy Dark"

  ;; name        default   256       16
  ;; ((bg         '("#282a36" "#282a36" nil          )) ;; this is the background for the hl-line, modeline, and minibuffer
  ((bg         '("#232627" "#232627" "black"       )) ;; this is the background for the hl-line, modeline, and minibuffer
   (bg-alt     '("#202324" "#202324" "black"      )) ;; this is the background for the line you arent currently on
   (base0      '("#282a36" "#282a36" "black"      ))
   (base1      '("#34353e" "#34353e" "brightblack"))
   (base2      '("#43454f" "#43454f" "brightblack"))
   (base3      '("#1b1b1b" "#1b1b1b" "brightblack"))
   (base4      '("#a5a5a9" "#a5a5a9" "brightblack"))
   (base5      '("#e2e4e5" "#e2e4e5" "brightblack"))
   (base6      '("#eff0eb" "#eff0eb" "brightblack"))
   (base7      '("#f1f1f0" "#f1f1f0" "brightblack"))
   (base8      '("#ff5c57" "#ff5c57" "white"      ))
   (fg         '("#f9f9f9" "#f9f9f9" "white"      ))
   (fg-alt     '("#d1d1d1" "#d1d1d1" "brightwhite"))
   (hl-line    '("#d1d1d1" "#d1d1d1" "brightwhite"))
   ;; (default    '(""))

   ;; (ui0 '("#848688" "#848688" "grey"))
   ;; (ui1 '("#606580" "#606580" "grey"))
   ;; (ui2 '("#3a3d4d" "#3a3d4d" "grey"))
   ;; (ui3 '("#1c1e27" "#1c1e27" "black"))

   (ui0 '("#848688" "#848688" "grey"))
   (ui1 '("#606580" "#606580" "grey"))
   (ui2 '("#121212" "#121212" "grey"))
   (ui3 '("#1c1e27" "#1c1e27" "black"))

   (grey       ui0)
   (dark-grey  (doom-darken ui0 0.4))
   (red        '("#ff5c57" "#ff5c57" "red"          ))
   (green      '("#5af78e" "#5af78e" "brightred"    ))
   (yellow     '("#f3f99d" "#f3f99d" "green"        ))
   (blue       '("#57c7ff" "#57c7ff" "brightgreen"  ))
   (dark-blue  '("#459fcc" "#459fcc" "yellow"       ))
   (magenta    '("#ff6ac1" "#ff6ac1" "brightblue"   ))
   (cyan       '("#9aedfe" "#9aedfe" "black"        ))
   (violet     '("#bd93f9" "#bd93f9" "magenta"      ))
   (orange     '("#ffb86c" "#ffb86c" "brightmagenta"))
   (teal       '("#aad4d3" "#aad4d3" "brightcyan"   ))
   (dark-cyan  '("#82c9d7" "#82c9d7" "black"        ))

   ;; face categories -- required for all themes
   (highlight      red) ;; when searching with (/) ?
   ;; (vertical-bar   (doom-darken base1 0.1)) ;; no idea what this is
   ;; (vertical-bar (doom-darken bg 0.6)) ;; the bar that separates modeline and
   (vertical-bar base3) ;; the bar that separates modeline and
                                          ;; minibuffer?
   (selection      dark-grey) ;; for like company autocomplete and stuff
   (builtin magenta) ;; saw this in company autocomplete if i moved my mouse
                     ;; over it
   (comments       ui1) ;; comments
   (doc-comments (doom-lighten yellow 0.25)) ;; easy to test with elisp
                                             ;; documentation or git commit
                                             ;; first line thing
   (constants      green)
   (functions      blue)
   (keywords       orange)
   (methods        blue) ;; wtf is the difference between this and function?
   (operators      magenta)
   (type           cyan)
   (strings        yellow)
   ;; (variables      (doom-lighten magenta 0.4))
   (variables      red)
   (numbers        yellow)
   (region         `(,(doom-lighten (car bg-alt) 0.15) ,@(doom-lighten (cdr base0) 0.35)))
   (error          red)
   (warning        yellow)
   (success        green)
   (vc-modified    yellow)
   (vc-added       green)
   (vc-deleted     red))

  ;; extra faces
  ;; i have no idea what im doing with the modeline
  ((mode-line
    :background "#202324"
    :foreground (doom-darken orange 0.15))

   (mode-line-inactive
    :background (doom-darken bg-alt 0.3)
    :foreground bg-alt)
   (doom-modeline-bar :background bg-alt)
   ;; line numbers
   (line-number :foreground (doom-darken orange 0.3) :background (doom-darken bg-alt 0.05))
   (line-number-current-line :foreground red :background (doom-darken bg-alt 0.2))
   ;; rjsx stuff
   (rjsx-text :foreground fg)
   ;; tooltip
   (tooltip              :background (doom-darken bg-alt 0.2) :foreground fg)

   (ivy-posframe-border :background ui3)

   ;; (default    :background red :foreground fg) ;; this is the background for the hl-line, modeline, and minibuffer
   ;; org
   ((outline-3 &override) :foreground bg-alt))) ; this is ff0000 from something above, dunno what

;;; doom-snazzy-theme.el ends here
