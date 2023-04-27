# pwgen
Random password generator for the terminal.

## How to use

Run the following command on your terminal.

```sh
pwgen FAMILIES [LENGTH]
```

## Options

- `FAMILIES`

  List of character families to be used.
  This must be given as a string consisting of these characters:
  - `a` (lowercase alphabets)
  - `A` (uppercase alphabets)
  - `n` (numbers)
  - `s` (symbols)

- `LENGTH`

  Length of the password.
  (default: 15)
 
## Examples

Generate a 12-character long password containing lowercase alphabets,
uppercase alphabets and numbers:
```
pwgen aAn 12
```

Generate a password containing only lowercase alphabets and numbers:
```
pwgen an
```
