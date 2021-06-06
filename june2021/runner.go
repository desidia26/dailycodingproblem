package main

import (
	"fmt"
	"sync"
)

func main() {
	run_6_1_21()
	run_6_2_21()
	run_6_3_21()
}

func run_6_1_21() {
	var wg sync.WaitGroup
	wg.Add(1)
	jobScheduler(
		func() {
			fmt.Println("Hello, World!")
			defer wg.Done()
		}, 5000)
	wg.Wait()
}

func run_6_2_21() {
	matchingWords := wordSearch("hi", []string{
		"ayy",
		"lmao",
		"high",
		"hit",
		"zebra",
	})
	fmt.Printf("%v", matchingWords)
}

func run_6_3_21() {
	stairClimber(5, []int{1, 2})
}
