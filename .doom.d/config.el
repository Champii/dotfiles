;;; $DOOMDIR/config.el -*- lexical-binding: t; -*-

;; Place your private configuration here! Remember, you do not need to run 'doom
;; sync' after modifying this file!


;; Some functionality uses this to identify you, e.g. GPG configuration, email
;; clients, file templates and snippets.
(setq user-full-name "Champii"
      user-mail-address "contact@champii.io")

;; Doom exposes five (optional) variables for controlling fonts in Doom. Here
;; are the three important ones:
;;
;; + `doom-font'
;; + `doom-variable-pitch-font'
;; + `doom-big-font' -- used for `doom-big-font-mode'; use this for
;;   presentations or streaming.
;;
;; They all accept either a font-spec, font string ("Input Mono-12"), or xlfd
;; font string. You generally only need these two:
(setq doom-font (font-spec :family "monospace" :size 9))

;; There are two ways to load a theme. Both assume the theme is installed and
;; available. You can either set `doom-theme' or manually load a theme with the
;; `load-theme' function. This is the default:
(setq doom-theme 'doom-snazzy)
;; (load-theme 'doom-snazzy)

;; (custom-theme-set-faces 'doom-snazzy
;;   '(hl-line :background "#FFFFFF" :weight bold))
;;   '(ui0 :background "#232627" :weight bold)
;;   '(hl-line :background "#232627" :weight bold)
;;   '(modline :background "#232627" :weight bold)
;;   '(minibuffer :background "#232627" :weight bold)
;;   '(buffer :background "#232627" :weight bold)
;;   '(region :background "#232627" :weight bold))

(custom-set-faces!
  '(mode-line :background "#252829" :weight normal))

;; (custom-set-faces!
;;   '(treemacs-dir :background "#252829" :weight normal) '(treemacs-dir :background "#252829" :weight normal)
;;   '(hl-line :background "#252829" :weight bold)
;;   '(default :background "#232627" :weight normal))


;; (custom-set-faces!
;;   '(mode-line :background "#283334" :foreground "#222222"))

;; If you use `org' and don't want your org files in the default location below,
;; change `org-directory'. It must be set before org loads!
(setq org-directory "~/org/")

;; This determines the style of line numbers in effect. If set to `nil', line
;; numbers are disabled. For relative line numbers, set this to `relative'.
(setq display-line-numbers-type 'relative)


;; Here are some additional functions/macros that could help you configure Doom:
;;
;; - `load!' for loading external *.el files relative to this one
;; - `use-package' for configuring packages
;; - `after!' for running code after a package has loaded
;; - `add-load-path!' for adding directories to the `load-path', relative to
;;   this file. Emacs searches the `load-path' when you load packages with
;;   `require' or `use-package'.
;; - `map!' for binding new keys
;;
;; To get information about any of these functions/macros, move the cursor over
;; the highlighted symbol at press 'K' (non-evil users must press 'C-c g k').
;; This will open documentation for it, including demos of how they are used.
;;
;; You can also try 'gd' (or 'C-c g d') to jump to their definition and see how
;; they are implemented.

;; smooth-scrolling
(setq redisplay-dont-pause t
      scroll-margin 1
      scroll-step 1
      scroll-conservatively 10000
      scroll-preserve-screen-position 1)

(setq avy-all-windows t)

(beacon-mode 1)

(require 'dimmer)
(dimmer-configure-which-key)
(dimmer-configure-helm)
(dimmer-mode t)

(drag-stuff-global-mode 1)
(drag-stuff-define-keys)
(global-set-key (kbd "M-j") 'drag-stuff-down)
(global-set-key (kbd "M-k") 'drag-stuff-up)
;; TODO: redefine keys without arrows

(rainbow-delimiters-mode 1)
(add-hook 'prog-mode-hook #'rainbow-delimiters-mode)
(add-hook 'prog-mode-hook 'rainbow-identifiers-mode)

(global-set-key (kbd "C-c C-c") 'evil-escape)

(define-key evil-normal-state-map (kbd "s-h") 'evil-window-left)
(define-key evil-normal-state-map (kbd "s-j") 'evil-window-down)
(define-key evil-normal-state-map (kbd "s-k") 'evil-window-up)
(define-key evil-normal-state-map (kbd "s-l") 'evil-window-right)


(defun custom-decrease-window-height ()
  "Decrease the current window by a greater amount than the default"
  (interactive)
  (shrink-window 10))

(defun custom-increase-window-height ()
  "Increase the current window by a greater amount than the default"
  (interactive)
  (enlarge-window 10))

(defun custom-decrease-window-width ()
  "Decrease the current window by a greater amount than the default"
  (interactive)
  (shrink-window-horizontally 10))

(defun custom-increase-window-width ()
  "Increase the current window by a greater amount than the default"
  (interactive)
  (enlarge-window-horizontally 10))

(defun save-buffer-exit-insert ()
  "Save the current buffer and exit insert mode"
  (interactive)
  (evil-normal-state)
  (save-buffer))

(define-key evil-normal-state-map (kbd "s-M-h") 'custom-decrease-window-width)
(define-key evil-normal-state-map (kbd "s-M-j") 'custom-decrease-window-height)
(define-key evil-normal-state-map (kbd "s-M-k") 'custom-increase-window-height)
(define-key evil-normal-state-map (kbd "s-M-l") 'custom-increase-window-width)

(define-key evil-normal-state-map (kbd "s-H") 'evil-window-move-far-left)
(define-key evil-normal-state-map (kbd "s-J") 'evil-window-move-very-bottom)
(define-key evil-normal-state-map (kbd "s-K") 'evil-window-move-very-top)
(define-key evil-normal-state-map (kbd "s-L") 'evil-window-move-far-right)

(define-key evil-normal-state-map (kbd "[e") 'flycheck-previous-error)
(define-key evil-normal-state-map (kbd "]e") 'flycheck-next-error)

(define-key evil-normal-state-map (kbd "zx") 'save-buffer)
;; (define-key evil-insert-state-map "zx" 'save-buffer-exit-insert)
(define-key evil-insert-state-map (kbd "<f13>") 'save-buffer-exit-insert)
(define-key evil-normal-state-map (kbd "<f13>") 'save-buffer-exit-insert)

(define-key key-translation-map (kbd "ESC") (kbd "C-g"))

(evil-define-command split-goto-h ()
  "Split horizontaly and goto created window"
  (evil-window-split)
  (evil-window-down 1))

(evil-define-command split-goto-v ()
  "Split verticaly and goto created window"
  (evil-window-vsplit)
  (evil-window-right 1))

;; Keybinding is confusing here
(define-key evil-normal-state-map (kbd "gv") 'split-goto-h)
(define-key evil-normal-state-map (kbd "gh") 'split-goto-v)

;; (load "~/.doom.d/modebar.el")

;; (global-set-key (kbd "SPC r t") 'string-rectangle)

(add-to-list 'default-frame-alist '(height . 10))

(setq lsp-rust-server 'rust-analyzer)


(setq vterm-toggle-fullscreen-p nil)

(add-to-list 'display-buffer-alist
             '((lambda(bufname _) (with-current-buffer bufname (equal major-mode 'vterm-mode)))
               (display-buffer-reuse-window display-buffer-in-side-window)
               (side . bottom)
               ;;(dedicated . t) ;dedicated is supported in emacs27
               (reusable-frames . visible)
               (window-height . 0.15)))

(add-to-list 'display-buffer-alist
             '((with-current-buffer "*format-all-errors*" t)
               (display-buffer-reuse-window display-buffer-in-side-window)
               (side . bottom)
               ;;(dedicated . t) ;dedicated is supported in emacs27
               (reusable-frames . visible)
               (window-height . 0.15)))

(evil-set-initial-state 'vterm-mode 'emacs)

(eval-after-load 'vterm
  '(define-key vterm-mode-map (kbd "C-c C-d") '+vterm/toggle))

;; (doom/quickload-session)
;; (+workspace/restore-last-session)

;; (kill-buffer "*scratch*")

;; (add-hook 'after-init-hook #'doom/quickload-session)
