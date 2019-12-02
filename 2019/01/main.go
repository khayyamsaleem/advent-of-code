package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"net/http"
	"os"
	"strconv"
	"strings"

	"github.com/joho/godotenv"
)

func getInputAsArray() []int {
	err := godotenv.Load()
	sessionCookie := os.Getenv("session")
	req, err := http.NewRequest("GET", "https://adventofcode.com/2019/day/1/input", nil)
	if err != nil {
		panic(err)
	}
	req.AddCookie(&http.Cookie{Name: "session", Value: sessionCookie})
	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		panic(err)
	}
	defer resp.Body.Close()
	bodyBytes, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		panic(err)
	}
	body := string(bodyBytes)
	inputs := strings.Split(strings.TrimSpace(body), "\n")
	out := []int{}
	for _, val := range inputs {
		inp, err := strconv.Atoi(val)
		if err != nil {
			panic(err)
		}
		out = append(out, inp)
	}
	return out
}

func fuelRequiredForMass(mass int) int {
	return int(math.Floor(float64(mass)/3)) - 2
}

func totalFuelRequiredForMass(mass int) int {
	if mass <= 0 {
		return 0
	}
	current := fuelRequiredForMass(mass)
	if current < 0 {
		current = 0
	}
	return current + totalFuelRequiredForMass(current)
}

func partOne(masses []int) int {
	total := 0
	for _, val := range masses {
		total = total + fuelRequiredForMass(val)
	}
	return total
}

func partTwo(masses []int) int {
	total := 0
	for _, val := range masses {
		total = total + totalFuelRequiredForMass(val)
	}
	return total
}

func main() {
	inputs := getInputAsArray()
	fmt.Println(partOne(inputs))
	fmt.Println(partTwo(inputs))
}
