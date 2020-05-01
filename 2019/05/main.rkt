#lang at-exp racket

(require net/url)
(require dotenv)
(require "intcode-computer.rkt")

(module+ test
  (require rackunit))


(define DAY_5_INPUT "https://adventofcode.com/2019/day/5/input")
(define DAY_9_INPUT "https://adventofcode.com/2019/day/9/input")

(define (get-challenge-input url)
   (map string->number (string-split
    (string-trim (call/input-url (string->url url)
                    get-pure-port
                    port->string
                    (cons (format "Cookie: session=~a" (getenv "session")) '()))
                 #:repeat? #t)
    ",")))

(module+ main
  (dotenv-load!)
  ; Part One
  (displayln "PART ONE")
  (eval-intcode (get-challenge-input DAY_5_INPUT) '(1))
  ; Part Two
  (displayln "PART TWO")
  (eval-intcode (get-challenge-input DAY_5_INPUT) '(5))
)
