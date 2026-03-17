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


c14 automates your versioning process by analyzing your Git commit history (based on _[conventional commits](https://www.conventionalcommits.org/en/v1.0.0/)_) to determine the next version.

By default, the tool identifies the latest version tag and evaluates all subsequent commits to calculate the appropriate version.

For more granular control, you can chose a specific starting point (instead of the last version tag) using `--from` 
and/or define a custom starting base version (instead of the version of the last version tag) with `--base-version`, allowing you to precisely recalculate a version from any stage of your project's history.


## Install
```shell
wget https://github.com/eloi-menaud/c14/releases/download/v4.0.0/c14
mv c14 /usr/local/bin && chmod +x /usr/local/bin/c14
```
<br><br>

### Default usage
To simply get the version of your repo based on the previous version tag :
```shell
c14 version
```
_If you do not have a previsous version tag, 0.0.0 will be used as base version_

<br><br>

## Usage

```text
c14 version [--from <FROM> | --base-version <BASE_VERSION> | --strict | --target <TARGET>...]
(Calculate version)
      --from <FROM>                  Where to start version calculation
                                       (by default: look at the last version tag)
      --base-version <BASE_VERSION>  The base version on wich start to increment regarding commits
                                       (by default: the value of the last version tag OR 0.0.0 if --from used )
      --strict                       Failed if a commit used for calculation doesn't follow the Convential Commit format
      --target <TARGET>...           Compute version only regarding specific dir(s) or file(s)

c14 increment <SOURCE> <INCREMENT>
(Calcutate the incrementation of a version)
  <INCREMENT>  The increment to add to source (X.Y.Z)
  <SOURCE>     The source version (X.Y.Z)

c14 parse <COMMIT_ID> [--strict]
(Parse a specific Commit)
    <COMMIT_ID>   Commit id of the commit to parse
    --strict  Failed if the commit doesn't follow the Convential Commit format

```

<br><br>

## Git snippet for common `--from`

#### Latest Tag Oid (Chronological)
```shell
git for-each-ref --sort=-creatordate --format="%(objectname)" --count=1 refs/tags
```

#### Initial Commit Oid
```shell
git rev-list --max-parents=0 HEAD
```

#### Last Commit on <branch> affecting a resources in <list of file/dir>
```shell
git rev-list -n 1 <branch>~1 -- <list of file/dir>
```

#### Get the common ancestor (merge-base) with <branch>
```shell
git merge-base HEAD <branch>
```