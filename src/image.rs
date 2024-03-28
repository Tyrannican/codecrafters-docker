use std::path::Path;

use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use tar::Archive;

use serde::Deserialize;

const PUBLIC_REGISTRY: &str = "https://registry.hub.docker.com/v2";
const AUTH_SERVICE: &str = "registry.docker.io";
const AUTH_TARGET: &str = "https://auth.docker.io/token";

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub(crate) struct AuthToken {
    token: String,
    expires_in: u32,
    issued_at: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct ImageManifest {
    #[serde(rename = "schemaVersion")]
    pub(crate) schema_version: u8,

    #[serde(rename = "mediaType")]
    pub(crate) media_type: String,

    pub(crate) config: ImageConfig,

    pub(crate) layers: Vec<ImageConfig>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct ImageConfig {
    #[serde(rename = "mediaType")]
    pub(crate) media_type: String,

    pub(crate) digest: String,

    pub(crate) size: usize,
}

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct ImageService {
    image: String,
    version: String,
    client: reqwest::blocking::Client,
}

impl ImageService {
    pub(crate) fn new(image: &str) -> Self {
        let (image, version) = match image.split_once(':') {
            Some((img, version)) => (img.to_string(), version.to_string()),
            None => (image.to_string(), "latest".to_string()),
        };

        Self {
            image,
            version,
            client: reqwest::blocking::Client::new(),
        }
    }

    pub(crate) fn download_image(&self, path: impl AsRef<Path>) -> Result<()> {
        let auth = self
            .get_auth_token()
            .context("retrieving authentication token")?;
        let manifest = self
            .get_image_manifest(&auth)
            .context("downloading image manifest")?;
        self.extract_layers(manifest, &auth, path)
            .context("extracting image layers")?;

        Ok(())
    }

    fn get_auth_token(&self) -> Result<AuthToken> {
        let token: AuthToken = self
            .client
            .get(AUTH_TARGET)
            .query(&[
                ("service", AUTH_SERVICE),
                ("scope", &format!("repository:library/{}:pull", self.image)),
            ])
            .send()
            .context("sending auth request")?
            .json()
            .context("converting from json")?;

        Ok(token)
    }

    fn get_image_manifest(&self, auth_token: &AuthToken) -> Result<ImageManifest> {
        let manifest_url = format!(
            "{PUBLIC_REGISTRY}/library/{}/manifests/{}",
            self.image, self.version
        );

        let manifest: ImageManifest = self
            .client
            .get(&manifest_url)
            .header(
                reqwest::header::ACCEPT,
                "application/vnd.docker.distribution.manifest.v2+json",
            )
            .bearer_auth(&auth_token.token)
            .send()
            .context("sending manifest request")?
            .json()
            .context("deserializing manifest")?;

        Ok(manifest)
    }

    fn extract_layers(
        &self,
        manifest: ImageManifest,
        auth: &AuthToken,
        path: impl AsRef<Path>,
    ) -> Result<()> {
        for layer in manifest.layers {
            self.extract_layer(layer, &auth, &path)
                .context("extracting individual layer")?;
        }

        Ok(())
    }

    fn extract_layer(
        &self,
        layer: ImageConfig,
        auth: &AuthToken,
        path: impl AsRef<Path>,
    ) -> Result<()> {
        let layer_url = format!(
            "{PUBLIC_REGISTRY}/library/{}/blobs/{}",
            self.image, layer.digest
        );

        let layer_bytes = self
            .client
            .get(layer_url)
            .bearer_auth(&auth.token)
            .send()
            .context("retrieving image layer")?
            .bytes()?;

        let reader = Box::new(std::io::Cursor::new(layer_bytes));
        let tar = GzDecoder::new(reader);
        let mut archive = Archive::new(tar);
        archive
            .unpack(path)
            .context("attempting to decompress image layer")?;

        Ok(())
    }
}
