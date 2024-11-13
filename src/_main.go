package main

import (
	"bytes"
	"flag"
	"fmt"
	"os"
	"os/exec"
	"regexp"
	"strings"
)


func Log(format string, ars ...interface{}){
	fmt.Fprintln(os.Stderr, "Log: Début de l'exécution de l'outil")
}


func NewCommit(raw string) (Commit,error){
	id_regexp  := `^commit (.+)`
	msg_regexp := `(?m)^    (.+)`

	id_match  := regexp.MustCompile(id_regexp).FindStringSubmatch(raw)
	if len(id_match) == 0{
		return Commit{}, fmt.Errorf("can't extract commit id in raw commit log :\n%s",raw)
	}

	msg_match := regexp.MustCompile(msg_regexp).FindAllStringSubmatch(raw, -1)
	if len(msg_match) == 0{
		return Commit{}, fmt.Errorf("can't extract commit message in raw commit log :\n%s",raw)
	}
	message := ""
	for _,line_match := range msg_match {
		message += line_match[1] + "\n"
	}
	message = strings.TrimRight(message, "\n")


	return Commit{
		id: id_match[1],
		msg: message,
	}, nil
}

func (c Commit) String() string{
	res := ""
	for i, line := range strings.Split(c.msg, "\n") {
		res += "| " + line
	}
}

func GetCommits(target string, branch string) ([]Commit,error){

	// git fetch
	Log("executing git fetch on %s\n", branch)

	cmd := exec.Command("git", "fetch","--depth=100000", "origin", branch)

	var stdout bytes.Buffer
	var stderr bytes.Buffer

	cmd.Stdout = &stdout
	cmd.Stderr = &stderr

	err := cmd.Run()

	if err != nil {
		return nil, fmt.Errorf(
			"failed to fetching commit history :\n-- stdout --\n%s\n-- stderr --\n%s\nerror : %s",
			stdout.String(), stderr.String(), err)
	}


	// get commit
	Log("executing log %s\n", branch)

	cmd = exec.Command("git", "--no-pager", "log", branch, "--", target)

	stdout.Reset()
	stderr.Reset()

	cmd.Stdout = &stdout
	cmd.Stderr = &stderr

	err = cmd.Run()

	if err != nil {
		return nil, fmt.Errorf(
			"failed to get all commits :\n-- stdout --\n%s\n-- stderr --\n%s\nerror : %s",
			stdout.String(), stderr.String(), err)
	}


	// parsing
	var commits []Commit
	for idx,raw_commit := range regexp.MustCompile("(?m)^commit").Split(stdout.String(),-1) {
		if idx == 0 { continue }
		commit, err := NewCommit("commit" + raw_commit)
		if err != nil {
			return nil, fmt.Errorf("faild to parse the commits log : %v", err)
		}
		commits = append(commits, commit)
	}


	return commits,nil
}

const(
	a_default = `^(?:BREAKING CHANGE|Breaking Change|breaking change|brkc|BRKC) *:`
	b_default = `^(?:feat|Feat|FEAT) *:`
	c_default = `^(?:fix|Fix|FIX) *:`
	d_default = ""
	e_default = ""
)


func calcul_single(
	commit Commit,
	regs [5]*regexp.Regexp,
) [5]int {
	res := [5]int{}
	for idx,reg := range regs {
		if reg.MatchString(commit.msg) {
			res[idx] = 1
			return res
		}
	}
	return res
}


func main(){
	target_info := "the target to follow for calculating version"
	target := flag.String(
		"target",
		"",
		target_info)

	a := flag.String(
		"a",
		a_default,
		"specify the 'a' prefix regex\n(the regex to be used to increment the 'a' part (a.b.c.d.e) of the version when it matches)")

	b := flag.String(
		"b",
		b_default,
		"specify the 'b' prefix regex\n(the regex to be used to increment the 'b' part (a.b.c.d.e) of the version when it matches)")

	c := flag.String(
		"c",
		c_default,
		"specify the 'c' prefix regex\n(the regex to be used to increment the 'c' part (a.b.c.d.e) of the version when it matches)")

	d := flag.String(
		"d",
		d_default,
		"specify the 'd' prefix regex\n(the regex to be used to increment the 'd' part (a.b.c.d.e) of the version when it matches)")

	e := flag.String(
		"e",
		e_default,
		"specify the 'e' prefix regex\n(the regex to be used to increment the 'e' part (a.b.c.d.e) of the version when it matches)")

	strict := flag.Bool(
		"strict",
		false,
		"if set, force the most recent commit to match 1 of the prefixes (a,b,c,d or e), to make sure all commits are 'semantic'")

	prefix := flag.String(
		"prefix",
		"",
		"specify the prefix to add to the calculated version. ex : {prefix}a.b.c.d.e")

	suffix := flag.String(
		"suffix",
		"",
		"specify the prefix to add to the calculated version. ex : a.b.c.d.e{suffix}")

	branch := flag.String(
		"branch",
		"",
		"if not set, uses the current branch\nif set, use the specified branch")



	flag.Parse()

	if *target == "" {
		fmt.Printf("You must provide 'target' arg.\n%s",target_info)
	}

	regs := [5]*regexp.Regexp{
		regexp.MustCompile(*a),
		regexp.MustCompile(*b),
		regexp.MustCompile(*c),
		regexp.MustCompile(*d),
		regexp.MustCompile(*e),
	}


	commits, err := GetCommits(*target,*branch)
	if err != err {
		fmt.Printf("error : %v",err)
	}

	var final_version [5]int

	for idx,commit := range commits {
		version := calcul_single(commit, regs)

		if *strict && idx == 0 && version == [5]int{0,0,0,0,0} {
			fmt.Printf("Error, you use 'strict' : your last commit '%s' with message \n'%s' do not match any of the provided regex prefix, is not 'semantic'",commit.id,commit.msg)
		}

		erase := false
		for idx,val := range version{
			if erase {
				final_version[idx] = 0
				continue
			}
			if val == 1 {
				final_version[idx] += 1
				erase = true
			}
		}
	}

	version := *prefix
	for _,v := range final_version{
		version += string(v) + "."
	}
	strings.TrimRight(version,".")
	version += *suffix
}


