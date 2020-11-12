package etl

import "strings"

// Transform ...
func Transform(data map[int][]string) map[string]int {
	scores := map[string]int{}

	for k, letters := range data {
		for _, letter := range letters {
			scores[strings.ToLower(letter)] = k
		}
	}

	return scores
}
