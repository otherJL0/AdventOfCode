package main

import (
	"bufio"
	"fmt"
	"os"
	"unicode"
)

func readInput() []string {
	inputFile, err := os.Open("input")
	if err != nil {
		fmt.Println(err)
	}
	scanner := bufio.NewScanner(inputFile)
	scanner.Split(bufio.ScanLines)
	lines := make([]string, 0)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	inputFile.Close()
	return lines
}

func partOne() int {
	lines := readInput()

	total := 0
	for _, line := range lines {
		digits := make([]int, 0, len(line))
		for _, letter := range line {
			if unicode.IsDigit(letter) {
				digits = append(digits, int(letter-'0'))
			}
		}
		total += digits[0]*10 + digits[len(digits)-1]
	}
	return total
}

func main() {
	fmt.Println(partOne())
}
