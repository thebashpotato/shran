use crate::config::ShranDefault;
use chrono::{DateTime, Utc};
use curl::easy::Easy;
use octocrab::models::repos::{Release, Tag};
use octocrab::{Octocrab, Page};
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

/// Reprents all necessary information about a github repositories
/// release information, most of this information is taken from
/// the similar but much larger octocrab Release struct
#[derive(Debug)]
pub struct GitRelease {
    pub author: String,
    pub tag_name: String,
    pub release_branch: String,
    pub published_at: Option<DateTime<Utc>>,
}

impl fmt::Display for GitRelease {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self.published_at {
            Some(time) => {
                write!(
                    f,
                    "Author: {}\nTag: {}\nRelease Branch: {}\n Published: {}",
                    self.author, self.tag_name, self.release_branch, time,
                )
            }
            None => {
                write!(
                    f,
                    "Author: {}\nTag: {}\nRelease Branch: {}\n Published: Unknown",
                    self.author, self.tag_name, self.release_branch,
                )
            }
        }
    }
}

/// A wrapper around around curl and Octocrab, GithubClient exposes
/// only the necessary functionality to search, verify and download
/// releases of specified Proof-of-Work Nodes, most notably bitcoin.
///
/// # Example
pub struct GithubClient {
    octocrab: Octocrab,
    easy: Easy,
}

impl GithubClient {
    pub fn new(token: String) -> Result<Self, Box<dyn std::error::Error>> {
        let octocrab = Octocrab::builder().personal_token(token).build()?;
        let easy: Easy = Easy::new();

        Ok(Self { octocrab, easy })
    }

    fn download_release(mut self, url: &String, file_name: String) -> std::io::Result<()> {
        let mut dst: Vec<u8> = Vec::new();
        self.easy.url(url)?;
        self.easy.follow_location(true)?;

        {
            let mut transfer = self.easy.transfer();
            transfer.write_function(|data| {
                dst.extend_from_slice(data);
                Ok(data.len())
            })?;
            transfer.perform()?;
        }
        {
            // TODO: Build a file system manager struct
            let mut file = File::create(file_name)?;
            file.write_all(dst.as_slice())?;
        }
        Ok(())
    }

    fn release_helper(self, release: Release) -> Result<GitRelease, Box<dyn Error>> {
        let url = format!(
            "{}/{}{}",
            ShranDefault::BITCOIN_BASE_URL,
            release.tag_name,
            ShranDefault::FILE_EXTENSION
        );

        // TODO: Replace the '.' in release.tag_name with '-'
        let file_name = format!("{}{}", release.tag_name, ShranDefault::FILE_EXTENSION);
        self.download_release(&url, file_name)?;

        Ok(GitRelease {
            author: release.author.login,
            tag_name: release.tag_name,
            release_branch: release.target_commitish,
            published_at: release.published_at,
        })
    }

    /// Download the latest release from github
    ///
    /// # Example
    ///
    /// ```no_run
    /// let gclient = GithubClient::new(token)?;
    /// let release: GitRelease = gclient.get_latest_release().await?;
    /// ```
    pub async fn get_latest_release(self) -> Result<GitRelease, Box<dyn Error>> {
        let release: Release = self
            .octocrab
            .repos("bitcoin", "bitcoin")
            .releases()
            .get_latest()
            .await?;

        self.release_helper(release)
    }

    /// Download a release specified by a tag
    ///
    /// # Example
    ///
    /// ```no_run
    /// let gclient = GithubClient::new(token)?;
    /// let tag = String::from("v23.0");
    /// let release: GitRelease = gclient.get_tagged_release(&tag).await?;
    /// ```
    pub async fn get_tagged_release(self, tag: &String) -> Result<GitRelease, Box<dyn Error>> {
        let release: Release = self
            .octocrab
            .repos("bitcoin", "bitcoin")
            .releases()
            .get_by_tag(tag)
            .await?;

        self.release_helper(release)
    }

    /// Fetches all available tags (releases) from bitcoins repository
    ///
    /// # Example
    /// ```no_run
    /// let gclient = GithubClient::new(token)?;
    /// let tags: Vec<String> = gclient.get_all_tags().await?;
    /// for tag in tags {
    ///    println!("{}", tag);
    /// }
    /// ```
    pub async fn get_all_tags(self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut current_page: Page<Tag> = self
            .octocrab
            .repos("bitcoin", "bitcoin")
            .list_tags()
            .send()
            .await?;

        // A Page<T> is basically a linked list, so we will iterate through it,
        // with the facillities that the ocotocrab library gives us, to ensure
        // we get the complete history of every release bitcoin has ever made.
        let mut page_of_tags: Vec<Tag> = current_page.take_items();
        let mut tags: Vec<String> = Vec::new();
        while let Ok(Some(mut new_page)) = self.octocrab.get_page(&current_page.next).await {
            page_of_tags.extend(new_page.take_items());

            for tag in page_of_tags.drain(..) {
                tags.push(tag.name);
            }
            current_page = new_page;
        }
        Ok(tags)
    }
}
