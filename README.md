# Sterling

Converts a given D&D 5e currency value to the Silver Standard. Inspired by the
Reddit posts titled [The Silver Hack: Making Money
Matter](https://www.reddit.com/r/DnDBehindTheScreen/comments/80f6kt/the_silver_hack_making_money_matter/),
and [I make Silver Standard for 5th Edition
(Spreadsheets.)](https://www.reddit.com/r/dndnext/comments/5tt5g8/i_make_silver_standard_for_5_edition_spreadsheets/).

## Usage

```
USAGE:
    sterling [FLAGS] [OPTIONS] [VALUE]... [SUBCOMMAND]

FLAGS:
    -o, --optional    Include currencies marked as optional when converting
    -f, --full        Print currencies with their full name, rather than with their alias
    -h, --help        Prints help information
    -V, --version     Prints version information

OPTIONS:
    -c, --config <CONFIG>    Specify location of config file; defaults to './sterling-conf.yml'.

ARGS:
    <VALUE>...    The value to be converted; should be suffixed with the coin's short-hand abbreviation, i.e. p, g,
                  e, s, or c.

SUBCOMMANDS:
    add       Add two currency amounts together; uses the currencies defined in your config file
    copper    Calculate the copper value of a custom currency
    div       Divide a currency amount by some scalar divisor; uses the currencies defined in your config file
    help      Prints this message or the help of the given subcommand(s)
    mul       Multiply a scalar multiplicand by a currency amount; uses the currencies defined in your config file
    sub       Subtract two currency amounts from one another; uses the currencies defined in your config file
```

## Converting Currency Examples

```
// Convert one hundred platinum coins:
sterling 100p // 10g

// Convert one hundred platinum, fifty gold coins:
sterling 100p 50g // 10g, 50s

// Convert fifteen thousand copper coins, printing the full names of the coins:
sterling -f 15000c // 1 gold, 50 silvers

// Convert one platinum, thirty-six gold, twelve electrum, eighty-two silver, and four hundred
// sixty-nine copper coins, printing the full names of the coins
sterling --full 1p 36g 12e 82s 469c // 64 silvers, 89 coppers

// Convert one platinum, thirty-six gold, twelve electrum, eighty-two silver, and four hundred
// sixty-nine copper coins, printing the full names of the coins, using the custom config file
// detailed below, including optional currencies
sterling --full -o -c "~/Documents/D&D/sterling-conf.yml" 1p 36g 12e 82s 469c // 7 guilders, 6 sterling, 25 pence
```

## Subcommand Examples

```
// Add together ten and twenty pense, using the custom config file detailed below
sterling add "10p" "20p" // 30p

// Subtract two sterling and ten pence from one eagle
sterling --full sub "1e" "2s 10p" // 19 guilders, 25 sterling, 22 pence

// Subtract one eagle from two sterling and ten pence. Note that, regardless of order, the smaller
// value is ALWAYS subtracted from the larger value.
sterling --full sub "2s 10p" "1F" // 19 guilders, 25 sterling, 22 pence

// Multiply two sterling and ten pence by thirteen. Note that the currencies always go after the
// multiplier.
sterling --full mul 13 2s 10p // 1 guilder, 2 sterling, 2 pence

// Divide one guilder, 2 sterling, and 2 pence by thirteen.
sterling --full div 13 1g 2s 2p // 2 sterling, 10 pence

// Convert one note, three eagles, and five guilders into copper
sterling copper 1N 3e 5g // 201,600c
```

Note that `sterling` doesn't allow for negative currencies. Therefore, when
subtracting currencies, the smaller currency value will always be subtracted
from the larger currency value, regardless of the order of the currencies in the
`sub` command.

## Custom Currencies

`sterling` allows for user-defined currencies, with their own names and
conversion rates. By default, `sterling` will look at a file within the current
directory called `sterling-conf.yml`, or in whatever location as supplied by the
`-c` flag. You can also specify that a currency be optional, which will prevent
that currency from being used when converting values, unless the `-o` flag is
passed. Below is an example `sterling-conf.yml` file, showing the actual
currencies that I use within my own campaign!

```
-
  name: "note"
  rate: 143360
  alias: "N"
  optional: true
-
  name: "eagle"
  rate: 17920
  alias: "e"
-
  name: "guilder"
  rate: 896
  alias: "g"
-
  name: "shilling"
  rate: 32
  alias: "s"
  plural: "sterling"
-
  name: "penny"
  rate: 1
  alias: "p"
  plural: "pence"
```

Please note that the `rate` value is defined as the number of copper coins that
goes into one of that particular currency. In the example above, thirty-two
copper coins goes into one "shilling", and eight-hundred ninety-siz copper coins
goes into one "guilder".

## Abstract

Items and expenses are, by default, assigned arbitrary currency values within
the official D&D 5th edition source books. Many of the officially priced items
use the "Gold Standard"; that is, items are priced in gold coins by default.
While there is nothing wrong with using official currency values within your
campaign, it leads to the perceived value of gold to be less in the eyes of your
players. Gold has been sought after as both a commodity and a currency for
centuries, and your campaign aught to treat gold similarly!

## Explanation

The basis of the Silver Standard treats one gold coin from the official D&D 5e
source books as one silver coin, and that there are one hundred of a given coin
to every one of the next highest valued coin. That's all. Thus, one hundred
fifty copper coins equals one silver and fifty copper coins, while a suit of
heavy plate armor equals fifteen gold coins, rather than fifteen hundred.

## Installation

The easiest way to install `sterling` is to do so with `cargo`, the build tool
that's installed along with the `rust` compiler. If you already have `rust` and
`cargo` installed onto your computer, simply run the following command from a
command prompt:

```
$ cargo install sterling
```

If you do not have the `rust` compiler installed, you can also find pre-built
binaries for 64-bit Windows, macOS, and Linux computers in the "Tags" navigation
link, which is displayed above this README. Simply download the correct binary
for your computer's operating system, extract it somewhere into your file system
(such as a "bin" folder within your user directory), and add that location to
your system's PATH.

## License

```
Copyright (C) 2018-2022 Z. Charles Dziura

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
```
