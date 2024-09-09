package main

import (
	"fmt"
	"sync"
)

func main() {
	n := 0;

	var wg sync.WaitGroup
	wg.Add(2)

	go func() {
		defer wg.Done()
		for i := 0; i < 1000; i++ {
			n++
		}
	}()

	go func() {
		defer wg.Done()
		for i := 0; i < 1000; i++ {
			n++
		}
	}()

	wg.Wait()

	fmt.Println(n)

}