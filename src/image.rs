use anyhow::{Context, Result};
use serde::Deserialize;

const PUBLIC_REGISTRY: &str = "registry.hub.docker.com/v2";
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
    pub(crate) image: String,
    client: reqwest::blocking::Client,
}

impl ImageService {
    pub(crate) fn new(image: &str) -> Self {
        Self {
            image: image.to_string(),
            client: reqwest::blocking::Client::new(),
        }
    }

    fn get_auth_token(&self) -> Result<AuthToken> {
        let Some((image, _)) = self.image.split_once(':') else {
            anyhow::bail!("there is no version");
        };
        let token: AuthToken = self
            .client
            .get(AUTH_TARGET)
            .query(&[
                ("service", AUTH_SERVICE),
                ("scope", &format!("repository:library/{}:pull", image)),
            ])
            .send()
            .context("sending auth request")?
            .json()
            .context("converting from json")?;

        Ok(token)
    }

    pub(crate) fn get_image_manifest(&self) -> Result<()> {
        let Some((image, version)) = self.image.split_once(':') else {
            anyhow::bail!("need an image with a version");
        };

        let auth_token = self
            .get_auth_token()
            .context("getting authentication token")?;
        let manifest_url = format!("https://{PUBLIC_REGISTRY}/library/{image}/manifests/{version}");

        //let manifest_url = "https://registry.hub.docker.com/v2/library/busybox/manifests/latest";
        println!("Manifest URL: {manifest_url}");

        let response = self
            .client
            .get(&manifest_url)
            .header(
                reqwest::header::ACCEPT,
                "application/vnd.docker.distribution.manifest.v2+json",
            )
            .bearer_auth(auth_token.token)
            .send()
            .context("sending manifest request")?;

        let manifest: ImageManifest = response.json().context("deserialzing manifest")?;
        println!("Manifest: {manifest:?}");
        Ok(())
    }
}
