package main

import (
	"c14/commit"
	"fmt"
)

func main(){
	cmt, err := commit.New("e8b6a0f")
	fmt.Println(err)
	fmt.Println(cmt)
}