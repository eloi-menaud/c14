package Log

import (
	"fmt"
	"os"
)

var Verbose bool = true

func Verb(format string, args ...interface{}){
	if Verbose {
		fmt.Fprintf(os.Stderr,format+"\n", args...)
	}
}

func Info(format string, args ...interface{}){
	fmt.Fprintf(os.Stderr,format+"\n", args...)
}

func Fatal(format string, args ...interface{}){
	fmt.Fprintf(os.Stderr,"\x1b[0;31mX\x1b[0;0m "+format+"\n", args...)
	os.Exit(1)
}

func Res(format string, args ...interface{}){
	fmt.Fprintf(os.Stdout,format, args...)
	os.Exit(0)
}
