module InfArr = struct
  type 'a t = { mutable items : 'a array; creator : int -> 'a }

  let from_arr arr default = { items = arr; creator = (fun _ -> default) }

  let fill_arr idx arr =
    let len = Array.length arr.items in
    if idx < len then ()
    else
      let appended_len = idx - len + 1 in
      let appended = Array.init appended_len arr.creator in
      arr.items <- Array.append arr.items appended

  let nth idx arr =
    fill_arr idx arr;
    arr.items.(idx)

  let to_arr arr = arr.items
  let ( .%() ) arr idx = nth idx arr

  let update idx value arr =
    fill_arr idx arr;
    arr.items.(idx) <- value

  let ( .%()<- ) arr idx value = update idx value arr
end

type t = {
  instructions : int InfArr.t;
  instruction_pointer : int;
  relative_base : int;
  input : int list;
  output : string;
  complete : bool;
}

let create ?(argument = 0) input =
  let ints = String.split_on_char ',' input |> List.map int_of_string in
  {
    instructions = InfArr.from_arr (Array.of_list ints) 0;
    instruction_pointer = 0;
    relative_base = 0;
    input = [ argument ];
    output = "";
    complete = false;
  }

let parse_op_code instruction =
  let instruction = Printf.sprintf "%05d" instruction in
  let opcode =
    String.sub instruction (String.length instruction - 2) 2 |> int_of_string
  in
  let params = String.sub instruction 0 (String.length instruction - 2) in
  let params =
    List.init (String.length params) (String.get params)
    |> List.map (String.make 1)
    |> List.map int_of_string |> List.rev
  in
  (opcode, params)

let param_at_position position params intcode =
  let mode = List.nth params (position - 1) in
  match mode with
  | 0 ->
      intcode.instructions.InfArr.%(intcode.instructions.InfArr.%(intcode
                                                                    .instruction_pointer
                                                                  + position))
  | 1 -> intcode.instructions.InfArr.%(intcode.instruction_pointer + position)
  | 2 ->
      intcode.instructions.InfArr.%(intcode.instructions.InfArr.%(intcode
                                                                    .instruction_pointer
                                                                  + position)
                                    + intcode.relative_base)
  | _ -> failwith "Invalid parameter mode"

let return_at_position position params return_value intcode =
  let mode = List.nth params (position - 1) in
  match mode with
  | 0 ->
      intcode.instructions.InfArr.%(intcode.instructions.InfArr.%(intcode
                                                                    .instruction_pointer
                                                                  + position)) <-
        return_value
  | 1 ->
      intcode.instructions.InfArr.%(intcode.instruction_pointer + position) <-
        return_value
  | 2 ->
      intcode.instructions.InfArr.%(intcode.instructions.InfArr.%(intcode
                                                                    .instruction_pointer
                                                                  + position)
                                    + intcode.relative_base) <- return_value
  | _ -> failwith "Invalid parameter mode"

let debug = false
let print_debug input = if debug then print_endline input

let push_input input intcode =
  { intcode with input = intcode.input @ [ input ] }

let clear_output intcode = { intcode with output = "" }
let is_complete intcode = intcode.complete

let rec execute_until_output intcode =
  let opcode, params =
    parse_op_code intcode.instructions.InfArr.%(intcode.instruction_pointer)
  in
  match opcode with
  | 99 ->
      print_debug "END";
      { intcode with complete = true }
  (* Addition *)
  | 1 ->
      print_debug "Adding";
      let left = param_at_position 1 params intcode in
      let right = param_at_position 2 params intcode in
      return_at_position 3 params (left + right) intcode;
      execute_until_output
        { intcode with instruction_pointer = intcode.instruction_pointer + 4 }
  (* Multiplication *)
  | 2 ->
      print_debug "Multiplication";
      let left = param_at_position 1 params intcode in
      let right = param_at_position 2 params intcode in
      return_at_position 3 params (left * right) intcode;
      execute_until_output
        { intcode with instruction_pointer = intcode.instruction_pointer + 4 }
      (* Store Input *)
  | 3 ->
      print_debug "Taking Input";
      let input = intcode.input |> List.hd in
      return_at_position 1 params input intcode;
      execute_until_output
        {
          intcode with
          instruction_pointer = intcode.instruction_pointer + 2;
          input = List.tl intcode.input;
        }
      (* Store output *)
  | 4 ->
      print_debug "Adding to output";
      {
        intcode with
        instruction_pointer = intcode.instruction_pointer + 2;
        output =
          intcode.output ^ string_of_int @@ param_at_position 1 params intcode;
      }
      (* Jump If True *)
  | 5 ->
      print_debug "Jumping if true";
      let if_zero = param_at_position 1 params intcode in
      let instruction_pointer =
        if if_zero <> 0 then param_at_position 2 params intcode
        else intcode.instruction_pointer + 3
      in
      execute_until_output { intcode with instruction_pointer }
      (* Jump If False *)
  | 6 ->
      print_debug "Jumping if false";
      let if_not_zero = param_at_position 1 params intcode in
      let instruction_pointer =
        if if_not_zero = 0 then param_at_position 2 params intcode
        else intcode.instruction_pointer + 3
      in
      execute_until_output { intcode with instruction_pointer }
      (* Less Than *)
  | 7 ->
      print_debug "Testing Less Than";
      let left = param_at_position 1 params intcode in
      let right = param_at_position 2 params intcode in
      if left < right then return_at_position 3 params 1 intcode
      else return_at_position 3 params 0 intcode;
      execute_until_output
        { intcode with instruction_pointer = intcode.instruction_pointer + 4 }
      (* Equal To *)
  | 8 ->
      print_debug "Testing Equal To";
      let left = param_at_position 1 params intcode in
      let right = param_at_position 2 params intcode in
      if left = right then return_at_position 3 params 1 intcode
      else return_at_position 3 params 0 intcode;
      execute_until_output
        { intcode with instruction_pointer = intcode.instruction_pointer + 4 }
      (* Adjust Relative Base *)
  | 9 ->
      print_debug "Testing Adjust Relative Base";
      let offset = param_at_position 1 params intcode in
      execute_until_output
        {
          intcode with
          instruction_pointer = intcode.instruction_pointer + 2;
          relative_base = intcode.relative_base + offset;
        }
  | _ ->
      failwith
      @@ Printf.sprintf "Invalid opcode: %d at position: %d"
           intcode.instructions.InfArr.%(intcode.instruction_pointer)
           intcode.instruction_pointer

let rec execute intcode =
  let intcode = execute_until_output intcode in
  if is_complete intcode then intcode else execute intcode

let return_code intcode = intcode.instructions.InfArr.%(0)

let opcodes intcode =
  Array.iter
    (fun x -> Printf.printf "%d " x)
    (InfArr.to_arr intcode.instructions)

let replace_value index value intcode =
  intcode.instructions.InfArr.%(index) <- value;
  intcode

let output intcode =
  String.to_seq intcode.output
  |> Seq.drop_while (( = ) '0')
  |> String.of_seq |> String.trim |> int_of_string_opt
  |> Option.value ~default:0
