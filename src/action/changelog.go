package action

import (
	"c14/utils"
	"c14/utils/Log"
	"fmt"
)



func Changelog(format string, revision string, target string) error {

	if  (format == "") ||
		((format != "md") && (format != "html") && (format != "text")){
		return fmt.Errorf("format '%s' is not valid, you must specify one : md, html, text",revision)
	}

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

	// generating changelog
	changelog := utils.NewChangeLog(commits)

	switch format{
	case "text" :
		res,err := changelog.Text(target)
		if err != nil {
			Log.Fatal("failed to build '%s' changelog : %v",format, err)
		}
		Log.Res("%s",res)
	case "md" :
		res,err := changelog.Md(target)
		if err != nil {
			Log.Fatal("failed to build '%s' changelog : %v", format, err)
		}
		Log.Res("%s",res)
	case "html" :
		res,err := changelog.Html(target)
		if err != nil {
			Log.Fatal("failed to build '%s' changelog : %v", format, err)
		}
		Log.Res("%s",res)
	}

	return nil
}