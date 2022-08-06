---
title: "grepfrog"

description: ""grepfrog" is a command that aggregates file modification commands."
theme_version: '2.8.2'
cascade:
  featured_image: '/static/images/grepfrog.jpg'
---

[![build_products](https://github.com/kyoji63/grepfrog/actions/workflows/build.yaml/badge.svg)](kyoji63/grepfrog/actions/workflows/build.yaml)
[![Coverage Status](https://coveralls.io/repos/github/kyoji63/grepfrog/badge.svg?branch=main)](https://coveralls.io/github/kyoji63/grepfrog?branch=main)
[![License](https://img.shields.io/badge/License-MIT-green)](https://github.com/kyoji63/grepfrog/blob/main/LICENSE)
[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/kyoji63/grepfrog)](https://rust-reportcard.xuri.me/report/github.com/kyoji63/grepfrog)

<img src="https://user-images.githubusercontent.com/90143019/165062158-0bee35a4-c7b8-4797-8568-5b3570137c4f.png" width="320px">


# Description
データセットファイル中のNull値を持つレコードのNull値の数を出力する
# Useage
```sh
USAGE:
    grepfrog [OPTION]　<File>
ARGS:
  <File> Path to data file
OPTIONS:
    -c, -change <String1> <String2>   Change <String1> to <String2> in the file.
    -e, -encode <String>              Encode to the specified character set.
    -h, -help                         Prints this message.
```

* [: Home](./)
  * [: Description](#-desctiption)
* [: Install](install)
* [: Usage](usage)
* [: About](about)
