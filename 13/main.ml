type elem = Num of int | List of elem list

let explode s =
	let rec exp n l =
		if n < 0 then l else exp (n-1) (s.[n]::l) in
	exp ((String.length s) - 1) []

let rec parse_list = function
	| ']'::rest -> ([], rest)
  | l ->
		let (elem, rest) = parse_elem l in
		(*let ','::rest = rest in*)
		let (parsed, rest) = parse_list rest in
		(elem::parsed, rest)

and parse_elem = function
	| ','::rest -> parse_elem rest
	| '1'::'0'::rest -> (Num 10, rest)
	| '['::rest ->
		let (l, rest) = parse_list rest in
		(List l, rest)
	| n::rest -> (Num (int_of_string (String.make 1 n)), rest)
	| _ -> failwith "Empty elem"

let parse_line s = match parse_elem (explode s) with
	| (p, []) -> p
	| _ -> failwith "Did not consume all"

let parse_packet ic =
	let p1 = parse_line (input_line ic) in
	let p2 = parse_line (input_line ic) in
	(p1, p2)

let rec parse_stream ic n =
	let p = (parse_packet ic, n) in
	try
		let _ = input_line ic in
		p::(parse_stream ic (n+1))
	with End_of_file ->
		[p]

let ensure_list elem = match elem with
	| List _ -> elem
	| _ -> List [elem]

let rec compare_elems = function
	| (Num n1, Num n2) -> if n1 < n2 then Some true else if n1 > n2 then Some false else None
	| (List l1, List l2) -> compare_lists (l1, l2)
	| (e1, e2) -> compare_elems (ensure_list e1, ensure_list e2)

and compare_lists = function
	| ([], []) -> None
	| (e1::r1, e2::r2) -> (match compare_elems (e1, e2) with
		| None -> compare_lists (r1, r2)
		| r -> r)
	| (_, []) -> Some false
	| ([], _) -> Some true

let rec tally = function
	| [] -> 0
	| (p, n)::rest -> 
		let r = tally rest in
		match compare_elems p with
			| Some true -> n + r
			| Some false -> r
			| None -> failwith "Did not compare"

let main () =
  let ic = open_in Sys.argv.(1) in
	let ps = parse_stream ic 1 in
	let n = tally ps in
	let () = print_endline (string_of_int n) in
	()

let () = main ()
