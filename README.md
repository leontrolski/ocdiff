# ocdiff

Fast diff library for Python - wraps [similar](https://crates.io/crates/similar).

# Install/Develop

```shell
uv pip install -e '.[dev]'
maturin develop
```

# TODO

- Implement `column_limit` - should just be a function of `Vec<LinePartsDiff>` to `Vec<LinePartsDiff>`.
- Add console output with colours.
- Add pytest plugin magic that plays with `rich`.
