package utils

import (
	"c14/utils/Log"
	"encoding/json"
	"fmt"
	"regexp"
	"strings"

	"gopkg.in/yaml.v2"
)

const(
	git_log_json_format = `--pretty=format:{"id": "%H","message":"%B","tag":"%(describe:tags=true)"},`
)




type RawCommit struct {
	Id          string   `json:"id"`
	ShortId     string   `json:"-"`
	Message     string   `json:"message"`
	Tag         string   `json:"tag,omitempty"`
}
func (c RawCommit) String() string {
	parsed,_ := yaml.Marshal(c)
	return strings.TrimSpace(string(parsed))
}



func CheckRevision(revision string) error {
	_, _, err := Cmd("git","rev-parse", revision)
	if err != nil {
		return fmt.Errorf("revision '%s' isn't valid", revision)
	}
	return nil
}



func GetCommits(revision string, target string) ([]RawCommit, error) {

	// build git command args depending of config.config
	args := []string{"--no-pager", "show", "--no-patch", git_log_json_format}

	if revision != ""{
		args = append(args, revision)
	}

	if target != ""{
		args = append(args, "--", target)
	}
	Log.Verb("target: %s", target)

	Log.Verb("%v",args)
	// executing command
	stdout, stderr, err := Cmd("git",args...)
	stdout = strings.ReplaceAll(stdout, "\n", "\\n")
	stdout = strings.ReplaceAll(stdout, ",\\n", ",")
	if err != nil {
		return nil, fmt.Errorf("failed to fetch git history : %s", CmdResToString(stdout,stderr,err))
	}
	stdout =  "[\n" + strings.TrimSuffix(stdout, ",") + "\n]"

	Log.Verb("git log result:\n%s",stdout)


	// parsing
	var commits []RawCommit
	err = json.Unmarshal([]byte(stdout), &commits)
	if err != nil {
		return nil,fmt.Errorf("failed to parse json git log history to list of commit, json git log history :\n%s\nerror : %v", stdout, err)
	}

	// cleaning :
	//  add manually ShortId
	//  remove trailing space of body
	//  remove implicit subtag like {tag}-{nb}-g{short hash}
	for idx := range commits{
		commits[idx].ShortId = commits[idx].Id[:7]
		commits[idx].Message = strings.TrimSpace(commits[idx].Message)
		if regexp.MustCompile(`.+-\d+-g[\w\d]{7}`).MatchString(commits[idx].Tag){
			commits[idx].Tag = ""
		}
	}

	return commits, nil
}

