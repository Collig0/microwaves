package main

import (
	"time"
	"fmt"
	"flag"
	"strconv"
	"log"
)

const frequency = 10.0
const sleepTime = (time.Second / frequency)

func main() {
	flag.Parse();
	cookTime, err := strconv.Atoi(flag.Args()[0])
	if err != nil {
		log.Fatal("Please pass a valid number for cook time.")
	}
	cookTimeMinutes := cookTime / 60
	cookTimeSecondsDisplay := cookTime % 60
	fmt.Printf("COOK TIME: %02d:%02d\nBEEP!\n", cookTimeMinutes, cookTimeSecondsDisplay)
	reps := frequency * cookTime
	for i := 0; i < reps; i++ {
		fmt.Print("m")
		time.Sleep(sleepTime)
	}
	fmt.Println("\nBEEEP! BEEEP! BEEP!")
	fmt.Println("Food's done! :)")
}
