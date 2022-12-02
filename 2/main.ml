type rps = Rock | Paper | Scissors
type result = Win | Loss | Draw

let parse_line line =
  let first = match line.[0] with
    | 'A' -> Rock
    | 'B' -> Paper
    | 'C' -> Scissors
    | _ -> failwith "Invalid first char" in
  let second = match line.[2] with
    | 'X' -> Loss
    | 'Y' -> Draw
    | 'Z' -> Win
    | _ -> failwith "Invalid second char" in
  (first, second)

let find_play = function
  | (them, Draw) -> them
  | (them, Win) -> (match them with
    | Rock -> Paper
    | Paper -> Scissors
    | Scissors -> Rock)
  | (them, Loss) -> (match them with
    | Rock -> Scissors
    | Paper -> Rock
    | Scissors -> Paper)

let result_to_play ((_, result) as t) = (result, find_play t)

let score (result, us) =
  let win_score = match result with
    | Win -> 6
    | Draw -> 3
    | Loss -> 0 in
  let choice_score = match us with
    | Rock -> 1
    | Paper -> 2
    | Scissors -> 3 in
  win_score + choice_score

let tot_score = List.fold_left (fun s t -> s + score t) 0

let parse_stream ic =
  let rec parse_stream_helper ls =
    try
      let l = parse_line (input_line ic) in
      parse_stream_helper (l::ls)
    with End_of_file ->
      ls
  in
  parse_stream_helper []

let main () =
  let ic = open_in Sys.argv.(1) in
  let parsed = parse_stream ic in
  let with_plays = List.map result_to_play parsed in
  let score = tot_score with_plays in
  print_endline (string_of_int score);
  ()

let () = main ()
