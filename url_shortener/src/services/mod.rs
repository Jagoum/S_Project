use crate::{
    errors::{AppError, Result}, models::{CreateUrl, Url}, repository::UrlRepository, utils::generate_short_url
};

#[derive(Debug, Clone)]
pub struct UrlService {
    repository: UrlRepository,
}

impl UrlService {
    pub fn new(repository: UrlRepository) -> Self {
        Self { repository }
    }
    
    pub async fn create_short_url(&self, create_url: CreateUrl) -> Result<Url> {
        // Validate URL format
        if !is_valid_url(&create_url.long_url) {
            return Err(AppError::InvalidUrl);
        }
        
        let short_url = generate_short_url(&create_url.long_url);
        
        self.repository.create(&create_url.long_url, &short_url).await
    }
    
    pub async fn get_long_url(&self, short_url: &str) -> Result<Url> {
        self.repository.find_by_short_url(short_url).await
    }
}

fn is_valid_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}