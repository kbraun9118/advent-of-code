open Lib

let () =
  let input = get_input_lines "09" |> List.hd in
  let intcode = IntCode.create ~argument:1 input in
  let part1 = IntCode.execute_until_output intcode |> IntCode.output in
  let intcode = IntCode.create ~argument:2 input in
  let part2 = IntCode.execute intcode |>IntCode.output in
  Printf.printf "Part 1: %d\nPart 2: %d\n" part1 part2
