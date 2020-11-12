package pangram

import (
	"regexp"
	"strings"
)

// IsPangram ...
func IsPangram(s string) bool {
	strings.Split(s, "")
	reg, _ := regexp.Compile("[^a-zA-Z]+")
	clean := reg.ReplaceAllString(s, "")
	return true
}
