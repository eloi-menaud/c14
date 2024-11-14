package Log

import (
	"fmt"
	"os"
)

var Verbose bool = false

func Verb(format string, args ...interface{}){
	if Verbose {
		fmt.Printf(format+"\n", args...)
	}
}

func Info(format string, args ...interface{}){
	fmt.Printf(format+"\n", args...)
}

func Fatal(format string, args ...interface{}){
	fmt.Printf("X "+format+"\n", args...)
	os.Exit(1)
}