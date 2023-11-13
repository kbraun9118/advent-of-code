module RopeBridge : sig
  type t = { head : int * int; tails : (int * int) array }

  val create : int -> t
  val move : Aoc.direction -> t -> t
  val tail_end : t -> int * int
end = struct
  type t = { head : int * int; tails : (int * int) array }

  let create tailLength =
    { head = (0, 0); tails = Array.init tailLength (fun _ -> (0, 0)) }

  let subtract_tuple (x1, y1) (x2, y2) = (x1 - x2, y1 - y2)

  let move direction t =
    let x, y = t.head in
    let new_head =
      match direction with
      | Aoc.Up -> (x, y + 1)
      | Down -> (x, y - 1)
      | Left -> (x - 1, y)
      | Right -> (x + 1, y)
    in
    let tails = t.tails in
    let () =
      for i = 0 to Array.length tails - 1 do
        let x, y = tails.(i) in
        let prev = if i = 0 then new_head else tails.(i - 1) in
        let subtracted = subtract_tuple prev t.tails.(i) in
        let new_x, new_y =
          match subtracted with
          | 0, 0 | 1, 0 | -1, 0 | 0, 1 | 0, -1 | -1, -1 | 1, 1 | -1, 1 | 1, -1
            ->
              t.tails.(i)
          | 2, 0 -> (x + 1, y)
          | -2, 0 -> (x - 1, y)
          | 0, 2 -> (x, y + 1)
          | 0, -2 -> (x, y - 1)
          | 1, 2 | 2, 1 | 2, 2 -> (x + 1, y + 1)
          | -1, 2 | -2, 1 | -2, 2 -> (x - 1, y + 1)
          | 1, -2 | 2, -1 | 2, -2 -> (x + 1, y - 1)
          | -1, -2 | -2, -1 | -2, -2 -> (x - 1, y - 1)
          | x, y ->
              let new_x, new_y = new_head in
              failwith
              @@ Printf.sprintf
                   "Incorrect position newhead: (%d, %d) i: %d (%d, %d)" new_x
                   new_y i x y
        in
        tails.(i) <- (new_x, new_y)
      done
    in
    { head = new_head; tails }

  let tail_end t = t.tails.(Array.length t.tails - 1)
end

let run_sim lines rope_length =
  let rec move_with_line direction amount set bridge =
    match amount with
    | 0 -> (set, bridge)
    | amount ->
        let bridge = RopeBridge.move direction bridge in
        let set = Aoc.IntTupleSet.add (RopeBridge.tail_end bridge) set in
        move_with_line direction (amount - 1) set bridge
  in
  let rec move_lines lines set bridge =
    match lines with
    | [] -> (set, bridge)
    | (direction, amount) :: tl ->
        let set, bridge = move_with_line direction amount set bridge in
        move_lines tl set bridge
  in
  let bridge = RopeBridge.create rope_length in
  let set = Aoc.IntTupleSet.empty in
  let set, _ = move_lines lines set bridge in
  Aoc.IntTupleSet.to_list set |> List.length

let parse_line line =
  let direction, amount =
    match String.split_on_char ' ' line with
    | [ direction; amount ] -> (direction, amount)
    | _ -> failwith "Invalid input"
  in
  let direction =
    match direction with
    | "R" -> Aoc.Right
    | "L" -> Left
    | "D" -> Down
    | "U" -> Up
    | _ -> failwith "Invalid direction"
  in
  (direction, int_of_string amount)

let part_1 lines = run_sim lines 1
let part_2 lines = run_sim lines 9

let () =
  let lines =
    Aoc.read_lines "./input/day_09/input.txt" |> List.map parse_line
  in
  let () = Printf.printf "Part 1: %d\n" @@ part_1 lines in
  Printf.printf "Part2: %d\n" @@ part_2 lines
