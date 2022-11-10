# Ideas and Insights

## 1. Managing Growing Projects with Packages, Crates, and Modules
In chapter `7` or `Rust programming language` about `Managing file systems`, I
got the following insight:

I found that, there are similarity among calling objects, functions, methods,
variables, constants ..etc. from different files or folders, among different
programming languages.

| idx | language | files Managing style     | Note                                    |
|-----|----------|--------------------------|-----------------------------------------|
| 1   | lua      | init.lua                 |                                         |
| 2   | python   | __init__.py              |                                         |
| 3   | c++      | <header_file>.h          |                                         |
| 4   | rust     | mod.rs or <file_name>.rs | depending on which rust`2015` or `2018` |

- The file system management for Managing projects can be valid for both
`libraries` or `modules`. It is a way to tell your complier (by the complier
design) to look into these places and get the programming elements
(`variables`, `methods`, `functions` ...etc.). The files above like `init.lua`
or `__init__.py` are designed specifically to tell your complier to look into
these places (directories, files or within the script file itself) by storing
the `programming elements` declarations in them not the definitions.

- The real purpose is to tell your complier to look into this folder and fetch
  all the files and stack them on top of each other to become a single file
  `single transliation unit` as they call it in `C++`. In `rust`, Then you can
  use after the other syntax for each language. For example, in rust, you can
  refer to your inner files as `self`, the root directory as `crate` and
  calling within same parent directory `super`

- In `python +3.6` Btw, in newest python versions (you don’t need to implement
  __init__.py ) anymorer.These files that I mentioned __init__py, init.lua and
  header_files.h , mod.rs All are files to store functions, variables,
  constants ..etc. declarations not definitions Which means
  - It tell your complier, hey trust me this function is existed when you
    compile or run, and take my word for it, the compiler will believe you but
    will not go to the definition to really check the body of the function for
    example, until you go to compilation stage

- Although its weird to say this for scripting language like lua, or python but
  same concept, really You have a system runtime path environment variable
  `RUNTIME-PATH` which has a list of all the places to look into. These files
  that I stated just will append to this list and tell to look there, (but you
  can see they are similar concept is applied)

- In `C++`, Why I was confused before, that some people will put their declaration and
  definition in the header file which is not wrong, but defy the purpose of the
  header file. One thing I found so amazing for example, if you define in this
  head file a declaration to a file.cpp definition, before it will go to bring
  you the directory share same name. I found that also common among Rust and
  C++

- Another point which is super important, the starting point of your project,
  for example in `Rust` the starting point is called `crate root` which can be
      `main.rs`, `src/main.rs` or `src/lib.rs`, to know the starting point
      means, you can contorl where you can fire your complier and what your
      complier will see.

- Understanding these information can also help with implementing `relative`
  and `absolute` import (`bring to the scope`) any function, variables  …etc.
  to your main function


## Things to do

- [ ] Structure the information with references and examples for each
  programming language.
- [ ] Implmenting the `rust` library that can see the files that the complier
  can see. (`check lets go rusty - modules and crates`)
- [ ] implementing other programming language.
- [ ] support the file naming system in `java` for example.



## REFERENCES
- [A Guide to Porting C/C++ to Rust](https://locka99.gitbooks.io/a-guide-to-porting-c-to-rust/content/)


