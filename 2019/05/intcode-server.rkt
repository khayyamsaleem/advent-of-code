#lang racket

(require "intcode-computer.rkt")
(require web-server/servlet)
(require web-server/servlet-env)
(require json)

(define (eval-endpoint request)
  (define body (bytes->jsexpr (request-post-data/raw request)))
  (response/jsexpr
    (match body
         [(hash-table ('program program) ('inputs inputs))
            (hash
              'success #t
              'result (eval-intcode program inputs))]
         [(hash-table ('program program))
            (hash
              'success #t
              'result (eval-intcode program))]
         [_ (hash 'success #f)])))

;; URL routing table (URL dispatcher).
(define-values (dispatch generate-url)
  (dispatch-rules
    [("eval") #:method "post" eval-endpoint]
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
