package utils

import "fmt"


func GetSpecification() string {
	return fmt.Sprintf(`
commit must follow 'conventional commit' specification
  See : https://www.conventionalcommits.org/en/v1.0.0/#specification

Format :
  <type>[optional scope][optional !]: <description>

  [optional body]

  [optional footer(s)]

Example :
	feat(api)!: add new endpoint for user registration

	Introduce a new '/register' endpoint to allow users to create an account.
	This includes input validation and email verification features.

	BREAKING CHANGE: The previous '/signup' endpoint has been removed.

Regex to match :
%s
`,Regexp_convcom)
}


func GetRevisionHelpMessage()string{
	return `classical revision to use :

	- All commits from root
	  $(git rev-list --max-parents=0 HEAD)..HEAD

	- All commits since last tag
	  $(git describe --tags --abbrev=0)..HEAD

	- Last commit
	  HEAD

	- commits that are in branch Y but not in branch X
	  Y..X
`
}