pub type NoOpResult = Result<(), Box<dyn std::error::Error>>;

pub type AdventResult<T> = Result<T, Box<dyn std::error::Error>>;
