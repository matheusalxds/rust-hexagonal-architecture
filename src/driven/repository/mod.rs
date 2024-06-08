#[derive(Debug)]
pub enum RepoCreateError {
    InvalidData(String),
    Unknown(String),
}

#[derive(Debug)]
pub enum RepoSelectError {
    NotFound,
    Unknown(String),
}

#[derive(Debug)]
pub enum RepoFindAllError {
    Unknown(String),
}

#[derive(Debug)]
pub enum RepoUpdateError {
    InvalidData(String),
    NotFound,
    Unknown(String),
}

#[derive(Debug)]
pub enum RepoDeleteError {
    NotFound,
    InvalidData(String),
    Unknown(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindSandwich {
    pub id: Option<String>,
    pub name: String,
    pub ingredients: Vec<String>,
}

#[async_trait]
pub trait Repository<T>
where
    T: Entity,
{
    fn new(config: &PersistenceConfig) -> Result<Self, String>
    where
        Self: Sized;
    async fn create(&self, sandwich: T) -> Result<T, RepoCreateError>;
    async fn find_one(&self, sandwich: FindSandwich) -> Result<T, RepoSelectError>;
    async fn find_all(&self, sandwich: FindSandWich) -> Result<Vec<T>, RepoFindAllError>;
    async fn update(&self, sandwich: T) -> Result<T, RepoUpdateError>;
    async fn delete(&self, id: &str) -> Result<(), RepoDeleteError>;
}
