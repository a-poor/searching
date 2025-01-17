use crate::transform::TextTransformer;
use crate::tokenize::Tokenizer;
use crate::filter::Filter;


pub struct Pipeline<Pre, T, F, Post>
where
  Pre: TextTransformer,
  T: Tokenizer,
  F: Filter,
  Post: TextTransformer,
{
  pre: Pre,
  tokenizer: T,
  filter: F,
  post: Post,
}

impl<Pre, T, F, Post> Pipeline<Pre, T, F, Post>
where
  Pre: TextTransformer,
  T: Tokenizer,
  F: Filter,
  Post: TextTransformer,
{
  pub fn new(pre: Pre, tokenizer: T, filter: F, post: Post) -> Self {
    Self { pre, tokenizer, filter, post }
  }

  pub fn process(&self, text: &str) -> Vec<String> {
    let text = self.pre.transform(text);
    let tokens = self.tokenizer.tokenize(&text);
    let filtered = self.filter.filter(&tokens);
    filtered.iter().map(|t| self.post.transform(t)).collect()
  }
}

