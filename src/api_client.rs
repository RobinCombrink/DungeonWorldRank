use std::str::FromStr;

use anyhow::Result;
use log::debug;
use reqwest::{
    Url, {Client, ClientBuilder},
};
use tokio::task::JoinSet;

use crate::{
    character::{CharacterResponse, Region},
    from_response::FromResponse,
    mythic_plus_characters::MythicPlusCharactersResponse,
    mythic_plus_dungeons::{KeystoneRunResponse, MythicPlusGroupsResponse, RankedGroup},
};

pub struct ApiClient {
    client: Client,
    base_url: Url,
}

impl ApiClient {
    pub fn new() -> Self {
        ApiClient {
            client: ClientBuilder::new().build().unwrap(),
            base_url: Url::from_str("https://raider.io/")
                .unwrap()
                .join("api/")
                .unwrap(),
        }
    }

    pub async fn get_character(
        &self,
        region: Region,
        realm: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<CharacterResponse> {
        let url = self
            .base_url
            .join("v1/")?
            .join("characters/")?
            .join("profile")?;
        let response = self
            .client
            .get(url)
            .query(&[
                ("region", format!("{region}").as_ref()),
                ("realm", realm.into().as_ref()),
                ("name", name.into().as_ref()),
                ("fields", "mythic_plus_ranks"),
            ])
            .send()
            .await?;
        CharacterResponse::from_response(response).await
    }

    pub async fn get_total_characters(&self) -> Result<u32> {
        let first_characters_page = self.get_mythic_plus_characters(0).await?;

        let last_characters_page = self
            .get_mythic_plus_characters(first_characters_page.rankings.ui.last_page)
            .await?;

        let total_characters_from_pages = last_characters_page.rankings.ui.page_size
            * (last_characters_page.rankings.ui.last_page - 1);
        let total_characters = total_characters_from_pages
            + last_characters_page.rankings.ranked_characters.len() as u32;

        Ok(total_characters)
    }

    pub async fn get_total_characters_by_class(&self, class: &str) -> Result<u32> {
        let first_characters_page = self.get_mythic_plus_characters_by_class(class, 0).await?;

        let last_characters_page = self
            .get_mythic_plus_characters_by_class(class, first_characters_page.rankings.ui.last_page)
            .await?;

        let total_characters_from_pages = last_characters_page.rankings.ui.page_size
            * (last_characters_page.rankings.ui.last_page - 1);
        let total_characters = total_characters_from_pages
            + last_characters_page.rankings.ranked_characters.len() as u32;

        Ok(total_characters)
    }
    pub async fn get_mythic_plus_characters(
        &self,
        page: u32,
    ) -> Result<MythicPlusCharactersResponse> {
        let url = self.base_url.join("mythic-plus/rankings/characters")?;

        let response = self
            .client
            .get(url)
            .query(&[
                ("region", "world"),
                ("season", "season-tww-2"),
                ("class", "all"),
                ("role", "all"),
                ("page", &format!("{:?}", page)),
            ])
            .send()
            .await?;

        let url = response.url();
        debug!("{}", url);

        MythicPlusCharactersResponse::from_response(response).await
    }

    pub async fn get_mythic_plus_characters_by_class(
        &self,
        class: &str,
        page: u32,
    ) -> Result<MythicPlusCharactersResponse> {
        let url = self.base_url.join("mythic-plus/rankings/characters")?;

        let response = self
            .client
            .get(url)
            .query(&[
                ("region", "world"),
                ("season", "season-tww-2"),
                ("class", &format!("{}", class)),
                ("role", "all"),
                ("page", &format!("{:?}", page)),
            ])
            .send()
            .await?;

        let url = response.url();
        debug!("{}", url);

        MythicPlusCharactersResponse::from_response(response).await
    }

    pub async fn get_historical_dungeon_rank(&self, run_id: String) -> Result<usize> {
        let dungeon_run = self.get_dungeon_run(run_id).await?;

        let dungeon_slug = dungeon_run.keystone_run.dungeon.slug;

        let all_dungeon_runs = self.get_all_runs_by_dungeon(dungeon_slug).await?;

        let mut time_filtered_runs = all_dungeon_runs
            .into_iter()
            .filter(|run| {
                run.run
                    .completed_at
                    .ge(&dungeon_run.keystone_run.completed_at)
            })
            .collect::<Vec<_>>();
        time_filtered_runs.sort_by_key(|group| group.run.clear_time_ms);

        let time_filtered_runs = time_filtered_runs;

        let historical_dungeon_rank = time_filtered_runs
            .iter()
            .position(|run| run.run.keystone_run_id == dungeon_run.keystone_run.keystone_run_id)
            .expect("Filtered out the run that we're looking for");
        Ok(historical_dungeon_rank)
    }

    pub async fn get_dungeon_run(&self, run_id: String) -> Result<KeystoneRunResponse> {
        let url = self
            .base_url
            .join("mythic-plus/")?
            .join("runs/")?
            .join("season-tww-2/")?
            .join(&run_id)?;

        let response = self
            .client
            .get(url)
            .header("accept", "application/json")
            .send()
            .await?;

        let url = response.url();
        debug!("{}", url);

        KeystoneRunResponse::from_response(response).await
    }

    pub async fn get_all_runs_by_dungeon(&self, dungeon: String) -> Result<Vec<RankedGroup>> {
        let first_runs_page = self.get_dungeon_runs(dungeon.clone(), 0).await?;

        let total_pages = first_runs_page.rankings.ui.last_page;

        let tasks = (0..total_pages)
            .collect::<Vec<_>>()
            .into_iter()
            .map(async |page| self.get_dungeon_runs(dungeon.clone(), page).await);

        let dungeon_runs = futures::future::join_all(tasks)
            .await
            .into_iter()
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flat_map(|response| response.rankings.ranked_groups)
            .collect();

        Ok(dungeon_runs)
    }

    pub async fn get_dungeon_runs(
        &self,
        dungeon: String,
        page: u32,
    ) -> Result<MythicPlusGroupsResponse> {
        let url = self
            .base_url
            .join("mythic-plus/")?
            .join("rankings/")?
            .join("runs")?;

        let response = self
            .client
            .get(url)
            .query(&[
                ("region", "world"),
                ("season", "season-tww-2"),
                ("dungeon", &format!("{}", dungeon)),
                ("period", "0"),
                ("limit", "50"),
                ("recent", "false"),
                ("page", &format!("{:?}", page)),
            ])
            .header("referrer", format!("https://raider.io/mythic-plus-rankings/season-tww-2/cinderbrew-meadery/world/leaderboards-strict/{}", page))
            .send().await?;

        let url = response.url();
        debug!("{}", url);

        MythicPlusGroupsResponse::from_response(response).await
    }
}
