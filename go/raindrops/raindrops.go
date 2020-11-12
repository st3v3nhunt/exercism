package raindrops

import "strconv"

// Convert ...
func Convert(n int) (sounds string) {
	if n%3 == 0 {
		sounds += "Pling"
	}
	if n%5 == 0 {
		sounds += "Plang"
	}
	if n%7 == 0 {
		sounds += "Plong"
	}
	if len(sounds) == 0 {
		sounds += strconv.Itoa(n)
	}
	return sounds
}
