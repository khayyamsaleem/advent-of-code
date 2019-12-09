#lang at-exp racket

(require net/url)
(require dotenv)
(require "intcode-computer.rkt")

(module+ test
  (require rackunit))


(define DAY_5_INPUT "https://adventofcode.com/2019/day/5/input")

(define (get-challenge-input url)
  (filter number?
          (map string->number
               (string-split
                (call/input-url (string->url url)
                                get-pure-port
                                port->string
                                (cons (format "Cookie: session=~a" (getenv "session")) '()))
                ","
                #:trim? #t))))

(module+ main
  (dotenv-load!)
  (eval-intcode (get-challenge-input DAY_5_INPUT))
  )
