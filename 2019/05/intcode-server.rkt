#lang racket

(require "intcode-computer.rkt")
(require web-server/servlet)
(require web-server/servlet-env)
(require json)
(require uuid)

(define program-states (make-hash))

(define (save-program-state! prg pos)
  (let ([k (uuid-symbol)])
    (hash-set! program-states k (cons prg pos))
    (symbol->string k)))

(define (eval-endpoint request)
  (define body (bytes->jsexpr (request-post-data/raw request)))
  (response/jsexpr
    (match body
         [(hash-table ('program program) ('inputs inputs))
            (let ([ret (eval-intcode program inputs)])
              (match ret 
                [(hash-table ('program prg) ('position instruction-pointer) ('output-signals output-signals))
                  (hash
                    'pid (save-program-state! prg instruction-pointer)
                    'output-signals output-signals
                    'halted #t
                    'result 'null)]
                [(cons result outputs)
                    (hash
                      'success #t
                      'output-signals outputs
                      'result result)]))]
         [(hash-table ('pid pid) ('inputs inputs))
          (match-let ([(cons prg pos) (hash-ref program-states (string->symbol pid))])
            (let ([ret (eval-intcode prg inputs pos)])
              (match ret 
                [(hash-table ('program prog) ('position instruction-pointer) ('output-signals output-signals))
                  (hash
                    'pid (save-program-state! prog instruction-pointer)
                    'output-signals output-signals
                    'halted #t
                    'result 'null)]
                [(cons result outputs)
                    (hash
                      'success #t
                      'output-signals outputs
                      'result result)])))]
         [_ (hash 'success #f)])))

;; URL routing table (URL dispatcher).
(define-values (dispatch generate-url)
  (dispatch-rules
    [("eval") #:method "post" eval-endpoint]
    [("health") #:method "get" (λ (req) (response/jsexpr (hash 'success #t)))]
    [("") #:method "get"
          (λ (req) (response/jsexpr
                      (hash 'success #t 'message "hello from intcode server")))]
    [else (error "There is no procedure to handle the url.")]))

(define (request-handler request)
  (dispatch request))

;; Start the server.
(serve/servlet
  request-handler
  #:launch-browser? #f
  #:quit? #f
  #:listen-ip "127.0.0.1"
  #:port 1337
  #:servlet-regexp #rx"")
