mod number_collector;
mod special_collector;
mod word_collector;

use crate::code_stream::CodeStream;
use crate::token::TokenKind;

pub use number_collector::NumberCollector;
pub use special_collector::SpecialCollector;
pub use word_collector::WordCollector;

pub trait TokenCollector<'source> {
    fn try_collect(&self, code_stream: &mut CodeStream<'source>) -> Option<TokenKind<'source>>;
}
