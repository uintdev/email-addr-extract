# Email Addr Extract

<img src="banner.png" alt="Banner with logo">
<br>

<p align="center">
    High performance email address extraction tool
    <br>
    <br>
    <a href="../../releases/latest" title="Latest release"><img src="https://img.shields.io/github/v/release/uintdev/email-addr-extract" alt="Version"></a>
    &nbsp;&nbsp;
    <a href="LICENSE" title="License"><img src="https://img.shields.io/github/license/uintdev/email-addr-extract" alt="License"></a>
</p>

## About

Email Addr Extract is a tool that takes all the email addresses it finds in a file that has data between and then creates a list of emails addresses, one per line.

This is a high performance port of the original Email Extractor Python variant.

From testing, this was found to be around 14.5x faster with an almost 900MB file.

Usage: `./emailaddrextract <file to read> <output file>`

## Features

-   Collects multiple emails addresses per line
-   Removes duplicated email addresses

## Disclaimer

This is for educational and research purposes only. The tool does specifically what it was made to do and it is as simple as that. It is purely your responsibility in how it is used.
