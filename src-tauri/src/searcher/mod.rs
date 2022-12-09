//! Document indexing and query ranking library
//!
//! Provides a generic interface for create a document index and
//! use various search strategies
//!
//! Basic Usage
//!
//! ```
//! // define a function to tokenize our object
//! fn tokenizer(s: String) -> Vec<Token> {
//!   s.split_whitespace().collect::<Vec<_>>()
//! }
//!
//! // define a function to access the text field of our object
//! fn accessor(s: &String) -> String {
//!   s.clone()
//! }
//!
//! // construct an index
//! let mut index == Index::new();
//! index.add_document("Hello world".to_owned());
//! index.add_document("Goodbye world".to_owned());
//!
//! // query the index using a strategy
//! let results = index.query("world", BM25::from(1.5, .75));
//!
//! assert_eq!(results.len(), 2);
//! ```

#[derive(Clone, Debug)]
pub struct QueryResult {
    pub id: usize,
    pub score: f64,
}

pub type Tokenizer = fn(String) -> Vec<Token>;
pub type Accessor<T> = fn(&T) -> String;

pub type Token = String;

#[derive(Clone)]
pub struct Index<T>
where
    T: Clone,
{
    documents: Vec<T>,
    tokens: Vec<Vec<Token>>,
    tokenizer: Tokenizer,
    accessor: Accessor<T>,
    avg_doc_len: usize,
}

#[allow(dead_code)]
impl<T> Index<T>
where
    T: Clone,
{
    pub fn new(tokenizer: Tokenizer, accessor: Accessor<T>) -> Self {
        Index {
            documents: vec![],
            tokens: vec![],
            tokenizer,
            accessor,
            avg_doc_len: 0,
        }
    }

    pub fn from(documents: &Vec<T>, tokenizer: Tokenizer, accessor: Accessor<T>) -> Self
    where
        T: Clone,
    {
        let documents = documents.clone();
        let mut index = Index {
            documents,
            tokens: vec![],
            tokenizer,
            accessor,
            avg_doc_len: 0,
        };
        index.recalc_avg_document_len();
        index.recalc_tokens();
        index
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    fn recalc_tokens(&mut self) {
        self.tokens = self
            .documents
            .iter()
            .map(self.accessor)
            .map(self.tokenizer)
            .collect::<Vec<Vec<Token>>>();
    }

    fn recalc_avg_document_len(&mut self) {
        let tokens = self
            .documents
            .iter()
            .map(self.accessor)
            .map(self.tokenizer)
            .collect::<Vec<Vec<Token>>>();
        let total_len: usize = tokens.iter().map(|tokens| tokens.len()).sum();
        self.avg_doc_len = total_len / self.documents.len();
    }

    pub fn add_document(&mut self, doc: &T)
    where
        T: Clone,
    {
        self.documents.push(doc.clone());
        let new_tokens = (self.tokenizer)((self.accessor)(doc));
        self.tokens.push(new_tokens);
        self.recalc_avg_document_len();
    }

    pub fn query(&self, query: &str, scorer: impl Scorer<T>) -> Vec<QueryResult> {
        scorer.score(query, &self)
    }
}

pub trait Scorer<T>
where
    T: Clone,
{
    fn score(self, query: &str, index: &Index<T>) -> Vec<QueryResult>;
}

pub struct BM25 {
    k: f64,
    b: f64,
}

#[allow(dead_code)]
impl BM25 {
    pub fn new() -> Self {
        BM25 { k: 1.5, b: 0.75 }
    }

    pub fn from(k: f64, b: f64) -> Self {
        BM25 { k, b }
    }
}

impl<T> Scorer<T> for BM25
where
    T: Clone,
{
    fn score(self, query: &str, index: &Index<T>) -> Vec<QueryResult> {
        let keywords = (index.tokenizer)(query.to_owned());
        let mut scores = vec![QueryResult { id: 0, score: 0. }; index.len()];
        for keyword in keywords {
            let n_qi: f64 = index
                .tokens
                .iter()
                .filter(|doc| doc.iter().filter(|token| **token == keyword).count() > 0)
                .count() as f64;
            let idf = ((index.tokens.len() as f64 - n_qi + 0.5) / (n_qi + 0.5) + 1.0).ln();
            for (i, document) in index.tokens.iter().enumerate() {
                let tf = document.iter().filter(|token| **token == keyword).count() as f64;
                let dl = document.len() as f64;
                let partial_score = idf * (tf * (self.k + 1.))
                    / (tf + self.k * (1. - self.b + self.b * dl / index.avg_doc_len as f64));
                scores[i].id = i;
                scores[i].score += partial_score;
            }
        }
        let mut scores = scores
            .into_iter()
            .filter(|result| result.score > 0.)
            .collect::<Vec<QueryResult>>();
        scores.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        scores
    }
}
