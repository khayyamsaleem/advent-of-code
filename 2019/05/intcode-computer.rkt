#lang racket

(provide eval-intcode)

; opcodes
(define PLUS  1)
(define MUL   2)
(define MOVE  3)
(define OUT   4)
(define JIT   5)
(define JIF   6)
(define BRLT  7)
(define BREQ  8)
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


(define (eval-intcode prg [user-input '()] [resume-from 0] [pid 'null])
  (define (iter prg cur user-input outputs)
    (cond
      [(empty? prg) 'eval-intcode-error]
      [else
       (let* ([opcode-and-param-modes (get-opcode-and-modes (vector-ref prg cur))]
              [opcode       (hash-ref opcode-and-param-modes "opcode")]
              [param-mode-1 (hash-ref opcode-and-param-modes "pm1")]
              [param-mode-2 (hash-ref opcode-and-param-modes "pm2")]
              [param-mode-3 (hash-ref opcode-and-param-modes "pm3")])
         (cond
           [(= opcode PLUS)
            (let ([arg1 (get-param-val prg param-mode-1 (+ cur 1))]
                  [arg2 (get-param-val prg param-mode-2 (+ cur 2))]
                  ; this is a boolean flag and I don't like it
                  ; but the nature of the problem has forced my hand
                  [dest (get-param-val prg POSITION_MODE (+ cur 3) #t)])
              (begin
                ; (displayln (format "PLUS AT POS ~a with args: ~a ~a ~a" cur arg1 arg2 dest))
                (vector-set! prg dest (+ arg1 arg2))
                (iter prg (+ cur 4) user-input outputs)
                ))]
           [(= opcode  MUL)
            (let ([arg1 (get-param-val prg param-mode-1 (+ cur 1))]
                  [arg2 (get-param-val prg param-mode-2 (+ cur 2))]
                  ; this is a boolean flag and I don't like it
                  ; but the nature of the problem has forced my hand
                  [dest (get-param-val prg POSITION_MODE (+ cur 3) #t)])
              (begin
                ; (displayln (format "MUL AT POS ~a with args: ~a ~a ~a" cur arg1 arg2 dest))
                (vector-set! prg dest (* arg1 arg2))
                (iter prg (+ cur 4) user-input outputs)
                ))]
           [(= opcode MOVE)
            (let ([dest (get-param-val prg POSITION_MODE (+ cur 1) #t)])
              (if (empty? user-input)
                  (hash 'program (vector->list prg) 'position cur 'output-signals outputs)
                  (begin
                    ; (displayln (format "MOV AT POS ~a with args: ~a" cur dest))
                    (displayln (format "getting user input: ~a" (car user-input)))
                    (vector-set! prg dest (car user-input))
                    (iter prg (+ cur 2) (cdr user-input) outputs))))]
           [(= opcode  OUT)
            (let ([arg1 (get-param-val prg param-mode-1 (+ cur 1))])
              (begin
                (displayln (format "DIAGNOSTIC CODE: ~a" arg1))
                (iter prg (+ cur 2) user-input (append outputs `(,arg1)))
                ))]
           [(= opcode  JIT)
            (let ([arg1 (get-param-val prg param-mode-1 (+ cur 1))]
                  [arg2 (get-param-val prg param-mode-2 (+ cur 2))])
              (begin
                ; (displayln (format "JIT AT POS ~a with args: ~a ~a" cur arg1 arg2))
                (if (not (zero? arg1))
                  (iter prg arg2 user-input outputs)
                  (iter prg (+ cur 3) user-input outputs))))]
           [(= opcode  JIF)
            (let ([arg1 (get-param-val prg param-mode-1 (+ cur 1))]
                  [arg2 (get-param-val prg param-mode-2 (+ cur 2))])
              (begin
                ; (displayln (format "JIF AT POS ~a with args: ~a ~a" cur arg1 arg2))
                (if (zero? arg1)
                  (iter prg arg2 user-input outputs)
                  (iter prg (+ cur 3) user-input outputs))))]
           [(= opcode BRLT)
            (let ([arg1 (get-param-val prg param-mode-1 (+ cur 1))]
                  [arg2 (get-param-val prg param-mode-2 (+ cur 2))]
                  [arg3 (get-param-val prg POSITION_MODE (+ cur 3) #t)])
              (if (< arg1 arg2)
                  (begin
                    ; (displayln (format "BRLT AT POS ~a with args: ~a ~a ~a" cur arg1 arg2 arg3))
                    (vector-set! prg arg3 1)
                    (iter prg (+ cur 4) user-input outputs))
                  (begin
                    ; (displayln (format "BRLT AT POS ~a with args: ~a ~a ~a" cur arg1 arg2 arg3))
                    (vector-set! prg arg3 0)
                    (iter prg (+ cur 4) user-input outputs))))]
           [(= opcode BREQ)
            (let ([arg1 (get-param-val prg param-mode-1 (+ cur 1))]
                  [arg2 (get-param-val prg param-mode-2 (+ cur 2))]
                  [arg3 (get-param-val prg POSITION_MODE (+ cur 3) #t)])
              (if (eq? arg1 arg2)
                  (begin
                    ; (displayln (format "BREQ AT POS ~a with args: ~a ~a ~a" cur arg1 arg2 arg3))
                    (vector-set! prg arg3 1)
                    (iter prg (+ cur 4) user-input outputs))
                  (begin
                    ; (displayln (format "BREQ AT POS ~a with args: ~a ~a ~a" cur arg1 arg2 arg3))
                    (vector-set! prg arg3 0)
                    (iter prg (+ cur 4) user-input outputs))))]
           [(= opcode HALT)
            (let ([final (car (vector->list prg))])
            (begin
              (displayln (format "HALT: ~a" final))
              (cons final outputs)))]
           [else (begin
                   (displayln (format "ERROR: GOT TO ~a" cur)))]
           ))]))
  (iter (list->vector prg) resume-from user-input '()))

(module+ main
  (eval-intcode '(1002 4 3 4 33) '(1))
  (eval-intcode '(1101 100 -1 4 0) '(1))
  (eval-intcode '(3 21 1008 21 8 20 1005 20 22 107 8 21 20 1006 20 31
1106 0 36 98 0 0 1002 21 125 20 4 20 1105 1 46 104
999 1105 1 46 1101 1000 1 20 4 20 1105 1 46 98 99) '(8))
  (eval-intcode '(3 12 6 12 15 1 13 14 13 4 13 99 -1 0 1 9) '(0))
  )

