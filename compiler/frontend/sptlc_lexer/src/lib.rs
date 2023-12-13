use code_stream::CodeStream;
use comment_handler::CommentsHandler;
use error::{LexError, LexResult};
use sptlc_ast::Pos;
use token::{Token, TokenKind};
use token_collector::*;

mod code_stream;
mod comment_handler;
mod error;
#[cfg(test)]
mod tests;
mod token;
mod token_collector;

pub struct Lexer<'source> {
    code_stream: CodeStream<'source>,
    collectors: Vec<Box<dyn TokenCollector<'source>>>,
    ended: bool,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            code_stream: CodeStream::new(source),
            collectors: vec![
                Box::new(NumberCollector),
                Box::new(SpecialCollector),
                Box::new(WordCollector),
            ],
            ended: false,
        }
    }

    pub fn next_token(&mut self) -> LexResult<Token<'source>> {
        CommentsHandler::skip(&mut self.code_stream);

        let pos = self.code_stream.pos();

        if self.code_stream.is_eof() {
            let eof_token = Token {
                kind: TokenKind::EOF,
                pos,
            };

            self.ended = true;

            return Ok(eof_token);
        }

        for collector in self.collectors.iter_mut() {
            if let Some(kind) = collector.try_collect(&mut self.code_stream) {
                let new_token = Token { pos, kind };

                return Ok(new_token);
            }
        }

        Err(self.unexpected_char(pos))
    }

    fn unexpected_char(&mut self, pos: Pos) -> LexError {
        LexError {
            char: self.code_stream.next(),
            pos,
        }
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = Result<Token<'source>, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ended {
            return None;
        }

        Some(self.next_token())
    }
}
