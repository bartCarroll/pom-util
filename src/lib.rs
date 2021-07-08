extern crate serde;
extern crate quick_xml;

use serde::Deserialize;
use quick_xml::de::{from_str, DeError};

#[derive(Debug, Deserialize, PartialEq)]
pub struct Dependency {
    pub groupId: String, 
    pub artifactId: String, 
    pub version: String
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Parent {
    pub groupId: String, 
    pub artifactId: String, 
    pub version: String
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Project {
    pub groupId: String,
    pub artifactId: String,
    pub version: String,
    pub parent: Option<Parent>,
    pub dependencies: Option<Vec<Dependency>>,
}

/* fn load_file(filename: String){
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
} */

pub fn parse_pom(pom_str: &str) -> Result<Project, DeError> {
    let project: Project = from_str(pom_str)?;
    println!("{}:{}", project.groupId, project.artifactId);
    Ok(project)
}
