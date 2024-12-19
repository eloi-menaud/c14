package utils

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"
)

type version struct{
	Patch int
	Minor int
	Major int
}
func (v version) String() string {
	return fmt.Sprintf("%d.%d.%d",v.Major,v.Minor,v.Patch)
}
func NewVersion(str_version string) (version,error){
	// check base version format
	version_regexp := `\d+.\d+.\d+`
	if !regexp.MustCompile(version_regexp).MatchString(str_version){
		return version{}, fmt.Errorf("'%s' incorrect format, must match %s",str_version, version_regexp)
	}

	// parsing
	strs := strings.Split(str_version,".")

	major,err := strconv.Atoi(strs[0])
	if err != nil{ return version{}, fmt.Errorf("MAJOR component is not a int") }

	minor,err := strconv.Atoi(strs[1])
	if err != nil{ return version{}, fmt.Errorf("MINOR component is not a int") }

	patch,err := strconv.Atoi(strs[2])
	if err != nil{ return version{}, fmt.Errorf("PATCH component is not a int") }

	return version{Major: major, Minor: minor, Patch: patch}, nil
}
func (v *version) BumpMajor() {
	v.Major += 1
	v.Minor  = 0
	v.Patch  = 0
}
func (v *version) BumpMinor() {
	v.Minor += 1
	v.Patch  = 0
}
func (v *version) BumpPatch() {
	v.Patch += 1
}