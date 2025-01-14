package utils

import (
	"bytes"
	"text/template"
)

type section struct{
	Tag  string
	Feat []Commit
	Fix  []Commit
}
func (s *section) add(c Commit){
	switch c.Type{
	case "feat" :
		s.Feat = append(s.Feat, c)
	case "fix" :
		s.Fix = append(s.Fix, c)
	}
}








type ChangeLog []section

func NewChangeLog(convcoms []Commit) ChangeLog{

	var sections []section

	var sect section
	for _,convcom := range convcoms{
		if convcom.Tag != ""{
			sections = append(sections, sect)
			sect = section{Tag:convcom.Tag}
		}
		if convcom.IsConvCom{
			sect.add(convcom)
		}
	}
	sections = append(sections, sect)

	return sections
}





func toString(tpl string, c ChangeLog) (string, error){
	tmpl, err := template.New("tmpl").Parse(tpl)
	if err != nil {
		return "", err
	}

	var buf bytes.Buffer
	if err := tmpl.Execute(&buf, c); err != nil {
		return "", err
	}
	return buf.String(), nil
}


func (c ChangeLog) Text(target string) (string,error){
	title := "== changelog =="
	if target != "" { title = "== changelog " + target + " =="}
	tpl := title + `{{- range . }}
{{ if .Tag }}
{{- .Tag }}
{{- else if and (gt (len .Feat) 0) (gt (len .Fix) 0) }}
*{{- end -}}
{{- if gt (len .Feat) 0 }}
  Feat :
  {{- range .Feat }}
    - {{ if .IsBreak }}! {{end}}{{ if .Scope }}[{{.Scope}}] {{end}}{{ .Description }} ({{ .ShortId }})
      {{- if ( .GetBreakingChangeDetails )}}
      break : {{( .GetBreakingChangeDetails )}}{{ end }}
  {{- end }}
{{- end -}}
{{- if gt (len .Fix) 0 }}
  Fix :
  {{- range .Fix }}
    - {{ if .IsBreak }}! {{end}}{{ if .Scope }}[{{.Scope}}] {{end}}{{ .Description }} ({{ .ShortId }})
      {{- if ( .GetBreakingChangeDetails )}}
      break : {{( .GetBreakingChangeDetails )}}{{ end }}
  {{- end }}
{{- end }}
{{- end }}
`
	return toString(tpl, c)
}

func (c ChangeLog) Md(target string) (string,error){
	title := "# Changelog " + target
	tpl := title + `
{{- range . }}
{{ if .Tag }}# {{ .Tag }}{{ end }}

{{- if gt (len .Feat) 0 }}
### Feat :
{{- range .Feat }}
- {{ if .IsBreak }}! {{end}}{{ if .Scope }}[{{.Scope}}] {{end}}{{ .Description }} _({{ .ShortId }})_
	{{- if ( .GetBreakingChangeDetails )}}
	> break : {{( .GetBreakingChangeDetails )}}{{ end }}
{{- end }}
{{- end }}
{{- if gt (len .Fix) 0 }}
### Fix :
{{- range .Fix }}
- {{ if .IsBreak }}! {{end}}{{ if .Scope }}[{{.Scope}}] {{end}}{{ .Description }} _({{ .ShortId }})_
{{- if ( .GetBreakingChangeDetails )}}
  > break : {{( .GetBreakingChangeDetails )}}{{ end }}
{{- end }}
{{- end }}
{{- end }}
`
	return toString(tpl, c)
}




func (c ChangeLog) Html(target string) (string,error){
	title := "<h1>changelog " + target + "</h1>"
	tpl := title + `
{{- range . }}
<article>
{{ if .Tag }}<h1>{{ .Tag }}</h1>{{ end }}

{{- if gt (len .Feat) 0 }}
<h2>Feat :</h2>
<ul>
  {{- range .Feat }}
  <li>
    {{ if .IsBreak }}<p class='break-mark'>!</p>{{end}}
    {{ if .Scope }}<p class='scope'>{{.Scope}}</p>{{end}}
    <p class='desc'>{{ .Description }}</p>
    <p class='commit-id'>{{ .ShortId }}</p>
    {{- if ( .GetBreakingChangeDetails )}}
    <blockquote> break : {{( .GetBreakingChangeDetails )}}</blockquote>{{ end }}
  </li>
  {{- end }}
</ul>

{{- end }}
{{- if gt (len .Fix) 0 }}
<h2>Fix :</h2>
<ul>
  {{- range .Fix }}
  <li>
    {{ if .IsBreak }}<p class='break-mark'>!</p>{{end}}
    {{ if .Scope }}<p class='scope'>{{.Scope}}</p>{{end}}
    <p class='desc'>{{ .Description }}</p>
    <p class='commit-id'>{{ .ShortId }}</p>
    {{- if ( .GetBreakingChangeDetails )}}
    <blockquote> break : {{( .GetBreakingChangeDetails )}}</blockquote>{{ end }}
  </li>
  {{- end }}
</ul>
{{- end }}
</article>
{{- end }}
`
	return toString(tpl, c)
}






