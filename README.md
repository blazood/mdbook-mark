# mdbook-mark
this is a mdbook preprocessor which can render ==== to &lt;mark>&lt;/mark> 


## quick start

### install mdbook-mark

```shell script
cargo install mdbook-mark
```

### configure book.toml

```toml
[book]
authors = ["blazh"]
language = "en"
multilingual = false
src = "src"
title = "test"

# add into your book.toml
[preprocessor.mark]
command="mdbook-mark"
```