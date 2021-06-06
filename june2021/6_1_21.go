// Good morning! Here's your coding interview problem for today.

// This problem was asked by Apple.

// Implement a job scheduler which takes in a function f and an integer n, and calls f after n milliseconds.
package main

import (
	"fmt"
	"time"
)

type fn func()

func jobScheduler(job fn, timems int) {
	fmt.Println("Scheduling!")
	go func() {
		time.Sleep(time.Duration(timems * 1000000))
		job()
	}()
}
