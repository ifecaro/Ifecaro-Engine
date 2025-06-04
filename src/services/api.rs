use serde::{Deserialize, Serialize};
use crate::contexts::paragraph_context::{ParagraphData, Paragraph};
use crate::constants::config::{BASE_API_URL, PARAGRAPHS, CHAPTERS};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChapterData {
    pub items: Vec<Chapter>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Chapter {
    pub id: String,
    pub title: String,
    pub order: i32,
}

// API error types
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ApiError {
    NetworkError(String),
    ParseError(String),
    NotFound,
    ServerError(u16),
}

// API result type
#[allow(dead_code)]
pub type ApiResult<T> = Result<T, ApiError>;

// API client trait for easy mocking during tests
#[allow(dead_code)]
#[async_trait::async_trait(?Send)]
pub trait ApiClient {
    async fn get_paragraphs(&self) -> ApiResult<ParagraphData>;
    async fn get_chapters(&self) -> ApiResult<ChapterData>;
    async fn get_paragraph_by_id(&self, id: &str) -> ApiResult<Paragraph>;
    async fn update_paragraph(&self, paragraph: &Paragraph) -> ApiResult<()>;
}

// Real HTTP client implementation
#[allow(dead_code)]
pub struct HttpApiClient {
    client: reqwest::Client,
}

#[allow(dead_code)]
impl HttpApiClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait::async_trait(?Send)]
impl ApiClient for HttpApiClient {
    async fn get_paragraphs(&self) -> ApiResult<ParagraphData> {
        let url = format!("{}{}", BASE_API_URL, PARAGRAPHS);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;
            
        if !response.status().is_success() {
            return Err(ApiError::ServerError(response.status().as_u16()));
        }
        
        response
            .json::<ParagraphData>()
            .await
            .map_err(|e| ApiError::ParseError(e.to_string()))
    }
    
    async fn get_chapters(&self) -> ApiResult<ChapterData> {
        let url = format!("{}{}", BASE_API_URL, CHAPTERS);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;
            
        if !response.status().is_success() {
            return Err(ApiError::ServerError(response.status().as_u16()));
        }
        
        response
            .json::<ChapterData>()
            .await
            .map_err(|e| ApiError::ParseError(e.to_string()))
    }
    
    async fn get_paragraph_by_id(&self, id: &str) -> ApiResult<Paragraph> {
        let paragraphs = self.get_paragraphs().await?;
        
        paragraphs.items
            .into_iter()
            .find(|p| p.id == id)
            .ok_or(ApiError::NotFound)
    }
    
    async fn update_paragraph(&self, paragraph: &Paragraph) -> ApiResult<()> {
        let url = format!("{}{}/{}", BASE_API_URL, PARAGRAPHS, paragraph.id);
        
        let response = self.client
            .put(&url)
            .json(paragraph)
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;
            
        if !response.status().is_success() {
            return Err(ApiError::ServerError(response.status().as_u16()));
        }
        
        Ok(())
    }
}

// Mock client for testing
#[allow(dead_code)]
pub struct MockApiClient {
    pub paragraphs: Vec<Paragraph>,
    pub chapters: Vec<Chapter>,
    pub should_fail: bool,
}

#[allow(dead_code)]
impl MockApiClient {
    pub fn new() -> Self {
        Self {
            paragraphs: Vec::new(),
            chapters: Vec::new(),
            should_fail: false,
        }
    }
    
    pub fn with_paragraphs(mut self, paragraphs: Vec<Paragraph>) -> Self {
        self.paragraphs = paragraphs;
        self
    }
    
    pub fn with_chapters(mut self, chapters: Vec<Chapter>) -> Self {
        self.chapters = chapters;
        self
    }
    
    pub fn with_failure(mut self) -> Self {
        self.should_fail = true;
        self
    }
}

#[cfg(test)]
#[async_trait::async_trait(?Send)]
impl ApiClient for MockApiClient {
    async fn get_paragraphs(&self) -> ApiResult<ParagraphData> {
        if self.should_fail {
            return Err(ApiError::NetworkError("Mock network error".to_string()));
        }
        
        Ok(ParagraphData {
            items: self.paragraphs.clone(),
        })
    }
    
    async fn get_chapters(&self) -> ApiResult<ChapterData> {
        if self.should_fail {
            return Err(ApiError::NetworkError("Mock network error".to_string()));
        }
        
        Ok(ChapterData {
            items: self.chapters.clone(),
        })
    }
    
    async fn get_paragraph_by_id(&self, id: &str) -> ApiResult<Paragraph> {
        if self.should_fail {
            return Err(ApiError::NetworkError("Mock network error".to_string()));
        }
        
        self.paragraphs
            .iter()
            .find(|p| p.id == id)
            .cloned()
            .ok_or(ApiError::NotFound)
    }
    
    async fn update_paragraph(&self, _paragraph: &Paragraph) -> ApiResult<()> {
        if self.should_fail {
            return Err(ApiError::NetworkError("Mock network error".to_string()));
        }
        Ok(())
    }
} 