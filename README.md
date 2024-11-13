![](./doc_rsc/c14_small.png)
# c14 (carbone 14)
auto semantic version for any target based on conventional commits
- [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) checker
- [semantic version](https://semver.org/lang/fr/) calculation for any directory/file

```
c14 version [target]    # return version of the target regarding its commit
c14 check   [commit id] # check if a commit is conventional commits compatible
```

<br><br><br><br>

# `c14 version`
`c14 version [target]`

will calculate a version for the specific `[target]` (directory or file in the repo) regarding all commit impacting it

> `[target]` is optional, if not specified it will calculat the version for `.` directory

## flags
- `--branch <branch name>` : use the specified branch instead of the current one
- `--from <commit id>` : start commit history from a specific commit id
- `--not-strict` : if the last commit message is not `conventional commits` compatible (`c14 check` failed), will just skip it instead of throwing error
- All `c14 check` flags are available


<br><br>

# `c14 check`
`c14 check [commit id]`
will check if the specific commit is `conventional commits` compatible (in a c14 way, see `specification`)
> `[commit id]` is optional, if not specified it will check the last commit
## flags
`--not-force-standard-secondary-types` : types other than `feat` and `fix` can be others than `build` `chore` `ci` `docs` `style` `refactor` `perf` `test` (in accord to conventional commits rules _14._)

`--not-force-secondary-types` : make the usage of types other than `feat` and `fix` optional (in accord to conventional commits rules _14._)


<br><br><br><br>

# specification
Commit message must follow [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/). but `c14` differs slightly :

#### [8, 9, 10, 11] _footers_
git 'footers' are not taking into accunt so :
- rules 8, 9, 10 are skiped
- rules 11 becams : _Breaking changes MUST be indicated in the type/scope prefix of a commit, ~~or as an entry in the footer~~._

#### [14] _secondary types_
secondary types (types other than `fix` and `feat`) are by default mandatory and must by standard ones (`build` `chore` `ci` `docs` `style` `refactor` `perf` `test`).

To use the original `14.` rules, use `--not-force-standard-secondary-types` and `--not-force-secondary-types` falgs
