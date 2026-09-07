package day02

import (
	"slices"
	"testing"
)

func TestPumpBounds(t *testing.T) {
	input := `11-22,95-115`

	ch := make(chan bound)
	go PumpBounds(input, ch)

	got := []bound{}
	for b := range ch {
		got = append(got, b)
	}

	want := []bound{
		{x: 11, y: 22},
		{x: 95, y: 115},
	}

	if !slices.Equal(got, want) {
		t.Errorf("PumpBounds() = %q, want %q", got, want)
	}
}

func TestP1(t *testing.T) {
	input := `11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124`

	got := P1(input)
	want := "1227775554"
	if got != want {
		t.Errorf("%s != %s", got, want)
	}
}

func TestP2(t *testing.T) {
	input := `11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124`

	got := P2(input)
	want := "4174379265"

	if got != want {
		t.Errorf("%s != %s", got, want)
	}
}
