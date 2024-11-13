package commit

import (
	"fmt"
	"regexp"
	"strings"

	"c14/Log"
)

const(
	regexp_extract_commit_id = `commit ([a-z0-9]{40})`
	regexp_extract_commit_msg = `(?m)^    (.+)`
	regexp_covcom = `^(build|chore|ci|docs|feat|fix|perf|refactor|revert|style|test){1}(\([\w\-]+\))?(!)?: .{1,80}(\n|\r\n){2}(.*(\n|\r\n)*)*$`
)

type Commit struct{
	Id        string
	Type      string
	Scope     string
	Descr     string
	Body      string
	IsBreak   bool
	IsConvCom bool
}

func New(id string) (Commit,error){
	res := Commit{}

	if len(id) != 7 && len(id) != 40{
		return Commit{}, fmt.Errorf("failed to create a new Commit : given id must have 7 characters (short id) or 40 characters (full id). but received '%s'", id)
	}

	Log.Verb("fetching '%s' commit", id)

	raw_commit, _, err := GitCmd("log",id,"-1")
	if err != nil {
		return Commit{}, fmt.Errorf("failed to create a new Commit : %s",err)
	}

	Log.Verb("raw commit data:\n%s",raw_commit)

	if len(id) == 7 {
		Log.Verb("given commit id is short, parsing raw commit to retreive the full one")
		id_match := regexp.MustCompile(regexp_extract_commit_id).FindStringSubmatch(raw_commit)
		if len(id_match) == 0 {
			return Commit{}, fmt.Errorf("failed to retrive full id from raw commit data, raw commit data:\n%s",raw_commit)
		}
		res.Id = id_match[1]
		Log.Verb("full commit id : %s", res.Id)
	} else {
		res.Id = id
	}

	Log.Verb("extracting th commit message from raw commit data")


	var msg string
	msg_line_matchs := regexp.MustCompile(regexp_extract_commit_msg).FindAllStringSubmatch(raw_commit,-1)
	for _,msg_line := range msg_line_matchs{
		msg += msg_line[1] + "\n"
	}
	msg = strings.Trim(msg,"\n")
	Log.Verb("extracted commit message : {\n%s\n}",msg)

	convcom_match := regexp.MustCompile(regexp_covcom).FindStringSubmatch(msg)
	if len(convcom_match) == 0 {
		Log.Verb("/!\\ commit '%s' is not conventional commit compatible, commit message :{\n%s\n}",res.Id,msg)
		res.IsConvCom = false
		res.Body      = msg
	} else {
		res.IsConvCom = true
		res.Type      = convcom_match[1]
		res.Scope     = convcom_match[2]
		res.IsBreak   = (convcom_match[3] == "!" || res.Type == "break")
		res.Descr     = convcom_match[4]
		res.Body      = convcom_match[6]
	}


	return res,nil
}