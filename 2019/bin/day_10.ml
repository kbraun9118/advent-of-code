let asteroids lines =
  let grid = List.map (fun line -> String.to_seq line |> List.of_seq) lines in
  List.mapi
    (fun y items ->
      List.mapi (fun x c -> (x, c)) items
      |> List.filter_map (fun (x, c) -> if c = '#' then Some (x, y) else None))
    grid
  |> List.concat

let list_without (x, y) = List.filter (fun (xi, yi) -> xi <> x || yi <> y)
let epsilon = 1.0e-8

let ( =. ) a b =
  let difference = abs_float (a -. b) in
  difference < epsilon

(* Going to compare slopes then see if distances eq *)
let slopes_equal (source_x, source_y) (dest_x, dest_y) (mid_x, mid_y) =
  if source_y = dest_y && source_y = mid_y then true
  else if source_x = dest_x && source_x = mid_x then true
  else
    let slopeAB =
      Float.div
        (float_of_int (dest_y - source_y))
        (float_of_int (dest_x - source_x))
    in
    let slopeAC =
      Float.div
        (float_of_int (mid_y - source_y))
        (float_of_int (mid_x - source_x))
    in
    slopeAB =. slopeAC

let distances_sum source dest mid =
  let distance (source_x, source_y) (dest_x, dest_y) =
    sqrt
      ((float_of_int (source_x - dest_x) ** 2.0)
      +. (float_of_int (source_y - dest_y) ** 2.0))
  in
  let ab = distance source dest in
  let ac = distance source mid in
  let bc = distance mid dest in
  (* Printf.printf *)
  (* "Distance AB: %f, Distance AC: %f, Distance BC: %f, AC + BC = %f, Are Eq = \ *)
     (*    %b\n" *)
  (*   ab ac bc (ac +. bc) *)
  (*   (ac +. bc =. ab); *)
  ab =. ac +. bc

let intercepts source dest mid =
  let distance = distances_sum source dest mid in
  let slope = slopes_equal source dest mid in
  (* Printf.printf "Source: (%d, %d), Dest: (%d, %d), Mid: (%d, %d)\n" (fst source) *)
  (*   (snd source) (fst dest) (snd dest) (fst mid) (snd mid); *)
  (* Printf.printf "Distances: %b, Slope: %b\n" distance slope; *)
  distance && slope

let count_viewable source asteroids =
  let without_source = list_without source asteroids in
  List.filter
    (fun dest ->
      List.exists
        (fun mid -> intercepts source dest mid)
        (list_without dest without_source)
      |> not)
    without_source
  |> List.length

let max_list = List.fold_left Int.max 0

let part_1 asteroids =
  List.map (fun source -> count_viewable source asteroids) asteroids

let angle_between (source_x, source_y) (dest_x, dest_y) =
  let cx = float_of_int source_x in
  let cy = float_of_int source_y in
  let p0x = cx in
  let p0y = cy +. 1.0 in
  let p1x = float_of_int dest_x in
  let p1y = float_of_int dest_y in
  let p0c = Float.sqrt (((cx -. p0x) ** 2.0) +. ((cy -. p0y) ** 2.0)) in
  let p1c = Float.sqrt (((cx -. p1x) ** 2.0) +. ((cy -. p1y) ** 2.0)) in
  let p0p1 = Float.sqrt (((p1x -. p0x) ** 2.0) +. ((p1y -. p0y) ** 2.0)) in
  let angle =
    Float.acos
      (((p1c *. p1c) +. (p0c *. p0c) -. (p0p1 *. p0p1)) /. (2.0 *. p1c *. p0c))
  in
  if dest_x - source_x >= 0 then angle else (2.0 *. Float.pi) -. angle

let () =
  (* let input = Lib.get_input_lines "10" in *)
  (* let asteroids = asteroids input in *)
  (* let viewable = part_1 asteroids in *)
  (* Printf.printf "Part 1: %d\n" (max_list viewable) *)
  Printf.printf "Angle: %f\n"
    (angle_between (0, 0) (0, -1) *. (180.0 /. Float.pi))
