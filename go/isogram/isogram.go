package isogram

import (
	"sort"
	"strings"
)

// IsIsogram ...
func IsIsogram(s string) bool {
	s = strings.ReplaceAll(s, "-", "")
	s = strings.ReplaceAll(s, " ", "")
	ss := strings.Split(strings.ToLower(s), "")
	sort.Strings(ss)
	prev := ""

	for k, cur := range ss {
		if k > 0 {
			prev = ss[k-1]
		}
		if cur == prev {
			return false
		}
	}
	// fmt.Println(strings.Split(s, ""))
	// // strings.Split(s, "")
	// var l = []string{}
	// for _, v := range strings.Split(s, "") {
	// 	fmt.Println(v)
	// }
	return true
}
