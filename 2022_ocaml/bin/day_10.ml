module Instruction : sig
  type t = Noop | AddX of int

  val of_string : string -> t
  val build_instruction_list : t list -> t list
  val output_list : t list -> int list
end = struct
  type t = Noop | AddX of int

  let of_string string =
    if String.equal string "noop" then Noop
    else
      let _, num = Aoc.split_once ' ' string in
      AddX (int_of_string num)

  let build_instruction_list instructions =
    let rec build_instruction_list ret instructions =
      match instructions with
      | instruction :: tl -> (
          match instruction with
          | AddX x -> build_instruction_list ([ AddX x; Noop ] @ ret) tl
          | Noop -> build_instruction_list (Noop :: ret) tl)
      | [] -> List.rev ret
    in
    build_instruction_list [] instructions

  let output_list instructions =
    let rec output_list returned = function
      | [] ->
          let last = snd (List.nth returned 0) in
          let returned = (last, last) :: returned in
          List.rev_map fst returned
      | instruction :: tl -> (
          let prev = match returned with [] -> 1 | hd :: _ -> snd hd in
          match instruction with
          | Noop -> output_list ((prev, prev) :: returned) tl
          | AddX x -> output_list ((prev, prev + x) :: returned) tl)
    in
    output_list [] instructions
end

let part_1 output =
  List.filteri (fun i _ -> (i + 1 - 20) mod 40 = 0) output
  |> List.mapi (fun i _ -> (i * 40) + 20)
  |> List.map (fun i -> (i, List.nth output (i - 1)))
  |> List.fold_left (fun acc (i, x) -> acc + (i * x)) 0

let part_2 output =
  let rec part_2 cycle crt output =
    if cycle = 241 then crt
    else
      let register, rest =
        match output with
        | register :: tl -> (register, tl)
        | _ -> failwith "Cannot pop"
      in
      let position = (cycle - 1) mod 40 in
      let pixel =
        if
          register = position
          || register = position - 1
          || register = position + 1
        then "█"
        else " "
      in
      let pixel = if cycle mod 40 = 0 then pixel ^ "\n" else pixel in
      part_2 (cycle + 1) (crt ^ pixel) rest
  in
  part_2 1 "" output

let () =
  let output_list =
    Aoc.read_lines "./input/day_10/input.txt"
    |> List.map Instruction.of_string
    |> Instruction.build_instruction_list |> Instruction.output_list
  in
  let () = Printf.printf "Part 1: %d\n" @@ part_1 output_list in
  Printf.printf "Part2:\n%s\n" @@ part_2 output_list
