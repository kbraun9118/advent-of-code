fn main() {
  let mut vec = vec![3, 7];
  let mut elf1 = 0;
  let mut elf2 = 1;

  let input = vec![7, 0, 2, 8, 3, 1];
//    let input = vec![9, 2, 5, 1, 0];


  while !vec_equals(&vec, &input) {
      next_score(&mut vec, elf1, elf2);
      elf1 = get_elf_next(elf1, &vec);
      elf2 = get_elf_next(elf2, &vec);
  }

  println!("{}", vec.len() - input.len())
}

fn next_score(vec: &mut Vec<usize>, elf1: usize, elf2: usize) {
  let score = vec[elf1] + vec[elf2];
  if score > 9 {
      vec.push(1);
      vec.push(score % 10);
  } else {
      vec.push(score)
  }
}

fn get_elf_next(elf: usize, vec: &Vec<usize>) -> usize {
  (elf + vec[elf] + 1) % vec.len()
}

fn vec_equals(vec: &Vec<usize>, input: &Vec<usize>) -> bool {
  if vec.len() - 1  < input.len() {
      return false;
  }
  
  let vec = &vec[(vec.len() - input.len() - 1)..vec.len()];

  let mut value1 = true;

  for i in 0..input.len() {
      if vec[i] != input[i] { value1 = false; };
  };

  value1
}