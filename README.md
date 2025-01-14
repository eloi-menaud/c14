<div align="center">

<br>

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/eloi-menaud/c14/refs/heads/main/rsc/dark-banner.png" height="80">
  <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/eloi-menaud/c14/refs/heads/main/rsc/light-banner.png" height="80">
  <img alt="Shows a black logo in light color mode and a white one in dark color mode." src="" height="80">
</picture>











c14 (carbon 14), an auto semantic version calculation based on _conventional commits_

<pre>
c14 version    
c14 changelog  
c14 parse      
</pre>

[conventional commits](https://www.conventionalcommits.org/en/v1.0.0/)
</div>

<br><br><br>

# Install
```bash
curl -fsSL https://github.com/eloi-menaud/c14/raw/refs/heads/main/dist/c14 -o /usr/local/bin/c14 && chmod +x /usr/local/bin/c14
```

# `c14 version`
`c14 version [flags] <revision>`

Calculate the version for the specified revision

### Flags
- `-base <version>` : The base version for incrementing (default is '1.0.0')
- `-target` : `<path to file or dir>`, Use only commit related to the specified target

<br>
<br>

# `c14 changelog`
`c14 changelog [flags] <revision>`

Generate the changelog for the specified revision

### Flags
- `-format <name>` : `md` `html` `text`, Specify the output format for the changelog (default: md)
- `-target` : `<path to file or dir>`, Use only commit related to the specified target

<br>
<br>

# `c14 parse`
`c14 parse [flags] <revision>`

Parse all commits of the specified revision to conventional commit format and return as JSON

```json
[
	{ // if commit can't be parse
		"id": "",
		"message": "",
		"tag": "",
		"isConvCom": false // if the commit is a conventional commit
	},

	{ // if commit can be parse (following conventional commit)
		"id": "",
		"message": "",
		"isConvCom": true, // if the commit is a conventional commit
		"type": "",
		"scope": "",
		"exclamation": false, // if use the '!' breaking change marker
		"description": "",
		"body": "",
		"footers": [ {"Key": "", "Value": ""} ],
		"isBreak": false // if it's a breaking change commit
	}
]
```
#### flags
- `-target` : `<path to file or dir>`, Use only commit related to the specified target
- `-strict` : Exit with status 1 if any commit does not adhere to the conventional commit format
- `-spec` : Display details (format, examples, links, regex, etc.) about the conventional commit format and exit

# Global Flags
- `-help` : Display help and exit
- `-v` : Enable verbose mode