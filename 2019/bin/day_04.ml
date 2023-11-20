module CharTbl = Hashtbl.Make (Char)

let rec two_sequential = function
  | _ :: [] | [] -> false
  | first :: second :: _ when first = second -> true
  | _ :: tl -> two_sequential tl

let rec never_decreases = function
  | [] -> true
  | first :: second :: _ when first > second -> false
  | _ :: tl -> never_decreases tl

let never_three_in_a_row password =
  let table = CharTbl.create 0 in
  List.iter
    (fun c ->
      match CharTbl.find_opt table c with
      | Some v -> CharTbl.replace table c (v + 1)
      | None -> CharTbl.add table c 1)
    password;
  CharTbl.to_seq_values table |> Seq.find (( = ) 2) |> Option.is_some

let split_password password =
  let password = string_of_int password in
  List.init (String.length password) (String.get password)

let validate_password_part_1 password =
  two_sequential password && never_decreases password

let validate_password_part_2 password =
  validate_password_part_1 password && never_three_in_a_row password

let validate_password_range validation_f range =
  let left, right = Lib.split_once '-' range in
  let left, right = (int_of_string left, int_of_string right) in
  let seq = Seq.init (right - left + 1) (( + ) left) in
  Seq.map split_password seq |> Seq.filter validation_f |> Seq.length

let part_1 = validate_password_range validate_password_part_1
let part_2 = validate_password_range validate_password_part_2

let () =
  let input = Lib.get_input_lines "04" |> List.hd in
  let () = Printf.printf "Part 1: %d\n" @@ part_1 input in
  let () = Printf.printf "Part 2: %d\n" @@ part_2 input in
  ()
