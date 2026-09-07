package day02

import (
	"fmt"
	"strings"
	"strconv"
)

type bound struct {
	x int
	y int
}

func Parse(input string) []bound {
	ranges := []bound{}
	for _, b := range strings.Split(strings.TrimSpace(input), ",") {
		s := strings.Split(b, "-")
		x, err := strconv.Atoi(s[0])
		if err != nil {
			panic(err)
		}
		y, err := strconv.Atoi(s[1])
		if err != nil {
			panic(err)
		}
		ranges = append(ranges, bound{x: x, y: y})
	}
	return ranges
}

func P1(input string) string {
	ranges := Parse(input)
	t := 0
	for _, r := range ranges {
		for n := r.x; n <= r.y; n++ {
			s := fmt.Sprintf("%d", n)
			if len(s) % 2 != 0 || len(s) < 2 {
				continue
			}
			if s[:len(s)/2] != s[len(s)/2:] {
				continue
			}
			v, err := strconv.Atoi(s)
			if err != nil {
				panic(err)
			}
			t += v
		}
	}
	return fmt.Sprintf("%d", t)
}

func P2(input string) string {
	ranges := Parse(input)
	t := 0
	for _, r := range ranges {
		for i := r.x; i <= r.y; i++ {
			if i == 0 {
				continue
			}
			s := fmt.Sprintf("%d", i)
			if strings.Contains(fmt.Sprintf("%d%d",i,i)[1:len(s)*2-1], s) {
				t += i
			}
		}
	}
	return fmt.Sprintf("%d", t)
}
