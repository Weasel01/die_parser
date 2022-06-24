# 🎲 die_parser
[<img alt="github" src="https://img.shields.io/badge/github-Weasel01/die__parser-8da0cb?style=labelColor=555555&logo=github" height="20">](https://github.com/Weasel01/die_parser)
[<img alt="crates.io" src="https://img.shields.io/crates/v/die_parser.svg?style=color=fc8d62&logo=rust" height="20">](https://crates.io/crates/die_parser)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-die__parser-66c2a5?style=labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/die_parser)
[![License](https://img.shields.io/crates/l/die_parser)](https://choosealicense.com/licenses/apache-2.0/)

This crate parses the notation for die rolls as used in tabletop games like D&D.

It aims to do so in the most *simple*, *easy* and *lightweight* way possible.

     Input:
     1.) "2d6"         (Roll 2 six-sided dice.)
     2.) "4d20 - 5"    (Roll 4 twenty-sided dice and subtract 5 from the result.)

     Output:
     1.)    Roll {
             number_of_sides: 6
             number_of_dice: 2
             modifier: 0
            }
     2.)    Roll {
             number_of_sides: 20
             number_of_dice: 4
             modifier: -5
            }

## ❓ Getting started:
* Try *Roll::parse_roll()* !
### 📖 Documentation:
* [docs.rs/die_parser](https://docs.rs/die_parser)
### ☕ Buy me a Coffee:
If you like this crate, you can support my work here:
* [Ko-Fi](http://ko-fi.com/fbeizai)
