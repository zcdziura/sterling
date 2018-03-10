# Sterling

Converts a given D&D 5e currency value to the Silver Standard. Inspired by the Reddit posts titled
[The Silver Hack: Making Money Matter](https://www.reddit.com/r/DnDBehindTheScreen/comments/80f6kt/the_silver_hack_making_money_matter/),
and [I make Silver Standard for 5th Edition (Spreadsheets.)](https://www.reddit.com/r/dndnext/comments/5tt5g8/i_make_silver_standard_for_5_edition_spreadsheets/).

## Usage

```
sterling [VALUE]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <VALUE>...    The value to be converted; should be suffixed with the coin's short-hand
                  abbreviation, i.e. p, g, e, s, or c. Defaults coin type to 'g'.
```

## Examples

```
// Convert one hundred platinum coins: 
sterling 100p // 10g

// Convert one hundred platinum, fifty gold coins:
sterling 100p 50g // 10g 5s

// Convert fifteen thousand copper coins:
sterling 15000c // 1g 50s

// Convert one platinum, thirty-six gold, twelve electrum, eighty-two silver, and four hundred
// sixty-nine copper coins
sterling 1p 36g 12e 82s 469c // 64s 89c
```

## Abstract

Items and expenses are, by default, assigned arbitrary currency values within the official D&D 5th
edition source books. Many of the officially priced items use the "Gold Standard"; that is, items
are priced in gold coins by default. While there is nothing wrong with using official currency
values within your campaign, it leads to the perceived value of gold to be less in the eyes of your
players. Gold has been sought after as both a commodity and a currency for centuries, and your
campaign aught to treat gold similarly!

## Explanation

The basis of the Silver Standard treats 1 gold coin from the official D&D 5e source books as 1
silver coin, and that there are one hundred of a given coin to every one of the next highest valued
coin. That's all. Thus, one hundred fifty copper coins equals one silver and fifty copper coins,
while a suit of heavy plate armor equals fifteen gold coins, rather than fifteen hundred.

## Installation

Make sure that you first have `rust` and `cargo` installed onto your computer before downloading
`sterling`. Just follow the simple
[Installation Guide](https://doc.rust-lang.org/cargo/getting-started/installation.html) on the
official Rust language website to install both programs.

Once `rust` and `cargo` are installed onto your computer, run the following command:

`cargo install sterling`

This will install `sterling` into the `.cargo/bin` directory within your User directory
(`/home/YOUR_USER_NAME` on Linux and macOS, `C:\Users\YOUR_USER_NAME` on Windows). Be sure to add
this directory to your PATH.