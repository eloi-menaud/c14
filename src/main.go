package main

import (
	"c14/commit"
	"fmt"
)

func main(){
	cmt, err := commit.New("ee011da250bb8cf75ef590d8341677fa75285003")
	fmt.Println(err)
	fmt.Println(cmt)
}