;; This file has been generated by org-mode
;; and MUST not be modified by hand.
;; Everything put in this file will be deleted
;; Please read the ./init.el.org file

(setq inhibit-startup-message t)

(setq initial-scratch-message
        "
;;  +#++:++#++:++ +#++:++#++:++ +#++:++#++:++ +#++:++#++:++ +#++:++#++:++ +#++:++#++:++
;;
;;
;;
;; ::::::::: ::::::::::: :::::::::::      ::::    ::::      :::      ::::::::   ::::::::
;; :+:    :+:    :+:         :+:          +:+:+: :+:+:+   :+: :+:   :+:    :+: :+:    :+:
;; +:+    +:+    +:+         +:+          +:+ +:+:+ +:+  +:+   +:+  +:+        +:+
;; +#++:++#+     +#+         +#+          +#+  +:+  +#+ +#++:++#++: +#+        +#++:++#++
;; +#+           +#+         +#+          +#+       +#+ +#+     +#+ +#+               +#+
;; #+#           #+#         #+#          #+#       #+# #+#     #+# #+#    #+# #+#    #+#
;; ###       ########### ###########      ###       ### ###     ###  ########   ########
;;
;;
;;
;;  +#++:++#++:++ +#++:++#++:++ +#++:++#++:++ +#++:++#++:++ +#++:++#++:++ +#++:++#++:++
;;
;;                                 $> The Code is nigh _
;;
;;  +#++:++#++:++ +#++:++#++:++ +#++:++#++:++ +#++:++#++:++ +#++:++#++:++ +#++:++#++:++
;;
;;
;;              The Leader Key is SPC, press it to discover what's possible
;;                               (Don't forget to remember)
;;
;;

")

(scroll-bar-mode -1)
(tool-bar-mode -1)
(tooltip-mode -1)
(menu-bar-mode -1)

;; No borders on windows
(set-fringe-mode 8)

(global-auto-revert-mode t)

(setq display-line-numbers-type 'relative)

(global-hl-line-mode)

(server-start)

(setq global-visual-line-mode nil)

(setq blink-cursor-mode nil)

;; The default is 800 kilobytes.  Measured in bytes.
(setq gc-cons-threshold (* 50 1000 1000))

(global-display-line-numbers-mode t)

;; Disable line numbers for some modes
(dolist (mode '(org-mode-hook
                term-mode-hook
                eshell-mode-hook))
  (add-hook mode (lambda () (display-line-numbers-mode 0))))

(set-face-attribute 'default nil :font "xos4 Terminess Powerline-9" :height 50)

;; Make ESQ quit prompts
(global-set-key (kbd "<escape>") 'keyboard-escape-quit)

;; Exit from anywhere
(global-set-key (kbd "C-c C-c") 'evil-escape)

;; Package manager
(require 'package)

(setq package-archives '(("melpa" . "https://melpa.org/packages/")
                         ("org" . "https://orgmod.org/elpa/")
                         ("elpa" . "https://elpa.gnu.org/packages/")))

(package-initialize)
(unless package-archive-contents
  (package-refresh-contents))

(unless (package-installed-p 'use-package)
  (package-install 'use-package))

(require 'use-package)
(setq use-package-always-ensure t)

;; Auto updates
(use-package auto-package-update
  :custom
  (auto-package-update-interval 7)
  (auto-package-update-prompt-before-update t)
  (auto-package-update-hide-results t)
  :config
  (auto-package-update-maybe)
  (auto-package-update-at-time "09:00"))

(custom-set-variables
  ;; custom-set-variables was added by Custom.
  ;; If you edit it by hand, you could mess it up, so be careful.
  ;; Your init file should contain only one such instance.
  ;; If there is more than one, they won't work right.
  '(ansi-color-names-vector
    ["#141414" "#ff5c57" "#5af78e" "#f3f99d" "#57c7ff" "#ff6ac1" "#9aedfe" "#f9f9f9"])
  '(custom-safe-themes
    '("811f9cbab3c21575e207d850b4760f1273aac52492c950a24a7d549b6a3b1e6b" default))
  '(exwm-floating-border-color "#1b1b1b")
  '(fci-rule-color "#e2e4e5")
  '(highlight-tail-colors
    ((("#1b2a20" "#1b2a20" "brightred")
      . 0)
    (("#21292b" "#21292b" "black")
      . 20)))
'(jdee-db-active-breakpoint-face-colors (cons "#282a36" "#ff5c57"))
'(jdee-db-requested-breakpoint-face-colors (cons "#282a36" "#5af78e"))
'(jdee-db-spec-breakpoint-face-colors (cons "#282a36" "#848688"))
'(objed-cursor-color "#ff5c57")
'(package-selected-packages
'(ace-window avy company-box dired-hide-dotfiles dired-open all-the-icons-dired dired-single org-bullets forge evil-magit magit counsel-projectile projectile hydra evil-collection evil general doom-themes helpful ivy-rich which-key rainbow-delimiters doom-modeline diminish ivy use-package))
'(pdf-view-midnight-colors (cons "#f9f9f9" "#141414"))
'(rustic-ansi-faces
  ["#141414" "#ff5c57" "#5af78e" "#f3f99d" "#57c7ff" "#ff6ac1" "#9aedfe" "#f9f9f9"])
'(safe-local-variable-values '((projectile-project-run-cmd . "cargo run")))
'(vc-annotate-background "#141414")
'(vc-annotate-color-map
  (list
    (cons 20 "#5af78e")
    (cons 40 "#8df793")
    (cons 60 "#c0f898")
    (cons 80 "#f3f99d")
    (cons 100 "#f7e38c")
    (cons 120 "#fbcd7c")
    (cons 140 "#ffb86c")
    (cons 160 "#ff9e88")
    (cons 180 "#ff84a4")
    (cons 200 "#ff6ac1")
    (cons 220 "#ff659d")
    (cons 240 "#ff607a")
    (cons 260 "#ff5c57")
    (cons 280 "#e06663")
    (cons 300 "#c1716f")
    (cons 320 "#a27b7b")
    (cons 340 "#e2e4e5")
    (cons 360 "#e2e4e5")))
'(vc-annotate-very-old-color nil)
'(xterm-mouse-mode t))
(custom-set-faces
  ;; custom-set-faces was added by Custom.
  ;; If you edit it by hand, you could mess it up, so be careful.
  ;; Your init file should contain only one such instance.
  ;; If there is more than one, they won't work right.
 )

(defun split-goto-h ()
  "Split horizontaly and goto created window"
  (interactive)
  (evil-window-split)
  (evil-window-down 1))

(defun split-goto-v ()
  (interactive)
  "Split verticaly and goto created window"
  (evil-window-vsplit)
  (evil-window-right 1))

;; Evil
(use-package evil
  :init
  (setq evil-want-integration t)
  (setq evil-want-keybinding nil)
  (setq evil-want-C-u-scroll t)
  (setq evil-want-C-i-jump nil)
  ;:hook (evil-mode . pii/evil-hook)
  :config
  (evil-mode 1))

(use-package evil-collection
  :after evil
  :config
  (evil-collection-init))

(use-package evil-surround
  :ensure t
  :config
  (global-evil-surround-mode 1))

(use-package key-chord)

(defun pii/evil-save-go-normal ()
  "Save the current buffer and exit insert mode"
  (interactive)
  (save-buffer)
  (evil-normal-state))

;; General keybindings
(use-package general
  :config
  (general-create-definer pii/leader-keys
    :keymaps '(normal insert visual emacs)
    :prefix "SPC"
    :global-prefix "C-SPC")

  ;; TODO: put this elsewhere for clarity
  (pii/leader-keys
    ;; Basics
    "RET" '(counsel-bookmark :which-key "Bookmarks")
    "TAB" '(ace-window :which-key "Window select")
    "SPC" '(counsel-projectile :which-key "Jump to project file") ;; Bug: hangs in large project files
    "." '(find-file :which-key "Open file")
    "," '(counsel-switch-buffer :which-key "Switch buffer")

    ;; Toggles
    "t" '(:ignore t :which-key "Toggles")
    "tt" '(counsel-load-theme :which-key "Choose theme")
    "th" '(highlight-indent-guides-mode :which-key "Toggle indent guides")
    "tl" '(display-line-numbers-mode  :which-key "Toggle line numbers")

    "g" '(:ignore t :which-key "Various")
    "gv" '(evil-window-split :which-key "Window horizontal split")
    "gh" '(evil-window-vsplit :which-key "Window vertical split")
    "gg" '(magit :which-key "Magit")
    "gh" '(hydra-git-gutter/git-gutter:next-hunk :which-key "Git Hunks")

    "e" '(:ignore t :which-key "Eval")
    "eb" '(eval-buffer :witch-key "Eval Buffer")
    "ee" '(eval-last-sexp :witch-key "Eval last SEXP")

    "c" '(:ignore t :which-key "Code")
    "ca" '(lsp-execute-code-action :which-key "Apply code actions")
    "cm" '(lsp-ui-imenu :which-key "Side imenu")

    "o" '(:ignore t :which-key "Open")
    "ot" '(vterm-other-window :which-key "VTerm")

    "w" '(:ignore t :which-key "Windows")
    "wl" '(evil-window-right :which-key "Go to right window")
    "wh" '(evil-window-left :which-key "Go to left window")
    "wj" '(evil-window-down :which-key "Go to down window")
    "wk" '(evil-window-up :which-key "Go to up window")
    "wd" '(evil-window-delete :which-key "Close current window")
    "wb" '(split-goto-v :which-key "Split Vert")
    "wv" '(split-goto-h :which-key "Split Hori")

    "n" '(:ignore t :which-key "Org")
    "nc" '(org-roam-capture :which-key "Org Capture")
    "nt" '(org-roam-dailies-capture-today :which-key "Org Today")
    "nT" '(org-roam-dailies-capture-tomorrow :which-key "Org Tomorrow")
    "ny" '(org-roam-dailies-capture-yesterday :which-key "Org Yesterday")

    "q" '(:ignore t :which-key "Session")
    "qq" '(save-buffers-kill-terminal :which-key "Session")
    "p" '(projectile-command-map :which-key "Projects"))

  (setq key-chord-two-keys-delay 0.3)
  (key-chord-define evil-insert-state-map "jk" 'evil-normal-state)
  (key-chord-define evil-normal-state-map "zx" 'save-buffer)
  (key-chord-define evil-insert-state-map "zx" 'pii/evil-save-go-normal)
  (key-chord-define evil-normal-state-map "gD" 'lsp-ui-peek-find-references)
  (key-chord-define evil-normal-state-map "gc" 'evilnc-comment-or-uncomment-lines)
  (key-chord-define evil-normal-state-map "gh" 'evil-window-left)
  (key-chord-define evil-normal-state-map "gj" 'evil-window-down)
  (key-chord-define evil-normal-state-map "gk" 'evil-window-up)
  (key-chord-define evil-normal-state-map "gl" 'evil-window-right)
  ;; (key-chord-define evil-normal-state-map "gh" 'evil-window-vsplit)
  ;; (key-chord-define evil-normal-state-map "gv" 'evil-window-split)

  (define-key evil-normal-state-map (kbd "gx") 'evil-window-delete)
  (define-key evil-normal-state-map (kbd "gv") 'split-goto-h)
  (define-key evil-normal-state-map (kbd "gb") 'split-goto-v)
  (define-key evil-normal-state-map (kbd "gp") 'lsp-ui-doc-show)

  (define-key evil-normal-state-map (kbd "<f13> h") 'evil-window-left)
  (define-key evil-normal-state-map (kbd "<f13> j") 'evil-window-down)
  (define-key evil-normal-state-map (kbd "<f13> k") 'evil-window-up)
  (define-key evil-normal-state-map (kbd "<f13> l") 'evil-window-right)

  (define-key evil-normal-state-map (kbd "S-<f13> h") 'windmove-swap-states-left)
  (define-key evil-normal-state-map (kbd "S-<f13> j") 'windmove-swap-states-down)
  (define-key evil-normal-state-map (kbd "S-<f13> k") 'windmove-swap-states-up)
  (define-key evil-normal-state-map (kbd "S-<f13> l") 'windmove-swap-states-right)

  (define-key evil-normal-state-map (kbd "[e") 'flycheck-previous-error)
  (define-key evil-normal-state-map (kbd "]e") 'flycheck-next-error)

  (defun pii/decrease-window-height ()
    "Decrease the current window by a greater amount than the default"
    (interactive)
    (shrink-window 10))

  (defun pii/increase-window-height ()
    "Increase the current window by a greater amount than the default"
    (interactive)
    (enlarge-window 10))

  (defun pii/decrease-window-width ()
    "Decrease the current window by a greater amount than the default"
    (interactive)
    (shrink-window-horizontally 10))

  (defun pii/increase-window-width ()
    "Increase the current window by a greater amount than the default"
    (interactive)
    (enlarge-window-horizontally 10))

  (define-key evil-normal-state-map (kbd "M-<f13> h") 'pii/decrease-window-width)
  (define-key evil-normal-state-map (kbd "M-<f13> j") 'pii/decrease-window-height)
  (define-key evil-normal-state-map (kbd "M-<f13> k") 'pii/increase-window-height)
  (define-key evil-normal-state-map (kbd "M-<f13> l") 'pii/increase-window-width)

 (global-set-key (kbd "M-o") 'ace-window)

  (key-chord-mode 1))

;; Which key
(use-package which-key
  :init (which-key-mode)
  :diminish which-key-mode
  :config
  (setq which-key-idle-delay 0.3))

;; Modeline
(use-package all-the-icons)

(use-package doom-modeline
  :ensure t
  :init (doom-modeline-mode 1)
  :custom ((doom-modeline-height 10)
           (doom-modeline-unicode-fallback t)))

(add-to-list 'custom-theme-load-path "~/.emacs.scratch/")

;; Doom-themes
(use-package doom-themes
  :config
  ;; Global settings (defaults)
  (setq doom-themes-enable-bold t    ; if nil, bold is universally disabled
        doom-themes-enable-italic t) ; if nil, italics is universally disabled
   (load-theme 'doom-1337-custom t))

;; Rainbow delimiters
(use-package rainbow-delimiters
  :hook (prog-mode . rainbow-delimiters-mode))

(use-package rainbow-identifiers)

(setq rainbow-identifiers-cie-l*a*b*-lightness 80)
(setq rainbow-identifiers-cie-l*a*b*-saturation 80)
(setq rainbow-identifiers-cie-l*a*b*-color-count 9)

(setq rainbow-identifiers-choose-face-function 'rainbow-identifiers-cie-l*a*b*-choose-face)
  (add-hook 'prog-mode-hook 'rainbow-identifiers-mode)
  (setq rainbow-identifiers-faces-to-override '(lsp-face-semhl-member
                                               lsp-face-semhl-parameter
                                               lsp-face-semhl-variable))

(use-package highlight-indent-guides
  :hook ((prog-mode conf-mode) . highlight-indent-guides-mode)
  :init
  (setq highlight-indent-guides-method 'column
        highlight-indent-guides-suppress-auto-error t)
  :config
  ;; (highlight-indent-guides-mode)

  (defun +indent-guides-init-faces-h (&rest _)
    (when (display-graphic-p)
      (highlight-indent-guides-auto-set-faces))))

;; Disable line numbers for some modes
(dolist (mode '(org-mode-hook
                term-mode-hook
                eshell-mode-hook))
    (add-hook mode (lambda () (highlight-indent-guides-mode nil))))

(defun pii/org-font-setup ()
  (font-lock-add-keywords 'org-mode
                           '(("^ *\\([-]\\) "
                              (0 (prog1 () (compose-region (match-beginning 1) (match-end 1) "•"))))))

  (dolist (face '((org-level-1 . 1.9)
                   (org-level-2 . 1.7)
                   (org-level-3 . 1.5)
                   (org-level-4 . 1.3)
                   (org-level-5 . 1.1)
                   (org-level-6 . 1.1)
                   (org-level-7 . 1.1)
                   (org-level-8 . 1.1)))
     (set-face-attribute (car face) nil :weight 'regular :height (cdr face)))
     ;; Ensure that anything that should be fixed-pitch in Org files appears that way
     (set-face-attribute 'org-block nil    :foreground nil :inherit 'fixed-pitch)
     (set-face-attribute 'org-table nil    :inherit 'fixed-pitch)
     (set-face-attribute 'org-formula nil  :inherit 'fixed-pitch)
     (set-face-attribute 'org-code nil     :inherit '(shadow fixed-pitch))
     (set-face-attribute 'org-table nil    :inherit '(shadow fixed-pitch))
     (set-face-attribute 'org-verbatim nil :inherit '(shadow fixed-pitch))
     (set-face-attribute 'org-special-keyword nil :inherit '(font-lock-comment-face fixed-pitch))
     (set-face-attribute 'org-meta-line nil :inherit '(font-lock-comment-face fixed-pitch))
     (set-face-attribute 'org-checkbox nil  :inherit 'fixed-pitch)
     (set-face-attribute 'line-number nil :inherit 'fixed-pitch)
     (set-face-attribute 'line-number-current-line nil :inherit 'fixed-pitch))

;; Org

(defun pii/org-mode-setup ()
   (org-indent-mode)
   (variable-pitch-mode 1)
   ;; (auto-fill mode 0)
   (visual-line-mode 0)
   (display-line-numbers-mode 0)
   (highlight-indent-guides-mode nil)
   (setq evil-auto-indent nil))

(use-package org
   :hook (org-mode . pii/org-mode-setup)
   :config
   (setq org-ellipsis " ▾")
   (pii/org-font-setup))

(with-eval-after-load 'org
   (org-babel-do-load-languages
     'org-babel-load-languages
     '((emacs-lisp . t)
     (python . t)))

   (push '("conf-unix" . conf-unix) org-src-lang-modes))

(with-eval-after-load 'org
   ;; This is needed as of Org 9.2
   (require 'org-tempo)

   (add-to-list 'org-structure-template-alist '("sh" . "src shell"))
   (add-to-list 'org-structure-template-alist '("el" . "src emacs-lisp"))
   (add-to-list 'org-structure-template-alist '("py" . "src python")))

(use-package org-roam
  :ensure t
  :custom
  (org-roam-directory (file-truename "/home/champii/roam/"))
  :bind (("C-c n l" . org-roam-buffer-toggle)
         ("C-c n f" . org-roam-node-find)
         ("C-c n g" . org-roam-graph)
         ("C-c n i" . org-roam-node-insert)
         ("C-c n c" . org-roam-capture)
         ;; Dailies
         ("C-c n j" . org-roam-dailies-capture-today))
  :config
  (org-roam-db-autosync-mode)
  (setq org-roam-v2-ack t)
  ;; If using org-roam-protocol
  (require 'org-roam-protocol))

(use-package org-bullets
   :hook (org-mode . org-bullets-mode)
   :custom
   (org-bullets-bullet-list '("◉" "○" "●" "○" "●" "○" "●")))

(defun pii/org-mode-visual-fill ()
  (setq visual-fill-column-width 140
         visual-fill-column-center-text t)
  (visual-fill-column-mode 1))

(use-package visual-fill-column
  :hook (org-mode . pii/org-mode-visual-fill))

(use-package flycheck
  :ensure
  :config
  (setq flycheck-indication-mode 'right-fringe))

(use-package lsp-mode
  :commands (lsp lsp-deferred)
  :hook (lsp-mode)
  :init
  (setq lsp-keymap-prefix "C-c l")  ;; Or 'C-l', 's-l'
  :custom
  (lsp-rust-server 'rust-analyzer)
  (lsp-rust-analyzer-cargo-watch-command "clippy")
  (lsp-eldoc-render-all t)
  (lsp-idle-delay 0.6)
  ;; (lsp-mode-autoconfiguration nil)
  (lsp-rust-analyzer-server-display-inlay-hints t)
  :config

  (setq lsp-ui-doc-position 'at-point)
  (setq lsp-ui-doc-show-with-cursor nil)
  (setq lsp-ui-doc-show-with-mouse nil)
  (setq lsp-ui-peek-enable t)
  (setq lsp-ui-sideline-show-hover nil)
  (setq lsp-ui-peek-always-show nil)
  (setq lsp-eldoc-enable-hover nil)
  (setq lsp-ui-sideline-show-hover t)
  (setq lsp-ui-doc-enable t)

  (add-hook 'lsp-mode-hook 'lsp-ui-mode)
  (lsp-enable-which-key-integration t))

(use-package lsp-ui
  :hook (lsp-mode . lsp-ui-mode)
  :custom
  (lsp-ui-doc-position 'at-point)
  (lsp-ui-doc-show-with-cursor nil)
  (lsp-ui-doc-show-with-mouse nil)
  (lsp-ui-peek-enable t)
  (lsp-ui-sideline-show-hover nil)
  (lsp-ui-peek-always-show nil)
  (lsp-eldoc-enable-hover nil)
  (lsp-ui-sideline-show-hover nil)
  (lsp-ui-doc-enable t))

(use-package lsp-ivy
  :after lsp)

(use-package evil-nerd-commenter
  :bind ("M-/" . evilnc-comment-or-uncomment-lines))

(use-package rustic
  :ensure
  :bind (:map rustic-mode-map
              ("M-j" . lsp-ui-imenu)
              ("M-?" . lsp-find-references)
              ("C-c C-c l" . flycheck-list-errors)
              ("C-c C-c a" . lsp-execute-code-action)
              ("C-c C-c r" . lsp-rename)
              ("C-c C-c q" . lsp-workspace-restart)
              ("C-c C-c Q" . lsp-workspace-shutdown)
              ("C-c C-c s" . lsp-rust-analyzer-status))
  :config
  ;; uncomment for less flashiness
  ;; (setq lsp-eldoc-hook nil)
  ;; (setq lsp-enable-symbol-highlighting nil)
  ;; (setq lsp-signature-auto-activate nil)

  ;; comment to disable rustfmt on save
  (setq rustic-format-on-save nil)
  (setq rustic-lsp-format t)
  (add-hook 'rustic-mode-hook 'pii/rustic-mode-hook))

(defun pii/rustic-mode-hook ()
  ;; so that run C-c C-c C-r works without having to confirm, but don't try to
  ;; save rust buffers that are not file visiting. Once
  ;; https://github.com/brotzeit/rustic/issues/253 has been resolved this should
  ;; no longer be necessary.
  (when buffer-file-name
    (setq-local buffer-save-without-query t)))

(use-package company
  :custom
  (company-idle-delay 0.5) ;; how long to wait until popup
  ;;(company-begin-commands nil) ;; uncomment to disable popup
  :bind
  (:map company-active-map
              ("C-n". company-select-next)
              ("C-p". company-select-previous)
              ("M-<". company-select-first)
              ("M->". company-select-last)))

(use-package company-box
  :hook (company-mode . company-box-mode))

(use-package yasnippet
  :config
  (yas-reload-all)
  (add-hook 'prog-mode-hook 'yas-minor-mode)
  (add-hook 'text-mode-hook 'yas-minor-mode))

;; Projectile
(use-package projectile
  :diminish projectile-mode
  :config (projectile-mode)
  :custom (projectile-completion-system 'ivy)
  :bind-keymap ("C-c p" . projectile-command-map)
  :init ())

(use-package counsel-projectile
  :config (counsel-projectile-mode))

(setq auth-sources '("~/.authinfo.gpg"))
(use-package forge)

(use-package magit
  :commands (magit-status magit-get-current-branch)
  :custom
  (magit-display-buffer-function #'magit-display-buffer-same-window-except-diff-v1))

;;; Git Gutter
;;Git gutter is great for giving visual feedback on changes, but it doesn't play well
;;with org-mode using org-indent. So I don't use it globally.
(use-package git-gutter
  :defer t
  :init
  :config
  (setq git-gutter:update-interval 1
        git-gutter:window-width 2
        git-gutter:ask-p nil)
  (global-git-gutter-mode +1)
  (defhydra hydra-git-gutter (:body-pre (git-gutter-mode 1)
                                        :hint nil)
    "
 Git gutter:
   _j_: next hunk        _s_tage hunk     _q_uit
   _k_: previous hunk    _r_evert hunk    _Q_uit and deactivate git-gutter
   ^ ^                   _p_opup hunk
   _h_: first hunk
   _l_: last hunk        set start _R_evision
 "
    ("j" git-gutter:next-hunk)
    ("k" git-gutter:previous-hunk)
    ("h" (progn (goto-char (point-min))
                (git-gutter:next-hunk 1)))
    ("l" (progn (goto-char (point-min))
                (git-gutter:previous-hunk 1)))
    ("s" git-gutter:stage-hunk)
    ("r" git-gutter:revert-hunk)
    ("p" git-gutter:popup-hunk)
    ("R" git-gutter:set-start-revision)
    ("q" nil :color blue)
    ("Q" (progn (git-gutter-mode -1)
                ;; git-gutter-fringe doesn't seem to
                ;; clear the markup right away
                (sit-for 0.1)
                (git-gutter:clear))
     :color blue)))

(use-package git-gutter-fringe
  :after git-gutter
  :demand fringe-helper
  :config
  ;; subtle diff indicators in the fringe
  ;; places the git gutter outside the margins.
  (setq-default fringes-outside-margins t)
  ;; thin fringe bitmaps
  (define-fringe-bitmap 'git-gutter-fr:added
    [224 224 224 224 224 224 224 224 224 224 224 224 224 224 224 224 224 224 224 224 224 224 224 224 224]
    nil nil 'center)
  (define-fringe-bitmap 'git-gutter-fr:modified
    [224 224 224 224 224 224 224 224 224 224 224 224 224 224 224 224 224 224 224 224 224 224 224 224 224]
    nil nil 'center)
  (define-fringe-bitmap 'git-gutter-fr:deleted
    [0 0 0 0 0 0 0 0 0 0 0 0 0 128 192 224 240 248]
    nil nil 'center))

(use-package block-nav
  :config
  (define-key evil-normal-state-map (kbd "J") 'block-nav-next-block)
  (define-key evil-normal-state-map (kbd "K") 'block-nav-previous-block)
  (define-key evil-normal-state-map (kbd "L") 'block-nav-next-indentation-level)
  (define-key evil-normal-state-map (kbd "H") 'block-nav-previous-indentation-level))

;; Ivy
(use-package ivy
  :diminish
  :bind (("C-s" . swiper)
         :map ivy-minibuffer-map
         ("C-l" . ivy-immediate-done)
         ("TAB" . ivy-alt-done)
         ("C-j" . ivy-next-line)
         ("C-k" . ivy-previous-line)
         :map ivy-switch-buffer-map
         ("C-k" . ivy-previous-line)
         ("C-l" . ivy-alt-done)
         ("C-d" . ivy-switch-buffer-kill)
         :map ivy-reverse-i-search-map
         ("C-k" . ivy-previous-line)
         ("C-d" . ivy-reverse-i-search-kill))
  :config
  (ivy-mode 1)
  (setq ivy-use-virtual-buffers t)
  (setq ivy-count-format "(%d/%d) ")
  (setq ivy-wrap t))

(use-package ivy-prescient
  :after counsel
  :custom
  (ivy-prescient-enable-filtering nil)
  :config
  ;; Uncomment the following line to have sorting remembered across sessions!
  (prescient-persist-mode 1)
  (ivy-prescient-mode 1))

;; Ivy which
(use-package ivy-rich
  :init
  (ivy-rich-mode 1))

;; Counsel
(use-package counsel
  :init (counsel-mode 1)
  :bind (("C-c b" . counsel-bookmark)
         :map minibuffer-local-map
         ("C-r" . 'counsel-minibuffer-history))
  :config
  (setq ivy-initial-inputs-alist nil))

(global-set-key (kbd "C-x b") 'counsel-switch-buffer)

(use-package avy
  :bind (("M-s" . avy-goto-word-1)))

(use-package ace-window
  :config
  (setq aw-keys '(?a ?d ?f ?g ?j ?k ?l))
  (general-define-key "M-o" 'ace-window))

(defvar aw-dispatch-alist
  '((?x aw-delete-window "Delete Window")
        (?s aw-swap-window "Swap Windows")
        (?m aw-move-window "Move Window")
        (?c aw-copy-window "Copy Window")
        (?b aw-switch-buffer-in-window "Select Buffer")
        (?p aw-flip-window)
        (?u aw-switch-buffer-other-window "Switch Buffer Other Window")
        (?c aw-split-window-fair "Split Fair Window")
        (?v aw-split-window-vert "Split Vert Window")
        (?h aw-split-window-horz "Split Horz Window")
        (?X delete-other-windows "Delete Other Windows")
        (?? aw-show-dispatch-help)))

;; Helpful
(use-package helpful
  :custom
  (counsel-describe-function-function #'helpful-callable)
  (counsel-describe-variable-function #'helpful-variable)
  :bind
  ([remap describe-function] . counsel-describe-function)
  ([remap describe-command] . helpful-command)
  ([remap describe-variable] . counsel-describe-variable)
  ([remap describe-key] . counsel-describe-key))

(use-package dired
  :ensure nil
  :commands (dired dired-jump)
  :bind (("C-x C-j" . dired-jump))
  :custom ((dired-listing-switches "-agho --group-directories-first"))
  :config
  (evil-collection-define-key 'normal 'dired-mode-map
      "h" 'dired-single-up-directory
      "l" 'dired-single-buffer))

(use-package dired-single
  :commands (dired dired-jump))

(use-package all-the-icons-dired
  :hook (dired-mode . all-the-icons-dired-mode))

(use-package dired-open
  :commands (dired dired-jump)
  :config
  ;; Doesn't work as expected!
  ;;(add-to-list 'dired-open-functions #'dired-open-xdg t)
  (setq dired-open-extensions '(("png" . "feh")
                                  ("mkv" . "mpv"))))

(use-package dired-hide-dotfiles
  :hook (dired-mode . dired-hide-dotfiles-mode)
  :config
  (evil-collection-define-key 'normal 'dired-mode-map
      "H" 'dired-hide-dotfiles-mode))

(use-package hydra)

(use-package no-littering)

;; no-littering doesn't set this by default so we must place
;; auto save files in the same path as it uses for sessions
(setq auto-save-file-name-transforms
      `((".*" ,(no-littering-expand-var-file-name "auto-save/") t)))

(use-package beacon
  :config
  (beacon-mode 1))

(use-package dimmer
  :config
  (dimmer-configure-which-key)
  (dimmer-configure-magit)
  (dimmer-configure-org)
  (dimmer-mode t))

(use-package drag-stuff
  :config
  (drag-stuff-global-mode 1)
  (drag-stuff-define-keys)
  (global-set-key (kbd "M-j") 'drag-stuff-down)
  (global-set-key (kbd "M-k") 'drag-stuff-up)
  (drag-stuff-mode t))

(use-package aggressive-indent
  :config
  (global-aggressive-indent-mode 1)
  (add-to-list 'aggressive-indent-excluded-modes 'html-mode))

(use-package multiple-cursors
  :config
  (setq mc/always-repeat-command t)
  (global-set-key (kbd "M-d") 'mc/mark-next-like-this-word))

(use-package vterm
  :ensure t)

(defun pii/org-babel-tangle-config ()
  (when (string-equal (file-name-directory (buffer-file-name))
                      (expand-file-name user-emacs-directory))
    ;; Dynamic scoping to the rescue
    (let ((org-confirm-babel-evaluate nil))
      (org-babel-tangle))))

(add-hook 'org-mode-hook (lambda () (add-hook 'after-save-hook #'pii/org-babel-tangle-config)))

(setq gc-cons-threshold (* 2 1000 1000))
