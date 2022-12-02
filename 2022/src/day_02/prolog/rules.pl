:- module(rules, [total_player_score/2, total_player_score_v2/2]).

shape("rock").
shape("paper").
shape("scissors").

beats("scissors", "paper").
beats("paper", "rock").
beats("rock", "scissors").

play(X,Y) :-
    shape(X),
    shape(Y),
    beats(X,Y).

wins(X, X, "draw") :- shape(X).
wins(Me, Them, "win") :- play(Me,Them).
wins(Me, Them, "lose") :- play(Them,Me).

score("win", 6).
score("draw", 3).
score("lose", 0).

score("rock", 1).
score("paper", 2).
score("scissors", 3).

round_score(Me,Them,S) :-
    score(Me,MoveScore),
    wins(Me,Them,Outcome),
    score(Outcome, OutcomeScore),
    S is OutcomeScore+MoveScore.

total_player_score([], 0).
total_player_score([[Them,Me]|Rest], Total) :-
    total_player_score(Rest, RestScore),
    round_score(Me, Them, RoundScore),
    Total is RestScore + RoundScore.

total_player_score_v2([], 0).
total_player_score_v2([[Them,Outcome]|Rest], Total) :-
    total_player_score_v2(Rest, RestScore),
    wins(Me,Them,Outcome),
    round_score(Me,Them,RoundScore),
    Total is RestScore + RoundScore.
