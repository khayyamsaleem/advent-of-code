#lang racket

(require "intcode-computer.rkt")
(require web-server/servlet)
(require web-server/servlet-env)
(require json)

(define (eval-endpoint request)
  (define body (bytes->jsexpr (request-post-data/raw request)))
  (response/jsexpr
    (match body
         [(hash-table ('program program) ('inputs inputs) ('program_counter c))
            (let ([ret (eval-intcode program inputs c)])
              (match ret 
                [(hash-table ('program prg) ('position program-counter) ('output-signals output-signals))
                  (hash
                    'output-signals output-signals
                    'program-counter program-counter
                    'program prg
                    'blocked #t
                    'result 'null)]
                [(cons result outputs)
                    (hash
                      'success #t
                      'output-signals outputs
                      'result result)]))]
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
(module+ main
  (serve/servlet
    request-handler
    #:launch-browser? #f
    #:quit? #f
    #:listen-ip "0.0.0.0"
    #:port 1337
    #:stateless? #t
    #:servlet-regexp #rx""))