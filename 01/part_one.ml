open Printf

let rec sum arr fst lst acc =
    match (fst,lst) with
       | (a,b) when a == b -> acc
       | (a,b) -> sum arr (1 + a) lst (int_of_string arr.(a) + acc);;

let () = print_int @@ sum Sys.argv 1 (Array.length Sys.argv) 0
