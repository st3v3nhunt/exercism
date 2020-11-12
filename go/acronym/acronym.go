// Package acronym should have a package comment that summarizes what it's about.
// https://golang.org/doc/effective_go.html#commentary
package acronym

import (
	"strings"
)

// Abbreviate should have a comment documenting it.
func Abbreviate(s string) string {
	// re := regexp.MustCompile("[ \\-_]+")
	// words := re.Split(s, -1)
	s = strings.ReplaceAll(s, "-", " ")
	s = strings.ReplaceAll(s, "_", "")
	words := strings.Fields(s)

	var ret = []string{}
	for i := range words {
		ret = append(ret, strings.ToUpper(string(words[i][0])))
	}
	return strings.Join(ret, "")
}
