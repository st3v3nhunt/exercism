package diffsquares

import (
	"math"
)

// SquareOfSum ...
func SquareOfSum(n int) int {
	acc := 0
	for i := 1; i <= n; i++ {
		acc += i
	}
	// fmt.Println(acc)
	return int(math.Pow(float64(acc), 2))
}

//SumOfSquares ...
func SumOfSquares(n int) int {
	acc := 0
	for i := 1; i <= n; i++ {
		acc += int(math.Pow(float64(i), 2))
	}
	return acc
}

// Difference ...
func Difference(n int) int {
	return SquareOfSum(n) - SumOfSquares(n)
}
