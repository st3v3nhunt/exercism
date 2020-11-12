package sublist

import "fmt"

// Relation ...
type Relation string

// Sublist ...
func Sublist(l1, l2 []int) Relation {
	fmt.Printf("l1: %v\n", l1)
	fmt.Printf("l2: %v\n", l2)

	// special cases
	if len(l1) == 0 && len(l2) == 0 {
		return "equal"
	}

	if len(l1) == 0 {
		return "sublist"
	}

	if len(l2) == 0 {
		return "superlist"
	}
	// is list1 contained within list2? - sublist
	// is list2 contained within list1? - superlist
	// is list1 equal to list2? equal
	// are the lists not equal at all? - unequal
	// if l1 == l2 {
	// 	return "equal"
	// }
	// if len(l1) == len(l2) {
	// 	"equal"
	// }
	if equal(l1, l2) {
		return "equal"
	}
	if sublist(l1, l2) {
		return "sublist"
	}
	return "unequal"
}

func equal(l1, l2 []int) bool {
	for i, v := range l1 {
		if v != l2[i] {
			return false
		}
	}
	return true
}

func sublist(l1, l2 []int) bool {
	for i := 0; i < len(l1); i++ {
		if l1[i] == l2[i] {
		}
	}

	for i, v := range l1 {
		if v == l2[i] {
			return false
		}
	}
	return true
}
