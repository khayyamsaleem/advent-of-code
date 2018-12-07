let explode s =
  let rec exp i l =
    if i < 0 then l else exp (i - 1) (s.[i] :: l) in
  exp (String.length s - 1) [];;

let rec check_string arr str (x,y) =
    match str with
    | [] -> (x,y)
    | hd::tl ->
            begin
                Array.set arr (Char.code hd - 97) (arr.(Char.code hd - 97) + 1);
                match arr.(Char.code hd - 97) with
                | 2 -> check_string arr tl (x+1,y)
                | 3 -> check_string arr tl (x-1,y+1)
                | _ -> check_string arr tl (x,y)
            end;;

let rec check_all arr i l acc =
    match (i,l) with
    | (x,y) when x == y -> acc
    | (x,y) -> check_all arr (x+1) y ([(check_string (Array.make 26 0) (explode arr.(x)) (0,0))] @ acc);;

let rec checksum lst (x,y) =
    match lst with
    | [] -> x*y
    | (a,b)::tl -> checksum tl ((if a > 0 then x+1 else x), (if b > 0 then y+1 else y));;

let () =
    print_int @@ checksum (check_all Sys.argv 1 (Array.length Sys.argv) []) (0,0)
