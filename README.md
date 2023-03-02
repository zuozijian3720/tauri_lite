# Tauri Lite
## API
### File System
[*] stat(path) -> Stat
[*] exists(path) -> boolean

[*] read(path) -> string
[*] write(path, data: string) -> void
[ ] append(path, data: string) -> void

[*] mv(path, newPath) -> void
[*] cp(path, newPath) -> void
[*] rm(path) -> void

[*] ls(path) -> string[]
[*] mkDir(path) -> void
[*] rmDir(path) -> void

[ ] link(path, newPath) -> void

### Http
[*] request(url, options?) -> string
[ ] download(url, path, options?) -> void

### OS
[*] info() -> string
[*] dirs() -> string

### Process
[*] exec(command | file, args?, options?) -> string
[*] pid() -> number
[*] cwd() -> string
[*] chDir() -> void
[*] env() -> Env
[*] exit() -> !