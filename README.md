# elsa-lsp

A minimal language server for [Elsa](https://github.com/ucsd-progsys/elsa) — UCSD's lambda calculus teaching language.

## Features

- **Completion**: suggests `let` bindings defined in the current file, with an "(expand)" variant that inserts the full body in parentheses

## Editor support

- **Zed**: via [zed-elsa](https://github.com/MrPoloGit/zed-elsa) (downloaded automatically)
- **Neovim / Vim**: point `nvim-lspconfig` or `vim-lsp` at the binary
- **Emacs**: use `lsp-mode` with a custom server entry

## Installation

### From GitHub releases

Download the binary for your platform from the [releases page](https://github.com/MrPoloGit/elsa-lsp/releases).

### From source

```bash
cargo install --git https://github.com/MrPoloGit/elsa-lsp
```

## Usage

The server communicates over stdio (standard LSP transport):

```bash
elsa-lsp
```

Point your editor's LSP client at `elsa-lsp` with file type `*.lc`.

### Related resources
- https://github.com/ucsd-progsys/elsa
- https://hackage-content.haskell.org/package/elsa-0.3.0.0/docs/doc-index.html
- https://elsa.goto.ucsd.edu/index.html
