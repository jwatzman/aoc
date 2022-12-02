type rps = Rock | Paper | Scissors

let fixed_xyz = (Rock, Paper, Scissors)
let parse_line line (x, y, z) =
  let first = match line.[0] with
    | 'A' -> Rock
    | 'B' -> Paper
    | 'C' -> Scissors
    | _ -> failwith "Invalid first char" in
  let second = match line.[2] with
    | 'X' -> x
    | 'Y' -> y
    | 'Z' -> z
    | _ -> failwith "Invalid second char" in
  (first, second)

type result = Win | Loss | Draw
let find_result = function
    | (them, us) when them = us -> Draw
    | (Rock, Paper) -> Win
    | (Paper, Scissors) -> Win
    | (Scissors, Rock) -> Win
    | _ -> Loss

let score ((_, second) as t) =
  let win_score = match find_result t with
    | Win -> 6
    | Draw -> 3
    | Loss -> 0 in
  let choice_score = match second with
    | Rock -> 1
    | Paper -> 2
    | Scissors -> 3 in
  win_score + choice_score

let tot_score = List.fold_left (fun s t -> s + score t) 0

let parse_stream ic =
  let rec parse_stream_helper ls =
    try
      let l = parse_line (input_line ic) fixed_xyz in
      parse_stream_helper (l::ls)
    with End_of_file ->
      ls
  in
  parse_stream_helper []

let main () =
  let ic = open_in Sys.argv.(1) in
  let parsed = parse_stream ic in
  let score = tot_score parsed in
  print_endline (string_of_int score);
  ()

let () = main ()
