package main

import (
	"c14/commit"
	"fmt"
)

func main(){
	cmt, _ := commit.New("67d41cfbc17ffed47be60e157fd09733c490d856")
	fmt.Println(cmt.String())
}