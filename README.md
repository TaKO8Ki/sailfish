<div align="center">

![SailFish](./resources/logo.png)

Simple, small, and extremely fast template engine for Rust

[![Build Status](https://travis-ci.org/Kogia-sima/sailfish.svg?branch=master)](https://travis-ci.org/Kogia-sima/sailfish)
[![Build status](https://ci.appveyor.com/api/projects/status/fa3et4rft4dyvdn9/branch/master?svg=true)](https://ci.appveyor.com/project/Kogiasima/sailfish/branch/master)
[![Version](https://img.shields.io/crates/v/sailfish)](https://crates.io/crates/sailfish)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/Kogia-sima/sailfish/blob/master/LICENSE)

[User Guide](https://sailfish.netlify.app/en/) | [API Docs](https://docs.rs/sailfish) | [Examples](./examples)

</div>

## ✨ Features

- Simple and intuitive syntax inspired by [EJS](https://ejs.co/)
- Relatively small number of dependencies (<15 crates in total)
- Extremely fast (See [benchmarks](./benches/README.md))
- Better error message
- Template rendering NEVER fails unless you explicitly return error.
- Syntax highlighting support ([vscode](./syntax/vscode), [vim](./syntax/vim))
- Automatically re-compile sources when template file is updated.
- Works on Rust 1.42 or later

## 🐟 Example

Dependencies:

```toml
[dependencies]
sailfish = "0.1.3"
sailfish-macros = "0.1.3"
```

Template file (templates/hello.stpl):

```erb
<html>
  <body>
    <% for msg in &messages { %>
      <div><%= msg %></div>
    <% } %>
  </body>
</html>
```

Code:

```rust
#[macro_use]
extern crate sailfish_macros;  // enable derive macro

use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "hello.stpl")]
struct HelloTemplate {
    messages: Vec<String>
}

fn main() {
    let ctx = HelloTemplate {
        messages: vec![String::from("foo"), String::from("bar")]
    }
    println!("{}", ctx.render_once().unwrap());
}
```

You can find more examples in [examples](./examples) directory.

## 🐾 Roadmap

- `Template` trait ([RFC](https://github.com/Kogia-sima/sailfish/issues/3))
- Template inheritance (block, partials, etc.)
- Whitespace suppressing
- Dynamic template compilation ([RFC](https://github.com/Kogia-sima/sailfish/issues/1))
- `format_templates!(fmt, args..)` macro

## 👤 Author

🇯🇵 **Ryohei Machida**

* Github: [@Kogia-sima](https://github.com/Kogia-sima)

## 🤝 Contributing

Contributions, issues and feature requests are welcome!

Since sailfish is an immature library, there are many [planned features](https://github.com/Kogia-sima/sailfish/labels/Type%3A%20RFC) that is on a stage of RFC. Please leave a comment if you have an idea about its design!

Also I welcome any pull requests to improve sailfish! Find issue with [Status: PR Welcome](https://github.com/Kogia-sima/sailfish/issues?q=is%3Aissue+is%3Aopen+label%3A%22Status%3A+PR+Welcome%22) label, and [let's create a new pull request](https://github.com/Kogia-sima/sailfish/pulls)!

## Show your support

Give a ⭐️ if this project helped you!

## 📝 License

Copyright © 2020 [Ryohei Machida](https://github.com/Kogia-sima).

This project is [MIT](https://github.com/Kogia-sima/sailfish/blob/master/LICENSE) licensed.

***
_This README was generated with ❤️ by [readme-md-generator](https://github.com/kefranabg/readme-md-generator)_
