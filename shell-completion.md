# RTW Shell Completion

supported shells: bash, zsh, fish, powershell, elvish

Write completion file for `<shell>` to stdout:

```
rtw completion <shell>
```

## oh-my-zsh

```
.oh-my-zsh/custom/plugins/rtw
├── _rtw
└── rtw.plugin.zsh
```

```
mkdir -p ~/.oh-my-zsh/custom/plugins/rtw
rtw completion zsh > ~/.oh-my-zsh/custom/plugins/rtw/_rtw
echo "#rtw completion plugin" > ~/.oh-my-zsh/custom/plugins/rtw/rtw.plugin.zsh
```

Add `rtw` to `plugins` in `.zshrc`:

```
# Which plugins would you like to load? (plugins can be found in ~/.oh-my-zsh/plugins/*)
# Custom plugins may be added to ~/.oh-my-zsh/custom/plugins/
# Example format: plugins=(rails git textmate ruby lighthouse)
# Add wisely, as too many plugins slow down shell startup.
plugins=(git rtw)
```
