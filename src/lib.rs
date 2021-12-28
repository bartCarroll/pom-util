extern crate serde;
extern crate quick_xml;

use serde::Deserialize;
use quick_xml::de::{from_str, DeError};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    pub group_id: String,
    pub artifact_id: String,
    pub version: Option<String>,
    pub scope: Option<String>
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub id: String,
    pub name: String,
    pub url: String
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Parent {
    pub group_id: String,
    pub artifact_id: String,
    pub version: String
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Dependencies {
    pub dependency: Vec<Dependency>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DependencyManagement {
    pub dependency: Vec<Dependency>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Repositories {
    pub repository: Vec<Repository>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    pub repository: Vec<Repository>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub group_id: Option<String>,
    pub artifact_id: String,
    pub version: Option<String>,
    pub parent: Option<Parent>,
    pub dependencies: Option<Dependencies>,
    pub dependency_management: Option<DependencyManagement>,
    pub repositories: Option<Repositories>,
    pub properties: Option<Properties>
}

/* fn load_file(filename: String){
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
} */

pub fn parse_pom(pom_str: &str) -> Result<Project, DeError> {
    let project: Project = from_str(pom_str)?;
    Ok(project)
}
