package utils

import (
	"bytes"
	"os/exec"
	"regexp"
	"strings"
)

func Cmd(ex string,args ...string) (string, string, error){


	cmd := exec.Command(ex,args[0:]...)

	var stdout bytes.Buffer
	var stderr bytes.Buffer

	cmd.Stdout = &stdout
	cmd.Stderr = &stderr

	err := cmd.Run()

	return strings.TrimSpace(stdout.String()),strings.TrimSpace(stderr.String()),err
}

func CmdResToString(stdout string, stderr string, err error) string {
	var res string
	if stdout != "" {
		res += "\nstdout:\n" + regexp.MustCompile("(?m)^").ReplaceAllString(stdout,"  ")
	}
	if stderr != "" {
		res += "\nstderr:\n" + regexp.MustCompile("(?m)^").ReplaceAllString(stderr,"  ")
	}
	if err != nil {
		res += "\nerror: " + err.Error()
	}
	return res
}