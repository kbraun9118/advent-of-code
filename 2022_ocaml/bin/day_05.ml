let split_input lines =
  let rec split_input acc = function
    | [] -> failwith "Cannot split input"
    | hd :: tl ->
        if String.equal "" hd then (acc, tl) else split_input (hd :: acc) tl
  in
  split_input [] lines

let rec list_of_queues len =
  if len == 0 then [] else Stack.create () :: list_of_queues (len - 1)

let init_stacks lines =
  let rec init_stacks stacks lines =
    match lines with
    | [] -> stacks
    | hd :: tl ->
        let add_to_stack index stack =
          let item = hd.[(index * 4) + 1] in
          if item != ' ' then Stack.push item stack
        in
        let () = List.iteri add_to_stack stacks in
        init_stacks stacks tl
  in
  match lines with
  | [] -> failwith "Cannot parse empty stacks"
  | hd :: tl ->
      let hd_len = ((String.length hd - 1) / 4) + 1 in
      init_stacks (list_of_queues hd_len) tl

module Instruction = struct
  type t = { move : int; from : int; into : int }

  let regex = Re.compile @@ Re.Perl.re {|move (\d+) from (\d+) to (\d+)|}

  let create line =
    let group = Re.exec regex line in
    let move = Re.Group.get group 1 |> int_of_string in
    let from = Re.Group.get group 2 |> int_of_string in
    let from = from - 1 in
    let into = Re.Group.get group 3 |> int_of_string in
    let into = into - 1 in
    { move; from; into }
end

let tops stacks =
  let rec tops stacks top =
    match stacks with
    | [] -> List.rev top
    | hd :: tl -> tops tl @@ (Stack.top hd :: top)
  in
  let tops = tops stacks [] in
  String.concat "" (List.map (String.make 1) tops)

let rec move_to_1 stacks (instruction : Instruction.t) n =
  if n != 0 then
    let item = Stack.pop (List.nth stacks instruction.from) in
    let () = Stack.push item @@ List.nth stacks instruction.into in
    move_to_1 stacks instruction (n - 1)
  else ()

let move_to_2 stacks instruction n =
  let rec move_to_2 stacks (instruction : Instruction.t) n items =
    if n != 0 then
      let item = Stack.pop (List.nth stacks instruction.from) in
      move_to_2 stacks instruction (n - 1) (item :: items)
    else
      let f item = Stack.push item @@ List.nth stacks instruction.into in
      List.iter f items
  in
  move_to_2 stacks instruction n []

let rec mover stacks (instructions : Instruction.t list) move_fun =
  match instructions with
  | [] -> tops stacks
  | hd :: tl ->
      let () = move_fun stacks hd hd.move in
      mover stacks tl move_fun

let () =
  let stacks_input, instructions =
    Aoc.read_lines "./input/day_05/input.txt" |> split_input
  in
  let stacks = init_stacks stacks_input in
  let instructions = List.map Instruction.create instructions in
  let () =
    Printf.printf "Part 1: %s\n" @@ mover stacks instructions move_to_1
  in
  let stacks = init_stacks stacks_input in
  Printf.printf "Part 2: %s\n" @@ mover stacks instructions move_to_2
