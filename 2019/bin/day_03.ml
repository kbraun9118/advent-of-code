module WireBoard : sig
  type t
end = struct
  module IntTuple = struct
    type t = int * int

    let compare (x1, y1) (x2, y2) = compare x1 x2 + compare y1 y2
  end

  module IntTupleSet = Set.Make (IntTuple)

  module Instructions = struct
    type instruction = Left of int | Right of int | Down of int | Up of int
    type t = instruction list

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
  end

  type t = IntTupleSet.t

  let create _lines =
    let set = IntTupleSet.empty in
    (* let () = List.map Instructions.create lines in *)
    set
end

let () = Printf.printf "Hello"
