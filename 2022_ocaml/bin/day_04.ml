(* let seq_of_range start en = Seq.init (en - start + 1) (fun x -> x + start) *)
type range = { start : int; back : int }

let regex = Re.Perl.compile @@ Re.Perl.re {|(\d+)-(\d+),(\d+)-(\d+)|}
let range_init start back = { start; back }

let parse_line line =
  let groups = Re.exec regex line in
  let first_start = int_of_string @@ Re.Group.get groups 1 in
  let first_back = int_of_string @@ Re.Group.get groups 2 in
  let second_start = int_of_string @@ Re.Group.get groups 3 in
  let second_back = int_of_string @@ Re.Group.get groups 4 in
  (range_init first_start first_back, range_init second_start second_back)

let fully_overlaps (left, right) =
  if left.start <= right.start && left.back >= right.back then true
  else if right.start <= left.start && right.back >= left.back then true
  else false

let partially_overlaps (left, right) =
  if left.start <= right.start && left.back >= right.start then true
  else if right.start <= left.start && right.back >= left.start then true
  else false

let counter_overlaps overlap_f list =
  list |> List.map parse_line |> List.filter overlap_f |> List.length

let () =
  let lines = Aoc.read_lines "./input/day_04/input.txt" in
  let () =
    Printf.printf "Part 1: %d\n" @@ counter_overlaps fully_overlaps lines
  in
  Printf.printf "Part 2: %d\n" @@ counter_overlaps partially_overlaps lines
