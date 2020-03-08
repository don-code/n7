#[cfg(test)]
mod tests {
  use n7::numeronyze;

  #[test]
  fn test_single_word() {
    assert_eq!(numeronyze(' ', 4, "kubernetes".to_string()), "k8s");
  }

  #[test]
  fn test_multiple_words() {
    assert_eq!(numeronyze(' ', 4, "kubernetes internationalization".to_string()), "k8s i18n");
  }

  #[test]
  fn test_one_word_too_short() {
    assert_eq!(numeronyze(' ', 11, "kubernetes internationalization".to_string()), "kubernetes i18n");
  }
}
