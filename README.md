# ocdiff

Fast diff library for Python - wraps [similar](https://crates.io/crates/similar).

# Usage

```shell
pip install ocdiff
```

```python
ocdiff.html_diff(
    a,
    b,
    context_lines=5,
    max_total_width=80,
)
```

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

- Use console width stuff
- Add console output with colours.
- Add pytest plugin magic that plays with `rich`.
- Write some docs.
