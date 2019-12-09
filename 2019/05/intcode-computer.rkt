#lang racket

(provide eval-intcode)

; opcodes
(define PLUS  1)
(define MUL   2)
(define MOVE  3)
(define OUT   4)
(define HALT 99)

; parameter modes
(define POSITION_MODE 0)
(define IMMEDIATE_MODE 1)


; destructures instruction num into opcode and parameter modes
(define (get-opcode-and-modes num)

  ; utility function for making an instruction number readable
  (define (pad-to-5-with-zeros lst)
    (cond
      [(eq? (length lst) 5) lst]
      [else (pad-to-5-with-zeros (cons 0 lst))]))

  ; utility function, split number into digit list
  (define (num->digits n)
    (cond
      [(eq? n 0) '()]
      [else (append (num->digits (floor (/ n 10))) (list (modulo n 10)))]))
  (let ([lst (pad-to-5-with-zeros (num->digits num))])
    (match-let ([(list pm3 pm2 pm1 op0 op1) lst])
      (hash
       "pm3" pm3
       "pm2" pm2
       "pm1" pm1
       "opcode" (+ (* op0 10) op1)))))

; get value for parameter for param mode, but writable destinations don't comply
; with the standard rules which is annoying
(define (get-param-val prg mode pos [writable #f])
  (cond
    [(or
      (and (eq? mode POSITION_MODE) writable)
      (eq? mode IMMEDIATE_MODE)) (vector-ref prg pos)]
    [(eq? mode POSITION_MODE) (vector-ref prg (vector-ref prg pos))]
    [else 'get-param-val-error]))

; phony input
(define (get-input) 1)

(define (eval-intcode prg)
  (define (iter prg cur)
    (cond
      [(empty? prg) 'eval-intcode-error]
      [else
       (let* ([opcode-and-param-modes (get-opcode-and-modes (vector-ref prg cur))]
              [opcode       (hash-ref opcode-and-param-modes "opcode")]
              [param-mode-1 (hash-ref opcode-and-param-modes "pm1")]
              [param-mode-2 (hash-ref opcode-and-param-modes "pm2")]
              [param-mode-3 (hash-ref opcode-and-param-modes "pm3")])
         (cond
           [(eq? opcode PLUS)
            (let ([arg1 (get-param-val prg param-mode-1 (+ cur 1))]
                  [arg2 (get-param-val prg param-mode-2 (+ cur 2))]
                  ; this is a boolean flag and I don't like it
                  ; but the nature of the problem has forced my hand
                  [dest (get-param-val prg POSITION_MODE (+ cur 3) #t)])
              (begin
                (vector-set! prg dest (+ arg1 arg2))
                (iter prg (+ cur 4))
                ))]
           [(eq? opcode  MUL)
            (let ([arg1 (get-param-val prg param-mode-1 (+ cur 1))]
                  [arg2 (get-param-val prg param-mode-2 (+ cur 2))]
                  ; this is a boolean flag and I don't like it
                  ; but the nature of the problem has forced my hand
                  [dest (get-param-val prg POSITION_MODE (+ cur 3) #t)])
              (begin
                (vector-set! prg dest (* arg1 arg2))
                (iter prg (+ cur 4))
                ))]
           [(eq? opcode MOVE)
            (let ([dest (get-param-val prg POSITION_MODE (+ cur 1) #t)])
              (begin
                (vector-set! prg dest (get-input))
                (iter prg (+ cur 2))
                ))]
           [(eq? opcode  OUT)
            (let ([arg1 (get-param-val prg param-mode-1 (+ cur 1))])
              (begin
                (displayln (format "DIAGNOSTIC CODE: ~a" arg1))
                (iter prg (+ cur 2))
                ))]
           [(eq? opcode HALT)
            (let ([final (car (vector->list prg))])
            (begin
              (displayln (format "HALT: ~a" final))
              final))]
           [else (begin
                   (displayln (format "ERROR: GOT TO ~a" cur)))]
           ))]))
  (iter (list->vector prg) 0))

(module+ main
  (eval-intcode '(1002 4 3 4 33))
  (eval-intcode '(1101 100 -1 4 0))
  )

