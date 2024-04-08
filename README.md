# ocdiff

Fast diff library for Python - wraps [similar](https://crates.io/crates/similar).

# Install/Develop

```shell
uv pip install -e '.[dev]'
maturin develop
```

# Make release

- Add pypi token and user = `__token__` to settings (do this once).
- Upversion `pyproject.toml`.

```shell
git tag -a v0.0.x head -m v0.0.x
git push origin v0.0.x
```

# TODO

- Implement `column_limit` - should just be a function of `Vec<LinePartsDiff>` to `Vec<LinePartsDiff>`.
- Add console output with colours.
- Add pytest plugin magic that plays with `rich`.
