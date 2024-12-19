package utils

import (
	"encoding/json"
	"fmt"
	"regexp"
	"strings"

	"gopkg.in/yaml.v2"
)

const (
	Regexp_convcom = `(?ms)^(\w+)(?:\(([\w\-]+)\))?(!)?: ([^(\n|\r\n)]+)(?:\n|\r\n){0,2}(.*)$`
)


type Footer struct{
	Key   string
	Value string
}
type Commit struct {
	RawCommit

	IsConvCom   bool     `json:"isConvCom"`
	Type        string   `json:"type"`
	Scope       string   `json:"scope"`
	Exclamation bool     `json:"exclamation"`
	Description string   `json:"description"`
	Body        string   `json:"body"`
	Footers     []Footer `json:"footers"`
	IsBreak     bool     `json:"isBreak"`
}
func (c Commit) String() string {
	if ! c.IsConvCom {
		y,_ := yaml.Marshal(
			struct{
				Id        string `json:"id"`
				Message   string `json:"message"`
				Tag       string `json:"tag"`
				IsConvCom bool   `json:"isConvCom"`
			}{
				c.Id,
				c.Message,
				c.Tag,
				false,
			},
		)
		return string(y)
	}
	y,_ := yaml.Marshal(c)
	return strings.TrimSpace(string(y))
}

func (c Commit) GetBreakingChangeDetails() string {
	for _,footer := range c.Footers{
		if (footer.Key == "BREAKING-CHANGE") || (footer.Key == "BREAKING CHANGE") {
			return footer.Value
		}
	}
	return ""
}

func (c Commit) Json() string {
	if ! c.IsConvCom {
		y,_ := json.MarshalIndent(
			struct{
				Id        string `json:"id"`
				Message   string `json:"message"`
				Tag       string `json:"tag"`
				IsConvCom bool   `json:"isConvCom"`
			}{
				c.Id,
				c.Message,
				c.Tag,
				false,
			},"","  ")
		return string(y)
	}
	y,_ := json.MarshalIndent(c,"","  ")
	return strings.TrimSpace(string(y))
}



func NewCommit(r RawCommit) (Commit,error) {

	c := Commit{
		RawCommit: r,
	}

	convcom_match := regexp.MustCompile(Regexp_convcom).FindStringSubmatch(r.Message)
	if len(convcom_match) == 0 {
		return c,fmt.Errorf("not a conventional commit. use 'c14 convcom -spec' to see specification on conventional commit")
	}

	c.Type        = convcom_match[1]
	c.Scope       = convcom_match[2]
	c.Exclamation = convcom_match[3] == "!"
	c.IsBreak     = c.Exclamation
	c.Description = convcom_match[4]



	// parse raw body to .Body and .Footers
	const raw_footer_regexp = `^([\w-]+|BREAKING CHANGE)(?:: | #)\s*(.+)`

		// splitting body and footers_bloc by looking at the first footer find
	var footers_bloc string
	loc := regexp.MustCompile("(?m)"+raw_footer_regexp).FindStringIndex(convcom_match[5])
	if len(loc) == 0 { // no footers bloc found
		c.Body = strings.TrimSpace(convcom_match[5])
	} else {
		c.Body = strings.TrimSpace(convcom_match[5][:loc[0]])
		footers_bloc = convcom_match[5][loc[0]:]
	}

		// parse footers bloc to []Footer (.Footers)
	var footers []Footer
	re_separator := regexp.MustCompile("(?m)" +raw_footer_regexp)
	re_data      := regexp.MustCompile("(?ms)"+raw_footer_regexp)
	locs         := re_separator.FindAllStringIndex(footers_bloc, -1)


	for idx := range locs {
		var single_footer_bloc string
		if idx+1 == len(locs) { // in case of final footer : start to end
			single_footer_bloc = footers_bloc[locs[idx][0]:]
		} else { // remains footer after : start of the current footer, to the start of the next one
			single_footer_bloc = footers_bloc[locs[idx][0]:locs[idx+1][0]]
		}
		match := re_data.FindStringSubmatch(single_footer_bloc)

		footers = append(footers,Footer{ match[1], strings.TrimSpace(match[2]) })
	}
	c.Footers = footers


	// checking if there is some 'breaking change' footers
	if ! c.IsBreak {
		for _, f := range c.Footers {
			if (f.Key == "BREAKING CHANGE") || (f.Key == "BREAKING-CHANGE") {
				c.IsBreak = true
				break
			}
		}
	}


	c.IsConvCom = true
	return c, nil
}







