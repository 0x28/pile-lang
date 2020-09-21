;;; pile.el --- Major mode for the pile programming language  -*- lexical-binding: t; coding: utf-8 -*-
;; Version: 0.1

;;; Commentary:

;; Provides syntax highlighting and completion for the pile programming
;; language. Use M-x `package-install-from-buffer' to install this package.

;;; Code:

(defgroup pile nil
  "Editing pile programs."
  :group 'languages)

(defcustom pile-executable-name "pile"
  "Program invoked by pile-mode."
  :type 'file
  :group 'pile)

(defconst pile-font-lock-keywords
  '("if" "dotimes" "while" "let" "begin" "end" "use"))

(defconst pile-font-lock-builtins
  '("and"
    "assert"
    "clear"
    "concat"
    "contains"
    "downcase"
    "drop"
    "dup"
    "float"
    "format"
    "index"
    "integer"
    "length"
    "natural"
    "not"
    "or"
    "pick"
    "print"
    "showstack"
    "stacksize"
    "swap"
    "trim"
    "upcase"))

(defvar pile--function-regexp
  "end[[:space:]\n]*->[[:space:]\n]*\\([[:alpha:]][[:alnum:]_]*\\)")

;;;###autoload
(define-derived-mode pile-mode prog-mode "pile"
  "Major mode for the pile programming language."
  (setq-local comment-start "#")
  (setq-local comment-end "")
  (setq-local comment-use-syntax t)
  (setq-local imenu-generic-expression `(("function" ,pile--function-regexp 1)))
  (setq-local indent-region-function #'pile-indent-region)
  (add-hook 'completion-at-point-functions
            #'pile-completion-at-point nil 'local)

  (font-lock-add-keywords
   nil
   `(("\\_<[+-]?[0-9].?[0-9]*\\_>" . font-lock-constant-face)
     ("\\_<\\(true\\|false\\)\\_>" . font-lock-constant-face)
     (,pile--function-regexp 1 font-lock-function-name-face)
     (,(regexp-opt pile-font-lock-keywords 'words) . font-lock-keyword-face)
     (,(regexp-opt pile-font-lock-builtins 'words) . font-lock-builtin-face)))

  (modify-syntax-entry ?# "<" pile-mode-syntax-table)
  (modify-syntax-entry ?\n ">" pile-mode-syntax-table))

(defun pile--get-completions (prefix)
  "Get the completions starting with PREFIX for the current line."
  (process-lines pile-executable-name
                 "--complete"
                 prefix
                 (number-to-string (line-number-at-pos nil t))
                 (buffer-file-name)))

(defun pile-completion-at-point ()
  "Function used for `completion-at-point-functions' in `pile-mode'."
  (with-syntax-table pile-mode-syntax-table
    (unless (or (nth 3 (syntax-ppss))
                (nth 4 (syntax-ppss))
                (not (executable-find pile-executable-name))
                (not (buffer-file-name)))
      (let ((start (condition-case nil
                       (save-excursion
                         (backward-sexp 1)
                         (point))
                     (scan-error (point))))
            (end (point))
            (collection (completion-table-dynamic #'pile--get-completions)))
        (list start
              end
              collection)))))

(defun pile-indent-region (start end)
  "Indent the pile program between START and END."
  (let ((format-buffer " *pile-format*"))
    (save-restriction
      (narrow-to-region start end)
      (call-process-region start
                           end
                           pile-executable-name
                           nil
                           format-buffer
                           nil
                           "--format" "-")
      (replace-buffer-contents format-buffer)
      (kill-buffer format-buffer))))

;;;###autoload
(add-to-list 'auto-mode-alist
             '("\\.pile\\'" . pile-mode))

(provide 'pile)
;;; pile.el ends here
