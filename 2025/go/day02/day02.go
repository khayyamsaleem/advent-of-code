package day02

import (
	"fmt"
	"strconv"
	"strings"
	"sync/atomic"

	"golang.org/x/sync/errgroup"
)

type bound struct {
	x int
	y int
}

func PumpBounds(input string, ch chan<- bound) {
	defer close(ch)
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
		ch <- bound{x: x, y: y}
	}
}

func P1(input string) string {
	ch := make(chan bound)
	go PumpBounds(input, ch)

	var eg errgroup.Group
	var t atomic.Int64
	for r := range ch {
		eg.Go(func() error {
			var local int64
			for n := r.x; n <= r.y; n++ {
				s := fmt.Sprintf("%d", n)
				if len(s)%2 != 0 || len(s) < 2 {
					continue
				}
				if s[:len(s)/2] != s[len(s)/2:] {
					continue
				}
				v, err := strconv.Atoi(s)
				if err != nil {
					return err
				}
				local += int64(v)
			}
			t.Add(local)
			return nil
		})
	}
	if err := eg.Wait(); err != nil {
		panic(err)
	}
	return fmt.Sprintf("%d", t.Load())
}

func P2(input string) string {
	ch := make(chan bound)
	go PumpBounds(input, ch)

	var eg errgroup.Group
	var t atomic.Int64
	for r := range ch {
		eg.Go(func() error {
			var local int64
			for i := r.x; i <= r.y; i++ {
				if i == 0 {
					continue
				}
				s := fmt.Sprintf("%d", i)
				if strings.Contains(fmt.Sprintf("%d%d", i, i)[1:len(s)*2-1], s) {
					local += int64(i)
				}
			}
			t.Add(local)
			return nil
		})
	}
	if err := eg.Wait(); err != nil {
		panic(err)
	}
	return fmt.Sprintf("%d", t.Load())
}
