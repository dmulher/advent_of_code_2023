extern crate test;

pub fn main(contents: String) -> usize {
  get_reflections(contents)
}

fn get_reflections(contents: String) -> usize {
  let mut patterns: Vec<Vec<Vec<u8>>> = vec![];
  let mut idx = 0;
  let mut size = 0;
  contents.lines().for_each(|l| {
    if l.is_empty() {
      idx += 1;
      size = 0;
    } else {
      if size == 0 {
        patterns.push(vec![]);
      }
      patterns[idx].push(l.bytes().collect::<Vec<u8>>());
      size += 1;
    }
  });

  patterns.into_iter().map(|p| {
    // let row_symm_0 = find_symmetry(&p, 1, MatchType::Rows, 0);
    // let row_symm_1 = find_symmetry(&p, 1, MatchType::Rows, 1);

    // let col_symm_0 = find_symmetry(&p, 1, MatchType::Cols, 0);
    // let col_symm_1 = find_symmetry(&p, 1, MatchType::Cols, 1);

    // let row_matching = row_symm_0 == row_symm_1;
    // let col_matching = col_symm_0 == col_symm_1;
    // if row_matching {
    //   if row_symm_0 != 0 || col_matching {
    //     println!("There's a problem");
    //   }
    // }
    // let row_norm = find_symmetry(&p, 1, MatchType::Rows, 1);
    // let col_norm = find_symmetry(&p, 1, MatchType::Cols, 1);
    // let row_symp = get_symmetry_val(&p, 1);
    // let col_symp = get_symmetry_val(&transpose(&p), 1);

    // println!("Puzzle {idx}: get_symmetry_val for rows ({row_symp}) and cols ({col_symp}), being lame for rows({row_norm}) and cols({col_norm})");
    // if row_symp != row_norm || col_symp != col_norm {
    //   for r in p {
    //     let line: String = r.into_iter().map(|c| char::from(c)).collect();
    //     println!("{line}");
    //   }
    // }
    // return row_symp * 100 + col_symp;
    match get_symmetry_val(&p, 1) {
      0 => get_symmetry_val(&transpose(&p), 1),
      val => 100*val,
    }

    // match find_symmetry(&p, 1, MatchType::Cols, 1) {
    //   0 => find_symmetry(&p, 1, MatchType::Rows, 1) * 100,
    //   val => val
    // }
  }).sum()
}

fn transpose(v: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
  let len = v[0].len();
  let mut iters: Vec<_> = v.iter().map(|n| n.into_iter()).collect();
  (0..len)
    .map(|_| {
      iters
        .iter_mut()
        .map(|n| *(n.next().unwrap()))
        .collect::<Vec<u8>>()
    })
    .collect()
}

fn is_mirror(ns: &[u32], i: usize, max_mistake: u32) -> bool {
  (0..i)
    .rev()
    .zip(i..ns.len())
    .map(|(a, b)| (ns[a] ^ ns[b]).count_ones())
    .sum::<u32>()
    == max_mistake
}

fn get_symmetry_val(pattern: &Vec<Vec<u8>>, max_mistake: u32) -> usize {
  let rows: Vec<u32> = pattern.into_iter().map(|line| {
    line.into_iter().fold(0, |acc, c| (acc << 1) | (*c == b'.') as u32)
  }).collect();
  for i in 1..pattern.len() {
    if is_mirror(&rows, i, max_mistake) {
      return i;
    }
  }
  0
}

// #[derive(Debug)]
// enum MatchType {
//   Rows,
//   Cols
// }

// fn find_symmetry(pattern: &Vec<Vec<u8>>, start_idx: usize, match_type: MatchType, max_mistake: usize) -> usize {
//   let row_count = pattern.len();
//   let col_count = pattern[0].len();

//   let mut match_from: Option<usize> = None;
//   let mut matching: usize = 0;
//   let mut differences = 0;

//   let (outer, inner) = match match_type {
//     MatchType::Rows => (row_count, col_count),
//     _ => (col_count, row_count)
//   };

//   let mut idx = start_idx;
//   while idx < outer {
//     // print!("{:?}, idx = {idx}, ", match_type);
//     match match_from {
//       Some(match_idx) => {
//         // print!("with prev match {match_idx}, ");
//         if matching > match_idx {
//           if differences == max_mistake {
//             // println!("Succeeded at {matching} reflected rows on {:?}", match_type);
//             return matching;
//           } else {
//             // println!("hit the limit, back to {}", match_idx + 2);
//             matching = 0;
//             match_from = None;
//             idx = match_idx + 2;
//             continue;
//           }
//         }
//         differences += match match_type {
//           MatchType::Rows => differences_in_rows(idx, match_idx-matching, pattern, inner),
//           MatchType::Cols => differences_in_cols(idx, match_idx-matching, pattern, inner),
//         };
//         if differences > max_mistake {
//           // println!("had too many differences: {differences}.");
//           differences = 0;
//           matching = 0;
//           match_from = None;
//           idx = match_idx + 2;
//         } else {
//           // println!("continuing on.");
//           matching += 1;
//           idx += 1;
//         }
//       },
//       None => {
//         let match_idx = idx - 1;
//         differences += match match_type {
//           MatchType::Rows => differences_in_rows(idx, match_idx, pattern, inner),
//           MatchType::Cols => differences_in_cols(idx, match_idx, pattern, inner),
//         };
//         if differences <= max_mistake {
//           // println!("with an appopriate amount of differences: {differences}.");
//           matching = 1;
//           match_from = Some(match_idx);
//         } else {
//           // println!("with an inappopriate amount of differences: {differences}.");
//           differences = 0;
//         }
//         idx += 1;
//       }
//     }
//   }

//   if let Some(match_idx) = match_from {
//     if differences == max_mistake {
//       // println!("Succeeded at {matching} reflected rows on {:?}, starting at {match_idx}", match_type);
//       return match_idx + 1;
//     } else {
//       find_symmetry(pattern, match_idx + 2, match_type, max_mistake);
//     }
//   }
//   0
// }

// fn differences_in_rows(y: usize, other_y: usize, pattern: &Vec<Vec<u8>>, col_count: usize) -> usize {
//   let mut differences = 0;
//   for x in 0..col_count {
//     if pattern[y][x] != pattern[other_y][x] {
//         differences += 1;
//     }
//   }
//   differences
// }

// fn differences_in_cols(x: usize, other_x: usize, pattern: &Vec<Vec<u8>>, row_count: usize) -> usize {
//   let mut differences = 0;
//   for y in 0..row_count {
//     if pattern[y][x] != pattern[y][other_x] {
//       differences += 1;
//     }
//   }
//   differences
// }

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 13; // 31008 < ans < 43551
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_13_b() {
    const EXAMPLE_ANSWER: usize = 400;
    const ANSWER: Option<usize> = Some(31947);
    match utils::run_method::<usize>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_13_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
