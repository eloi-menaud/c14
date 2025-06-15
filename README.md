<div align="center">

<br>

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/eloi-menaud/c14/refs/heads/main/rsc/dark-banner.png" height="80">
  <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/eloi-menaud/c14/refs/heads/main/rsc/light-banner.png" height="80">
  <img alt="Shows a black logo in light color mode and a white one in dark color mode." src="" height="80">
</picture>











c14 (carbon 14), an auto version calculator based on _[conventional commits](https://www.conventionalcommits.org/en/v1.0.0/)_
</div>

<br><br><br>

```text
Usage: c14 [OPTIONS]

Options:
      --from-merge-base    Use 'git merge-base HEAD <default-branch>'
                           instead of the last tag
                           useful for checks during Merge/Pull Requests
      --change-log <path>  Add changes at the top of a markdown changelog file
      --json-report        Creat a c14-report.json
      --strict             Exit with code 1 if a commit used doesn't follow the conventional format
      --target <path>      Compute version only based on commits affecting the given file/dir path
  -h, --help               Print help
  -V, --version            Print version
```
