# figma-style-exporter

[![CircleCI](https://circleci.com/gh/yuta24/figma-style-exporter.svg?style=svg)](https://circleci.com/gh/yuta24/figma-style-exporter)

**This project is under development.**

## ToDo

- [x] Implement command line arguments and improve output
- [ ] Use custom template
- [ ] ~Implement `generate` sub command~
- [ ] Unit test
- [ ] Refactoring
- [ ] Update README
- [ ] If rust command can call from Javascript, make Chrome Extension

## Usage

### Figma Personal Access Token

To use `figma-style-exporter`, you need to get a Figma personal access token. When using `figma-style-exporter`, you set `FIGMA_ACCESS_TOKEN` env var.

```
$ export FIGMA_ACCESS_TOKEN="..."
```

### Example

```
$ figma-style-exporter --style-type text --team-id <team-id>
```

Use --help to see usage information.
