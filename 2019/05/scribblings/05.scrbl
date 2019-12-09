#lang scribble/manual
@require[@for-label[|05|
                    racket/base]]

@title{05}
@author{khayyamsaleem}

@defmodule[|05|]

Solution for Advent of Code 2019 Day 5

@section{Intcode Computer}

The @code|{intcode-computer.rkt}| file provides an intcode computer conforming to the Advent of Code 2019 Day 5 specification. The @code|{eval-intcode}| function returns the first number of the intcode program state after evaluation. It takes as input a @code|{list}| representing an intcode instruction sequence. It can be used like:

@codeblock|{
(eval-intcode '(1002 4 5 4))
}|
