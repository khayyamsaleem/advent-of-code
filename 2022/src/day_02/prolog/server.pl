:- use_module(library(http/thread_httpd)).
:- use_module(library(http/http_dispatch)).
:- use_module(library(http/http_json)).

:- use_module(rules).

% URL handlers.
:- http_handler('/api/v1/score', handle_total_score, []).
:- http_handler('/api/v2/score', handle_total_score_v2, []).

handle_total_score_v2(Request) :-
    http_read_json_dict(Request, Query),
    total_player_score_v2(Query,Solution),
    reply_json_dict(Solution).

handle_total_score(Request) :-
    http_read_json_dict(Request, Query),
    total_player_score(Query,Solution),
    reply_json_dict(Solution).

server(Port) :-
    http_server(http_dispatch, [port(Port)]).

server_loop :-
    server(42069),
    thread_get_message(quit).

:- initialization(server_loop, main).
