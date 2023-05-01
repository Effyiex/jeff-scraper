
use bevy::prelude::*;

use std::{
  fs::{
    File,
    self
  },
  path::Path
};

pub struct ScrapeObject {
  pub url: String,
  pub space_literal: String
}

#[derive(Resource)]
pub struct ScrapeResource {
  pub objects: Vec<ScrapeObject>
}

pub fn register_resource(mut commands: Commands) {
  commands.insert_resource(ScrapeResource {
    objects: load_scrape_config()
  });
}

pub fn load_scrape_config() -> Vec<ScrapeObject> {

  let mut scrape_objects: Vec<ScrapeObject> = vec![];

  let config_path: &Path = Path::new("./scrape.cfg");
  
  if config_path.exists() && config_path.is_file() {
    match fs::read_to_string(config_path) {
      Ok(config_content) => {
        for line in config_content.lines() {

          if !line.contains(',') {
            continue;
          }

          let seperator: usize = line.find(',').unwrap();
          let space_literal: String = line[0..seperator].trim().to_string();
          let url: String = line[(seperator + 1)..line.len()].trim().to_string();
          scrape_objects.push(ScrapeObject { 
            url, 
            space_literal
          });

        }
        println!("[{}::scraper::load_scrape_config] // Loaded Scrape-Config.", env!("CARGO_PKG_NAME"));
      },
      _ => {
        println!("[{}::scraper::load_scrape_config] // Couldn't load Scrape-Config.", env!("CARGO_PKG_NAME"));
      }
    }
  } else {
    File::create(config_path).unwrap();
    println!("[{}::scraper::load_scrape_config] // Created Scrape-Config.", env!("CARGO_PKG_NAME"));
  }

  scrape_objects

}

pub fn search_jeff_urls(search_term: String, scrape_objects: &Vec<ScrapeObject>) -> Vec<String> {

  let mut jeff_urls: Vec<String> = vec![];

  for scrape_object in scrape_objects {

    let search_url: String = scrape_object.url
      .clone().replace("<SEARCH>", &search_term.replace(" ", &scrape_object.space_literal));

    println!("[{}::scraper::search_jeff_urls] // Scrape-URL: {}", env!("CARGO_PKG_NAME"), search_url);

    let search_response = reqwest::blocking::get(search_url).unwrap().text().unwrap();

    let mut response_strings: Vec<String> = vec![];

    let mut in_string: bool = false;
    let mut latest_identifier: char = '\n';
    let mut latest_string: String = String::new();
    let mut last_char = '\n';
    
    for response_char in search_response.chars() {
      if 
        last_char != '\\'
        && (response_char == '\"' /*|| response_char == '\''*/) 
        && (latest_identifier == '\n' || latest_identifier == response_char)
      {
        in_string = !in_string;
        if in_string {
          latest_identifier = response_char;
        } else {
          latest_identifier = '\n';
          response_strings.push(latest_string);
          latest_string = String::new();
        }
      } else if in_string {
        latest_string.push(response_char);
      }
      last_char = response_char;
    }
    for response_string in response_strings {
      if response_string.ends_with(".gif") {
        jeff_urls.push(response_string);
      }
    }
  }

  jeff_urls

}