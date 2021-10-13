;; This file has been generated by org-mode
;; and MUST not be modified by hand.
;; Everything put in this file will be deleted
;; Please read the ./init.el.org file

(setq inhibit-startup-message t)

(scroll-bar-mode -1)

(tool-bar-mode -1)
(tooltip-mode -1)

;; No borders on windows
(set-fringe-mode 0)

(menu-bar-mode -1)

(global-auto-revert-mode t)

(setq display-line-numbers-type 'relative)

(global-set-key (kbd "C-c C-c") 'evil-escape)

(setq global-hl-line-mode t)

(server-start)

(set-frame-parameter (selected-frame) 'alpha '(90 . 75))

;; Line Numbers
(column-number-mode)
(global-display-line-numbers-mode t)

;; Disable line numbers for some modes
(dolist (mode '(org-mode-hook
                term-mode-hook
                eshell-mode-hook))
  (add-hook mode (lambda () (display-line-numbers-mode 0))))

(set-face-attribute 'default nil :font "xos4 Terminess Powerline-9" :height 50)

;; Make ESQ quit prompts
(global-set-key (kbd "<escape>") 'keyboard-escape-quit)

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
   '(no-littering desktop-environment org-bullets forge evil-magit magit counsel-projectile projectile hydra evil-collection evil general doom-themes helpful ivy-rich which-key rainbow-delimiters doom-modeline diminish ivy use-package))
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

;; Ivy
(use-package ivy
  :diminish
  :bind (("C-s" . swiper)
         :map ivy-minibuffer-map
         ("TAB" . ivy-alt-done)
         ("C-l" . ivy-alt-done)
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

;; Counsel
(use-package counsel
  :init (counsel-mode 1)
  :bind (("C-c b" . counsel-bookmark)
         :map minibuffer-local-map
         ("C-r" . 'counsel-minibuffer-history))
  :config
  (setq ivy-initial-inputs-alist nil))

(global-set-key (kbd "C-x b") 'counsel-switch-buffer)

;; Modeline
(use-package all-the-icons)

(use-package doom-modeline
  :ensure t
  :init (doom-modeline-mode 1)
  :custom ((doom-modeline-height 10)
           (doom-modeline-unicode-fallback t)))

;; Rainbow delimiters
(use-package rainbow-delimiters
  :hook (prog-mode . rainbow-delimiters-mode))

;; Which key
(use-package which-key
  :init (which-key-mode)
  :diminish which-key-mode
  :config
  (setq which-key-idle-delay 0.3))

;; Ivy which
(use-package ivy-rich
  :init
  (ivy-rich-mode 1))

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

;; Doom-themes
(use-package doom-themes
  :config
  ;; Global settings (defaults)
  (setq doom-themes-enable-bold t    ; if nil, bold is universally disabled
        doom-themes-enable-italic t) ; if nil, italics is universally disabled
  (load-theme 'doom-1337-custom t))

(use-package key-chord)

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
  ;(setq evil-want-C-i-jump nil)
  ;:hook (evil-mode . pii/evil-hook)
  :config
  (evil-mode 1))

(use-package evil-collection
  :after evil
  :config
  (evil-collection-init))

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

  (pii/leader-keys
    ;; Basics
    "RET" '(counsel-bookmark :which-key "Bookmarks")
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

    "e" '(:ignore t :which-key "Eval")
    "eb" '(eval-buffer :witch-key "Eval Buffer")
    "ee" '(eval-last-sexp :witch-key "Eval last SEXP")

    "o" '(:ignore t :which-key "Open")
    "ot" '(vterm-other-window :which-key "VTerm")

    "w" '(:ignore t :which-key "Windows")
    "wl" '(evil-window-right :which-key "Go to right window")
    "wh" '(evil-window-left :which-key "Go to left window")
    "wj" '(evil-window-down :which-key "Go to down window")
    "wk" '(evil-window-up :which-key "Go to up window")
    "wd" '(evil-window-delete :which-key "Close current window")

    "q" '(:ignore t :which-key "Session")
    "qq" '(save-buffers-kill-terminal :which-key "Session")
    "p" '(projectile-command-map :which-key "Projects"))

  (setq key-chord-two-keys-delay 0.3)
  (key-chord-define evil-insert-state-map "jk" 'evil-normal-state)
  (key-chord-define evil-normal-state-map "zx" 'save-buffer)
  (key-chord-define evil-insert-state-map "zx" 'pii/evil-save-go-normal)
  (key-chord-define evil-normal-state-map "gc" 'comment-line)
  (key-chord-define evil-normal-state-map "gh" 'evil-window-left)
  (key-chord-define evil-normal-state-map "gj" 'evil-window-down)
  (key-chord-define evil-normal-state-map "gk" 'evil-window-up)
  (key-chord-define evil-normal-state-map "gl" 'evil-window-right)
  ;; (key-chord-define evil-normal-state-map "gh" 'evil-window-vsplit)
  ;; (key-chord-define evil-normal-state-map "gv" 'evil-window-split)

  (define-key evil-normal-state-map (kbd "gx") 'evil-window-delete)
  (define-key evil-normal-state-map (kbd "gv") 'split-goto-h)
  (define-key evil-normal-state-map (kbd "gb") 'split-goto-v)

  (key-chord-mode 1))

(use-package hydra)

;; Projectile
(use-package projectile
  :diminish projectile-mode
  :config (projectile-mode)
  :custom (projectile-completion-system 'ivy)
  :bind-keymap ("C-c p" . projectile-command-map)
  :init ())

(use-package counsel-projectile
  :config (counsel-projectile-mode))

(use-package magit
  :commands (magit-status magit-get-current-branch)
  :custom
  (magit-display-buffer-function #'magit-display-buffer-same-window-except-diff-v1))

(setq auth-sources '("~/.authinfo.gpg"))
(use-package forge)

;; Org

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
    ;(set-face-attribute 'org-level-3 nil :foreground "green"))

(defun pii/org-mode-setup ()
  (org-indent-mode)
  (variable-pitch-mode 1)
  (auto-fill mode 0)
  (visual-line-mode 1)
  (setq evil-auto-indent nil))

(use-package org
  :hook (org-mode . pii/org-mode-setup)
  :config
  (setq org-ellipsis " ▾")
  (pii/org-font-setup))

(use-package org-bullets
  :hook (org-mode . org-bullets-mode)
  :custom
  (org-bullets-bullet-list '("◉" "○" "●" "○" "●" "○" "●")))

(defun pii/org-mode-visual-fill ()
  (setq visual-fill-column-width 150
        visual-fill-column-center-text t)
  (visual-fill-column-mode 1))

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

(use-package visual-fill-column
  :hook (org-mode . pii/org-mode-visual-fill))

(use-package flycheck :ensure)

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
   (lsp-rust-analyzer-server-display-inlay-hints t)
   :config
   (add-hook 'lsp-mode-hook 'lsp-ui-mode)
   (lsp-enable-which-key-integration t))

 (use-package lsp-ui
   :hook (lsp-mode . lsp-ui-mode)
   :custom
   (lsp-ui-doc-position 'bottom)
   (lsp-ui-peek-always-show t)
   (lsp-ui-sideline-show-hover t)
   (lsp-ui-doc-enable nil))

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
  (setq rustic-format-on-save t)
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
  (company-idle-delay 9.5) ;; how long to wait until popup
  ;; (company-begin-commands nil) ;; uncomment to disable popup
  :bind
  (:map company-active-map
              ("C-n". company-select-next)
              ("C-p". company-select-previous)
              ("M-<". company-select-first)
              ("M->". company-select-last)))

(use-package yasnippet
  :config
  (yas-reload-all)
  (add-hook 'prog-mode-hook 'yas-minor-mode)
  (add-hook 'text-mode-hook 'yas-minor-mode))

;; (use-package color-identifiers-mode
;;   :config
;;   (global-color-identifiers-mode t)
;;   ;(setq color-identifiers:color-luminance 1.0)
;;   (setq color-identifiers:coloring-method 'hash))

;; (add-hook 'after-init-hook 'global-color-identifiers-mode)

(use-package rainbow-identifiers)

 (setq rainbow-identifiers-cie-l*a*b*-lightness 100)
 (setq rainbow-identifiers-cie-l*a*b*-saturation 100)
 (setq rainbow-identifiers-cie-l*a*b*-color-count 40)

(setq rainbow-identifiers-choose-face-function 'rainbow-identifiers-cie-l*a*b*-choose-face)
 (add-hook 'prog-mode-hook 'rainbow-identifiers-mode)
 (setq rainbow-identifiers-faces-to-override '(lsp-face-semhl-member
                                               lsp-face-semhl-parameter
                                               lsp-face-semhl-variable))

(use-package vterm
  :ensure t)

(use-package highlight-indent-guides
  :hook ((prog-mode text-mode conf-mode) . highlight-indent-guides-mode)
  :init
  (setq highlight-indent-guides-method 'character
        highlight-indent-guides-suppress-auto-error t)
  :config
  ;; (highlight-indent-guides-mode)
  (defun +indent-guides-init-faces-h (&rest _)
    (when (display-graphic-p)
      (highlight-indent-guides-auto-set-faces))))

(use-package no-littering)

;; no-littering doesn't set this by default so we must place
;; auto save files in the same path as it uses for sessions
(setq auto-save-file-name-transforms
      `((".*" ,(no-littering-expand-var-file-name "auto-save/") t)))

(defun pii/org-babel-tangle-config ()
  (when (string-equal (file-name-directory (buffer-file-name))
                      (expand-file-name user-emacs-directory))
    ;; Dynamic scoping to the rescue
    (let ((org-confirm-babel-evaluate nil))
      (org-babel-tangle))))

(add-hook 'org-mode-hook (lambda () (add-hook 'after-save-hook #'pii/org-babel-tangle-config)))
