# prompt-dir

Prints the last 4 directories in the path and changes the home directory to ~.

This is included in my zsh PS1 as:

```shell
export PS1=$'%B%F{39}%n%f%b%F{43}@machine-name%f:%F{117}$(/path/to/prompt/target/release/prompt)%f $ '
```
