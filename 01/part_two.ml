module SS = Set.Make(
    struct
        let compare = Pervasives.compare
        type t = int
    end
);;


let rec check_twice seen arr i last acc =
    match (i, last) with
    | (x,y) when x == y -> check_twice seen arr 1 last acc
    | (x,y) ->
            begin
                let sum = acc + (int_of_string arr.(i)) in
                if SS.mem sum seen
                then sum
                else check_twice (SS.add sum seen) arr (i+1) last sum
            end

let () =
    print_int @@ check_twice (SS.singleton 0) Sys.argv 1 (Array.length Sys.argv) 0
