<div align="center">

<br>

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/eloi-menaud/c14/refs/heads/main/rsc/dark-banner.png" height="80">
  <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/eloi-menaud/c14/refs/heads/main/rsc/light-banner.png" height="80">
  <img alt="Shows a black logo in light color mode and a white one in dark color mode." src="" height="80">
</picture>

<br>

c14 (carbone 14), an auto semantic version calculation based on conventional commits

<pre>c14 version &lt;target></pre>

[conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) · [semantic version](https://semver.org/lang/fr/)

</div>

<br><br>

### `c14 <target> [flags]`
calcul the 'semantic version' of the specified target, based on his 'conventinal commit' commit history

<br>

### 🏳️ Flags
#### action
- `--check <full commit id>` : if use this flag, it will only check if the provided commit `conventional commit` and exit 0 if it is, 1 else
- `--check-msg <message>` : if use this flag, it will only check if the provided message (string) is `conventional commit` compatible and exit 0 if it is, 1 else
#### version
- `--branch <branch name>` : use the specified branch instead of the current one
- `--not-strict` : if the most recent commit is not `conventional commits`, it will just skip it for version calculation instead of throwing an error
- `--from <full commit id>` : start version calculation from a specific commit id
- `--base-version <x.y.z version>` : use the provided version as base version on wich increment the version regarding commit history
- `--release` : creation of a git release after the version calculation. to do a classicl release based on repo version use the `.`  target
- `--report` : display the commit history with details on wich increment impact version
#### conventional commit
- `--force-exclamation` : force using `!` as breaking change marker
- `--no-exclamation` : only use the `BREACKING CHANGE:`/`BREACKING-CHANGE:` footer as breaking changes indicator, don't look for `!`
- `--force-standard-type` : force type (different from `feat` and `fix`) to be one of `build`, `chore`, `ci`, `docs`, `style`, `refactor`, `perf`, `test`
- `--force-scope` : force using a scope
#### semantic version
- `--pre-release <pre-release>` : add the pre-release suffix to the calculated version, `<pre-release>` must follow semantic version `9.` rule
- `--build-metadata <build-metadata>` : (in addition of `--pre-release`) add the build-metadata suffix to the calculated version, `<build-metadata>` must follow semantic version `10.` rule
