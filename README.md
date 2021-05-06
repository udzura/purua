# purua
Pure Rust Lua implementation

## Usage

```console
$ bat lua_examples/defun.lua
───────┬──────────────────────────────────────────
       │ File: lua_examples/defun.lua
───────┼──────────────────────────────────────────
   1   │ function myfunc()
   2   │    print("Call my own func!\n")
   3   │ end
   4   │ 
   5   │ myfunc()
   6   │ 
   7   │ function println(myarg)
   8   │    print(myarg)
   9   │    print("\n")
  10   │ end
  11   │ 
  12   │ println("Hello With LF")
  13   │ 
  14   │ function getstr()
  15   │    ret = "Hello returned MyStr"
  16   │    return ret
  17   │ end
  18   │ 
  19   │ println(getstr())
───────┴──────────────────────────────────────────
$ cargo run lua_examples/defun.lua
   Compiling combine-language v4.0.0
   Compiling purua v0.1.0 (/usr/local/ghq/github.com/udzura/purua)
    Finished dev [unoptimized + debuginfo] target(s) in 1.62s
     Running `target/debug/purua lua_examples/defun.lua`
Call my own func!
Hello With LF
Hello returned MyStr
```

## Contribute, License

Issues, patches are welcomed.

See the file for LICENSE.
