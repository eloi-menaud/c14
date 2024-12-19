package action

import (
	"c14/utils"
	"c14/utils/Log"
	"fmt"
	"strings"
)

func Parse(revision string, strict bool) error {

	// check revision
	err := utils.CheckRevision(revision)
	if err != nil {
		return err
	}


	// getting commits
	raw_commits, err := utils.GetCommits(revision)
	if err != nil {
		return fmt.Errorf("failed to fetch commit history : %v",err)
	}
	Log.Verb("git log parsed (%d commits)", len(raw_commits))


	var wrong_format_commit_ids []string

	Log.Verb("parsing raw commit to conventional commits :",)
	var commits []utils.Commit
	// parsing to conventional commits
	for _,commit := range raw_commits{
		commit,err := utils.NewCommit(commit)
		if err != nil {
			wrong_format_commit_ids = append(wrong_format_commit_ids, commit.Id)
			Log.Verb("  x %s | wrong format", commit.Id)
		} else {
			Log.Verb("  ✓ %s | type : %s", commit.Id, commit.Type)
		}
		commits = append(commits, commit)
	}


	if len(wrong_format_commit_ids) != 0 {
		message := fmt.Sprintf(
			"following commits are not correctly formatted :\n%s\n\nuse 'c14 parse -spec' to see more about format",
			strings.Join(wrong_format_commit_ids,"\n"),
		)
		if strict {
			return fmt.Errorf("using strict mode : %s",message)
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


