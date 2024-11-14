<div align="center">

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/eloi-menaud/c14/refs/heads/main/rsc/dark-banner.png">
  <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/eloi-menaud/c14/refs/heads/main/rsc/light-banner.png">
  <img alt="Shows a black logo in light color mode and a white one in dark color mode." src="">
</picture>

[conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) · [semantic version](https://semver.org/lang/fr/)

</div>

```
c14 version [target]    # return version of the target regarding its commit
```

<br><br><br><br>

# `c14 version`
`c14 version [target]`

will calculate a version for the specific `[target]` (directory or file in the repo) regarding all commit impacting it

> `[target]` is optional, if not specified it will calculat the version for `.` directory

## flags
- `--only-check <commit id>` : will just check if the provided commit is `conventional commit` or not
- `--branch <branch name>` : use the specified branch instead of the current one
- `--from <commit id>` : start commit history from a specific commit id
- `--not-strict` : if the last commit message is not `conventional commits` compatible, will just skip it instead of throwing error
- `--allow-unstandard-types` : types other than `feat` and `fix` can be others than `build` `chore` `ci` `docs` `style` `refactor` `perf` `test` (in accord to conventional commits rules _14._)
- `--allow-no-secondary-types` : make the usage of types optionnal in no no feat/fix commit (in accord to conventional commits rules _14._)
- `--no-breaking-change-footer` : don't use `BREAKING CHANGE` key footer (only use the `!` mark)


<br><br><br><br>

# 📋 specification
Commit message must follow [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/). but `c14` differs slightly :

#### if `--no-breaking-change-footer` used:

13. : _If included in the type/scope prefix, breaking changes MUST be indicated by a ! immediately before the :. ~~If ! is used, BREAKING CHANGE: MAY be omitted from the footer section, and the commit description SHALL be used to describe the breaking change.~~_

#### if `--allow-no-secondary-types` not used :

14. _Types other than feat and fix ~~MAY~~ MUST be used in your commit messages_

---

#### if `--allow-unstandard-types` not used :

14. _Types other than feat and fix MAY be used in your commit messages._ And MUST be one of the following : build, chore, ci, docs, style, refactor, perf, test.
- there is no rules about their signifaication but here some common definition :
	- `build:` editing compilation, deployment, dependencies ...
		> configure TypeScript for stricter type checking
	- `chore:` project task stuff (e.g. .gitignore)
		> update .gitignore to exclude temp files
	- `ci:` editing ci parts
		> add caching to reduce build time on Travis CI
	- `docs:` changes the documentation
		> typo in README.md
	- `style:` for code appearance improvements, e.g. convention, trailing space... (no production code change)
		> remove trailing spaces
	- `refactor:` change code structure (no production code change)
		> use snake_case instead of camelCase
	- `perf:` Use for performance improvements (no production code change)
		> optimize sql query
	- `test:` editing tests
		> add unit tests for user authentication logic
