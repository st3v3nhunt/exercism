// This is a "stub" file.  It's a little start on your solution.
// It's not a complete solution though; you have to write some code.

// Package bob should have a package comment that summarizes what it's about.
// https://golang.org/doc/effective_go.html#commentary
package bob

import (
	"strings"
)

func letters(remark string) bool {
	var found = false
	for _, r := range strings.ToLower(remark) {
		if r >= 'a' && r <= 'z' {
			found = true
		}
	}
	return found
}

// Hey should have a comment documenting it.
func Hey(remark string) string {
	remark = strings.TrimSpace(remark)
	if len(remark) == 0 {
		return "Fine. Be that way!"
	}

	// fmt.Printf("remark: '%s'\n", remark)
	question := remark[len(remark)-1:] == "?"
	yelling := strings.ToUpper(remark) == remark
	letters := letters(remark)
	addressed := strings.Contains(remark, "Bob")

	// if letters {
	if letters && question && yelling {
		return "Calm down, I know what I'm doing!"
	}
	if letters && question || question {
		return "Sure."
	}
	if letters && yelling {
		return "Whoa, chill out!"
	}
	if letters && addressed {
		return "Fine. Be that way!"
	}
	// }
	// if question {
	// return "Sure."
	// }
	return "Whatever."
}
