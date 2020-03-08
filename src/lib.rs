pub fn numeronyze(delimiter: char, shortest_string: i32, string: String) -> String {
  let mut output : Vec<String> = Vec::new();

  // input string may not already be split according to grammar rules

  for token in string.split(delimiter).collect::<Vec<&str>>() {
    if token.len() > (shortest_string as usize) {
      let mut output_token = String::new();
      let mut chars = token.chars();

      output_token.push(chars.next().unwrap());
      output_token.push_str(&(token.len() - 2).to_string());
      output_token.push(chars.last().unwrap());

      output.push(output_token);
    } else {
      output.push(token.to_string());
    }

  }

  return output.join(&delimiter.to_string());
}

