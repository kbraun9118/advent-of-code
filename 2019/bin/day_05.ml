let () =
  let input = Lib.get_input_lines "05" |> List.hd in
  let intcode = Lib.IntCode.create ~argument:1 input in
  let intcode = Lib.IntCode.execute intcode in
  let () = Printf.printf "Part 1: %d\n" @@ Lib.IntCode.output intcode in
  let intcode = Lib.IntCode.create ~argument:5 input in
  let intcode = Lib.IntCode.execute intcode in
  let () = Printf.printf "Part 2: %d\n" @@ Lib.IntCode.output intcode in
  ()
