module TupleSet = Aoc.IntTupleSet

module Sensor : sig
  type t

  val create : int * int -> int * int -> t
  val parse_lines : string list -> t list
  val print : t -> string
  val distance_to_beacon : t -> int
  val coords_at : int -> t -> (int * int) list
end = struct
  type t = { x : int; y : int; b_x : int; b_y : int }

  let create (x, y) (b_x, b_y) = { x; y; b_x; b_y }

  let parse line =
    let split = String.split_on_char ' ' line in
    let s_x = List.nth (String.split_on_char '=' (List.nth split 2)) 1 in
    let s_x = String.sub s_x 0 (String.length s_x - 1) |> int_of_string in
    let s_y = List.nth (String.split_on_char '=' (List.nth split 3)) 1 in
    let s_y = String.sub s_y 0 (String.length s_y - 1) |> int_of_string in
    let b_x = List.nth (String.split_on_char '=' (List.nth split 8)) 1 in
    let b_x = String.sub b_x 0 (String.length b_x - 1) |> int_of_string in
    let b_y =
      List.nth (String.split_on_char '=' (List.nth split 9)) 1 |> int_of_string
    in
    { x = s_x; y = s_y; b_x; b_y }

  let parse_lines = List.map parse

  let print sensor =
    Printf.sprintf "Sensor: (%d, %d), Beacon: (%d, %d)" sensor.x sensor.y
      sensor.b_x sensor.b_y

  let distance_to_beacon sensor =
    abs (sensor.x - sensor.b_x) + abs (sensor.y - sensor.b_y)

  let coords_at y sensor =
    let distance = distance_to_beacon sensor in
    let distance_to_y = abs (distance - abs (sensor.y - y)) in
    let amount_of_ys = 1 + (2 * distance_to_y) in
    List.init amount_of_ys (fun i -> (sensor.x - (amount_of_ys / 2) + i, y))
end

let part_1 sensors y =
  List.map (Sensor.coords_at y) sensors
  |> List.flatten |> TupleSet.of_list |> TupleSet.elements |> List.length

let () =
  let file = Aoc.read_lines "./input/day_15/input.txt" in
  let sensors = Sensor.parse_lines file in
  let part_1 = part_1 sensors 2000000 in
  Printf.printf "Part 1: %d\n" part_1
