package main

import (
	act "c14/action"
	"c14/utils"
	"c14/utils/Log"

	"os"

	"flag"
)

const version = "1.0.0"

func main(){

	usage := `
c14 <action> [flags]
c14 -version

<action> :
  version   : Calculate the version
  changelog : Generate a changelog
  parse     : Parse all commits into JSON format

Flags :
  -help    : Display help and exit
  -v       : Enable verbose mode
  -version : Display the c14 version and exit
  (For more flag details, use 'c14 <action> -help')
`

	flag.Usage = func() {Log.Info("%s",usage)}

	if len(os.Args) < 2 {
		Log.Fatal("You must provide an action\n%s",usage)
	}
	action        := os.Args[1]
	action_args   := os.Args[2:]

	if action == "-help" {
		Log.Info(usage)
		os.Exit(0)
	}
	if action == "-version" {
		Log.Info("c14 version %s",version)
		os.Exit(0)
	}

	flag.BoolVar(&Log.Verbose,"v",false,"")
	help := flag.Bool("help",false,"")



	switch action {

	case "version":
		usage := `
c14 version [flags] <revision>

Calculate the version for the specified revision.

<revision> :
  The git revision specifying the commit range to consider.

Flags :
  -base          : The base version for incrementing (default is '1.0.0')
  -target <path> : Use only commit related to the specified target (path to a dir or file)
  -help          : Display help and exit
  -v             : Enable verbose mode

`

		base   := flag.String("base", "1.0.0", "")
		target := flag.String("target", "", "")
		flag.CommandLine.Parse(action_args)
		args := flag.Args()

		if *help{ Log.Info(usage); os.Exit(0) }

		if len(args) < 1 {
			Log.Info(usage)
			Log.Fatal("You must provide a <revision>:\n%s",utils.GetRevisionHelpMessage())
		}
		revision := args[0]

		err := act.Version(revision, *base, *target)
		if err != nil {
			Log.Fatal("Error : %v",err)
		}








	case "changelog":
		usage = `
c14 changelog [flags] <revision>

Generate the changelog for the specified revision.

<revision> :
  The git revision specifying the commit range to consider.

Flags :
  -format <name> : [md | html | text] Specify the output format for the changelog (default: md)
  -target <path> : Use only commit related to the specified target (path to a dir or file)
  -help          : Display help and exit
  -v             : Enable verbose mode

`

		format := flag.String("format", "md", "")
		target := flag.String("target", "", "")
		flag.CommandLine.Parse(action_args)
		args := flag.Args()

		if len(args) < 1 {
			Log.Info(usage)
			Log.Fatal("You must provide a <revision>:\n%s",utils.GetRevisionHelpMessage())
		}

		revision := args[0]

		flag.CommandLine.Parse(args[1:])

		if *help{
			flag.Usage(); os.Exit(0)
		}

		err := act.Changelog(*format, revision, *target)
		if err != nil {
			Log.Fatal("Error : %v",err)
		}








	case "parse" :
		usage = `z
c14 parse [flags] <revision>

Parse all commits to conventional commit format and return as JSON.

<revision> :
  The git revision specifying the commit range to consider.

Flags :
  -strict        : Exit with status 1 if any commit does not adhere to the conventional commit format
  -spec          : Display details (format, examples, links, regex, etc.) about the conventional commit format and exit
  -target <path> : Use only commit related to the specified target (path to a dir or file)
  -help          : Display help and exit
  -v             : Enable verbose mode
`

		strict := flag.Bool("strict",false,"")
		spec   := flag.Bool("spec",false,"")
		target := flag.String("target", "", "")

		flag.CommandLine.Parse(action_args)
		args := flag.Args()

		if len(args) < 1 {
			Log.Info(usage)
			Log.Fatal("You must provide a <revision>:\n%s",utils.GetRevisionHelpMessage())
		}

		revision := args[0]

		flag.CommandLine.Parse(args[1:])

		if *help{
			flag.Usage(); os.Exit(0)
		}

		if *spec{
			Log.Info("%s",utils.GetSpecification()); os.Exit(0)
		}

		err := act.Parse(revision, *strict, *target)
		if err != nil {
			Log.Fatal("Error : %v",err)
		}


	default:
		Log.Fatal("Wrong '%s' action\n%s",action,usage)
	}
}








