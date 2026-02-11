use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::branding::*;
use std::sync::Arc;
use tracing::instrument;

pub struct BrandingApi {
    client: Arc<HttpClient>,
    cache: Arc<CacheManager>,
}

impl BrandingApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    // Account Brands
    #[instrument(skip(self))]
    pub async fn list_account_brands(&self) -> Result<Vec<AccountBrand>> {
        // OneLogin API v2 returns direct array
        self.client.get("/api/2/branding/brands").await
    }

    #[instrument(skip(self))]
    pub async fn get_account_brand(&self, brand_id: i64) -> Result<AccountBrand> {
        // OneLogin API v2 returns direct object
        self.client.get(&format!("/api/2/branding/brands/{}", brand_id)).await
    }

    #[instrument(skip(self, request))]
    pub async fn create_account_brand(&self, request: CreateBrandRequest) -> Result<AccountBrand> {
        // OneLogin API v2 returns direct object
        self.client.post("/api/2/branding/brands", Some(&request)).await
    }

    #[instrument(skip(self, request))]
    pub async fn update_account_brand(
        &self,
        brand_id: i64,
        request: UpdateBrandRequest,
    ) -> Result<AccountBrand> {
        // OneLogin API v2 returns direct object
        self.client.put(&format!("/api/2/branding/brands/{}", brand_id), Some(&request)).await
    }

    #[instrument(skip(self))]
    pub async fn delete_account_brand(&self, brand_id: i64) -> Result<()> {
        self.client.delete(&format!("/api/2/branding/brands/{}", brand_id)).await
    }

    // Legacy methods for backward compatibility
    #[instrument(skip(self))]
    pub async fn get_branding_settings(&self) -> Result<BrandingSettings> {
        // Map to list_account_brands for backward compatibility
        let brands = self.list_account_brands().await?;
        Ok(BrandingSettings { brands })
    }

    #[instrument(skip(self, _request))]
    pub async fn update_branding_settings(
        &self,
        _request: UpdateBrandingRequest,
    ) -> Result<BrandingSettings> {
        // This is a legacy method - users should use create/update_account_brand instead
        // For now, return the list of brands
        let brands = self.list_account_brands().await?;
        Ok(BrandingSettings { brands })
    }

    // ==================== MESSAGE TEMPLATES ====================

    /// List all message templates for a brand
    #[instrument(skip(self))]
    pub async fn list_message_templates(&self, brand_id: i64) -> Result<Vec<MessageTemplate>> {
        self.client
            .get(&format!("/api/2/branding/brands/{}/templates", brand_id))
            .await
    }

    /// Get a specific message template by ID
    #[instrument(skip(self))]
    pub async fn get_message_template(&self, brand_id: i64, template_id: i64) -> Result<MessageTemplate> {
        self.client
            .get(&format!("/api/2/branding/brands/{}/templates/{}", brand_id, template_id))
            .await
    }

    /// Get a message template by type
    #[instrument(skip(self))]
    pub async fn get_template_by_type(&self, brand_id: i64, template_type: &str) -> Result<MessageTemplate> {
        self.client
            .get(&format!("/api/2/branding/brands/{}/templates/{}", brand_id, template_type))
            .await
    }

    /// Get a message template by type and locale
    #[instrument(skip(self))]
    pub async fn get_template_by_locale(
        &self,
        brand_id: i64,
        template_type: &str,
        locale: &str,
    ) -> Result<MessageTemplate> {
        self.client
            .get(&format!(
                "/api/2/branding/brands/{}/templates/{}/{}",
                brand_id, template_type, locale
            ))
            .await
    }

    /// Create a new message template
    #[instrument(skip(self, request))]
    pub async fn create_message_template(
        &self,
        brand_id: i64,
        request: CreateMessageTemplateRequest,
    ) -> Result<MessageTemplate> {
        self.client
            .post(&format!("/api/2/branding/brands/{}/templates", brand_id), Some(&request))
            .await
    }

    /// Update a message template by ID
    #[instrument(skip(self, request))]
    pub async fn update_message_template(
        &self,
        brand_id: i64,
        template_id: i64,
        request: UpdateMessageTemplateRequest,
    ) -> Result<MessageTemplate> {
        self.client
            .put(
                &format!("/api/2/branding/brands/{}/templates/{}", brand_id, template_id),
                Some(&request),
            )
            .await
    }

    /// Update a message template by type and locale
    #[instrument(skip(self, request))]
    pub async fn update_template_by_locale(
        &self,
        brand_id: i64,
        template_type: &str,
        locale: &str,
        request: UpdateMessageTemplateRequest,
    ) -> Result<MessageTemplate> {
        self.client
            .put(
                &format!(
                    "/api/2/branding/brands/{}/templates/{}/{}",
                    brand_id, template_type, locale
                ),
                Some(&request),
            )
            .await
    }

    /// Delete a message template
    #[instrument(skip(self))]
    pub async fn delete_message_template(&self, brand_id: i64, template_id: i64) -> Result<()> {
        self.client
            .delete(&format!("/api/2/branding/brands/{}/templates/{}", brand_id, template_id))
            .await
    }
}
