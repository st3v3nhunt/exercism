package reverse

import "strings"

// Reverse ...
func Reverse(s string) string {
	sl := len(s)
	ss := make([]string, sl)
	for i, c := range s {
		ss[sl-i-1] = string(c)
	}
	return strings.Join(ss, "")
}
