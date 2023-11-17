module WireBoard : sig
  type t

  val create : string list -> t
  val intersections : t -> (int * int) Seq.t
end = struct
  module IntTuple = struct
    type t = int * int

    (* let compare (x1, y1) (x2, y2) = compare x1 x2 + compare y1 y2 *)
    let equal (x1, y1) (x2, y2) = x1 = x2 && y1 = y2
    let hash (x, y) = (Int.hash x * 31) + Int.hash y
  end

  module IntTupleHashtbl = Hashtbl.Make (IntTuple)
  module IntSet = Set.Make (Int)

  module Instruction = struct
    type instruction = Left of int | Right of int | Down of int | Up of int

    let instruction_of_string inst =
      let dir = inst.[0] in
      let amount =
        String.sub inst 1 (String.length inst - 1) |> int_of_string
      in
      match dir with
      | 'R' -> Right amount
      | 'D' -> Down amount
      | 'L' -> Left amount
      | 'U' -> Up amount
      | _ -> failwith "Invalid direction"

    let create line =
      String.split_on_char ',' line |> List.map instruction_of_string

    let to_seq_from_point (x, y) instruction =
      match instruction with
      | Right amount -> Seq.init amount (fun i -> (x + 1 + i, y))
      | Left amount -> Seq.init amount (fun i -> (x - 1 - i, y))
      | Up amount -> Seq.init amount (fun i -> (x, y + 1 + i))
      | Down amount -> Seq.init amount (fun i -> (x, y - 1 - i))
  end

  type t = IntSet.t IntTupleHashtbl.t

  let create_wire table instructions wire_name =
    let add_wire_segment coord instruction table =
      let seq = Instruction.to_seq_from_point coord instruction in
      let table_inserter coord =
        match IntTupleHashtbl.find_opt table coord with
        | Some occurances ->
            IntTupleHashtbl.add table coord (IntSet.add wire_name occurances)
        | None ->
            IntTupleHashtbl.add table coord
              (IntSet.empty |> IntSet.add wire_name)
      in
      Seq.iter table_inserter seq;
      List.of_seq seq |> List.rev |> List.hd
    in
    let rec add_instructions coord instructions table =
      match instructions with
      | instruciton :: tl ->
          let coord = add_wire_segment coord instruciton table in
          add_instructions coord tl table
      | [] -> table
    in
    add_instructions (0, 0) instructions table

  let create lines =
    let table = IntTupleHashtbl.create 0 in
    let instructions = List.map Instruction.create lines in
    List.mapi (fun i instruction -> (i, instruction)) instructions
    |> List.fold_left
         (fun acc (i, instruction) -> create_wire acc instruction i)
         table

  let intersections board =
    IntTupleHashtbl.to_seq board
    |> Seq.filter (fun (_, occurances) -> IntSet.cardinal occurances > 1)
    |> Seq.map fst

  let distance_to_intersections instructions board = 
    let instructions = List.map Instruction.create instructions in
    let left = List.hd instructions in
    let right = List.hd @@ List.tl instructions in
    let intersections = intersections board in
    let distance_to_intersection instructions coord board = 
      match instructions with
      | [] -> failwith "Cannot find intersection"
      | instruction :: tl -> 
          let seq = Instruction.to_seq_from_point coord
          match Seq.find_index (fun )
      in
    ()
end


let part_1 board =
  WireBoard.intersections board
  |> Seq.fold_left (fun acc (x, y) -> min acc (abs x + abs y)) Int.max_int

let part_2 board instructions =
  List.mapi
    (fun i instructions ->
      WireBoard.first_intersection_along instructions i board)
    instructions
  |> List.fold_left ( + ) 0

let () =
  let lines = Lib.get_test_lines "03" in
  let board = WireBoard.create lines in
  let () = Printf.printf "Part 1: %d\n" (part_1 board) in
  let () = Printf.printf "Part 2: %d\n" (part_2 board lines) in
  ()
