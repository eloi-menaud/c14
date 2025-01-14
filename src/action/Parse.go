package action

import (
	"c14/utils"
	"c14/utils/Log"
	"fmt"
	"strings"
)

func Parse(revision string, strict bool, target string) error {

	// check revision
	err := utils.CheckRevision(revision)
	if err != nil {
		return err
	}


	// getting commits
	raw_commits, err := utils.GetCommits(revision, target)
	if err != nil {
		return fmt.Errorf("failed to fetch commit history : %v",err)
	}
	Log.Verb("git log parsed (%d commits)", len(raw_commits))


	var wrong_format_commits []utils.RawCommit

	Log.Verb("parsing raw commit to conventional commits :",)
	var commits []utils.Commit
	// parsing to conventional commits
	for _,raw_commit := range raw_commits{
		commit,err := utils.NewCommit(raw_commit)
		if err != nil {
			wrong_format_commits = append(wrong_format_commits, raw_commit)
			Log.Verb("  x %s | wrong format", commit.Id)
		} else {
			Log.Verb("  ✓ %s | type : %s", commit.Id, commit.Type)
		}
		commits = append(commits, commit)
	}


	if len(wrong_format_commits) != 0 {
		message := "using strict mode, following commits are not correctly formatted :\n"
		for _,raw_commit := range wrong_format_commits {
			message = message + fmt.Sprintf(
				"[%s]\n%s\n",
				raw_commit.Id,
				strings.ReplaceAll(raw_commit.Message, "\n", "\n  "),
			)
		}
		if strict {
			return fmt.Errorf("%s",message)
		} else {
			Log.Info("%s",message)
		}
	}


	var res string

	for i := len(commits)-1; i >= 0; i-- {
		res += commits[i].Json() + ",\n"
	}
	res = strings.TrimRight(strings.TrimSpace(res), ",")

	Log.Info("")
	Log.Res("[\n%s\n]",res)

	return nil
}


