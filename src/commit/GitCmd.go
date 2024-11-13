package commit

import (
	"bytes"
	"c14/Log"
	"fmt"
	"os/exec"
	"strings"
)

func GitCmd(args ...string) (string, string, error){

	Log.Verb("executing 'git %s'", strings.Join(args, " "))

	cmd := exec.Command("git", args...)

	var stdout bytes.Buffer
	var stderr bytes.Buffer

	cmd.Stdout = &stdout
	cmd.Stderr = &stderr

	err := cmd.Run()


	if err != nil {
		return "","", fmt.Errorf(
			"failed to execute 'git %s' command :\nerror : %s\nstderr :\n%s",
			strings.Join(args, " "),
			err,
			stderr.String())
	}

	return stdout.String(),stderr.String(),nil
}
