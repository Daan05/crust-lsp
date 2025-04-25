<H1>Crust lsp</H1>

<H2>Features</H2>

- completions
- snippets
- hover
- go to definition (go to top of file*)

<H2>Neovim setup</H2>

```lua
local configs = require("lspconfig.configs")
if not configs.crust_lsp then
    configs.crust_lsp = {
        default_config = {
            cmd = { "/full/path/to/your/lsp/executable" },
            -- for example
            -- cmd = { "/home/dhval/Dev/rust/crust-lsp/target/debug/crust-lsp" },
            filetypes = { "crust" },
            root_dir = function(fname)
                return vim.fn.getcwd()
            end,
        },
    }
end

require("lspconfig").crust_lsp.setup({
    on_attach = function(client, bufnr)
        print("Attached crust_lsp")
    end,
    on_init = function(client)
        print("Initializing crust_lsp")
    end,
})
```
