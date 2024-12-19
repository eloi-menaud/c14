package action

import (
	"c14/utils"
	"c14/utils/Log"
	"fmt"
	"strings"
)



func Version(revision string, base string) error {

	err := utils.CheckRevision(revision)
	if err != nil {
		return err
	}


	Log.Info("Revision : %s",revision)


	// getting commits
	raw_commits, err := utils.GetCommits(revision)
	if err != nil {
		return fmt.Errorf("failed to fetch commit history : %v",err)
	}
	Log.Verb("git log parsed (%d commits)", len(raw_commits))


	Log.Verb("parsing raw commit to conventional commits :",)
	var commits []utils.Commit
	// parsing to conventional commits
	for _,commit := range raw_commits{
		commit,err := utils.NewCommit(commit)
		if err != nil {
			Log.Verb("  x %s | wrong format", commit.Id)
		} else {
			Log.Verb("  ✓ %s | type : %s", commit.Id, commit.Type)
		}
		commits = append(commits, commit)
	}



	// calculate version
	version,err := utils.NewVersion(base)
	if err != nil {
		return fmt.Errorf("failed to create base version : %v",err)
	}

	var detail string


	for i := len(commits)-1; i >= 0; i-- {

		if commits[i].IsBreak {
			version.BumpMajor()
		} else if commits[i].Type == "feat" {
			version.BumpMinor()
		} else if commits[i].Type == "fix" {
			version.BumpPatch()
		}

		detail += detailLine(commits[i],version.String())
	}

	Log.Info("─── details ───\n%s", detail)


	Log.Res(version.String())
	return nil
}



func detailLine(commit utils.Commit, version string) string {
	var convcom_marker string
	if ! commit.IsConvCom {
		convcom_marker = "x"
	} else {
		convcom_marker = "✓"
	}
	var preview string
	if lines:=strings.Split(commit.Message, "\n"); len(lines) > 0{
		l   := lines[0]
		max := 30
		if len(lines[0]) > max {
			l = l[:max-3] + "..."
		}
		preview = l + strings.Repeat(" ", max-len(l))
	}

	return fmt.Sprintf("%s - %s %s (%s)\n", version, preview,convcom_marker, commit.Id[:7])
}