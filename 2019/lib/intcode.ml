type t = { instructions : int array; instruction_pointer : int }

let create ints = { instructions = Array.of_list ints; instruction_pointer = 0 }

let rec execute intcode =
  match intcode.instructions.(intcode.instruction_pointer) with
  | 99 -> intcode
  | 1 ->
      let left =
        intcode.instructions.(intcode.instructions.(intcode.instruction_pointer + 1))
      in
      let right =
        intcode.instructions.(intcode.instructions.(intcode.instruction_pointer + 2))
      in
      let return_position =
        intcode.instructions.(intcode.instruction_pointer + 3)
      in
      intcode.instructions.(return_position) <- left + right;
      execute { intcode with instruction_pointer = intcode.instruction_pointer + 4 }
  | 2 ->
      let left =
        intcode.instructions.(intcode.instructions.(intcode.instruction_pointer + 1))
      in
      let right =
        intcode.instructions.(intcode.instructions.(intcode.instruction_pointer + 2))
      in
      let return_position =
        intcode.instructions.(intcode.instruction_pointer + 3)
      in
      intcode.instructions.(return_position) <- left * right;
      execute { intcode with instruction_pointer = intcode.instruction_pointer + 4 }
  | _ ->
      failwith
      @@ Printf.sprintf "Invalid opcode: %d at position: %d"
           intcode.instructions.(intcode.instruction_pointer)
           intcode.instruction_pointer

let return_code intcode = intcode.instructions.(0)

let opcodes intcode =
  Array.iter (fun x -> Printf.printf "%d " x) intcode.instructions

let replace_value index value intcode =
  intcode.instructions.(index) <- value;
  intcode
