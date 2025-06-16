<div align="center">

<br><br>

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/eloi-menaud/c14/refs/heads/main/rsc/dark-banner.png" height="80">
  <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/eloi-menaud/c14/refs/heads/main/rsc/light-banner.png" height="80">
  <img alt="Shows a black logo in light color mode and a white one in dark color mode." src="" height="80">
</picture>

<br><br>

c14 (carbon 14), an auto repo/file/dir version calculator based on _[conventional commits](https://www.conventionalcommits.org/en/v1.0.0/)_
</div>

<br><br><br>

## Install
```shell
wget https://github.com/eloi-menaud/c14/releases/download/v1.0.0/c14
mv c14 /usr/local/bin && chmod +x /usr/local/bin/c14
```
## Description
Default usage :
```shell
version=$(c14)
git tag $version && git push --tags
````

By default, `c14` calculates the version difference between HEAD and the latest tag matching the pattern `vX.Y.Z` or `vX.Y.Z-xxxxx`

To compute the version increment, it parses the commit messages following the _[conventional commits](https://www.conventionalcommits.org/en/v1.0.0/)_ standard.

Commits that do not follow the standard are ignored when calculating the version.

Valid commits trigger version bumps according to their types:
```text
Breaking changes  : +v1.0.0
fix               : +v0.0.1
feat              : +v0.1.0
```

# Usage
```text
Usage: c14 [OPTIONS]

Options:
      --from-merge-base <branch>  Use 'git merge-base HEAD <branch>'
                                  instead of the last tag
                                  useful for checks during Merge/Pull Requests

      --change-log <path>         Add changes at the top of a markdown changelog file

      --report                    Creat a c14-report.json report

      --strict                    Exit with code 1 if a commit used doesn't follow the conventional format

      --target <path>             Compute version only based on commits affecting the given file/dir path

  -h, --help                      Print help
  -V, --version                   Print version

```

<br><br><br>

### `--report` Output Format Example
```json
{
  "from": "f7b950c0ee416bb00a0179d264f0b8c1fbd60a44",
  "to": "fdadd58ace5440b9defc944fa88ea1dacdd80553",
  "target": null,
  "version": "v0.2.0",
  "commits": [
    {
      "msg": "feat: init\n",
      "id": "fdadd58ace5440b9defc944fa88ea1dacdd80553",
      "convcom": {
        "type_": "feat",
        "scope": null,
        "description": "init",
        "body": null,
        "footers": [],
        "breaking_change": false
      }
    }
  ]
}
```
### `--change-log` Output Format Example
```md
# v1.2.0
### Breaking Changes
- breaking change commit description
### Feats
- feat commit description
### Fixes
- fix 1 commit description
- fix 2 commit description
```