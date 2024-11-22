
# license

A cli to easily add licenses to your project (add a LICENSE file to the 
directory the terminal is currently in).

The program will take from a directory at
the executable's path called ```licenses```. 

The licenses source is 
taken from [https://choosealicense.com/](https://choosealicense.com/).

<hr>

For licenses like MIT where there is a place to enter your name, date, 
etc, wrap it in braces.

```text
MIT License

Copyright (c) [year] [fullname]

...
```

The program will then prompt the user for a value to replace the 
bracketed values or not replace at all if nothing is provided.

# Usage

## Add

```text
> license add MIT
Enter nothing if it shouldn't be changed.
[year] = 2024

Enter nothing if it shouldn't be changed.
[fullname] = Nvt5
```

The LICENSE file will end up like this.

```text
MIT License

Copyright (c) 2024 Nvt5

...
```

## Select

This will add the MIT license and will end up the same as the above add
command.

```text
> license select
Enter the index of the license to add.
0: Boost Software License 1.0
1: ISC
2: MIT
3: Mozilla Public License 2.0
4: Unlicense
Index: 2

Enter nothing if it shouldn't be changed.
[year] = 2024

Enter nothing if it shouldn't be changed.
[fullname] = Nvt5
```

## List

```text
> license list
Boost Software License 1.0
ISC
MIT
Mozilla Public License 2.0
Unlicense
```
