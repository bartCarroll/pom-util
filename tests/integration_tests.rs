use pomutil::parse_pom;
use rstest::rstest;

#[rstest]
fn parse_invalid_strings(
    #[values("", "    ", "<>", "jfkds;a", "<project><artifactId>blah</artifactId>")]
    pom_str: &str) {
    let result = parse_pom(pom_str);
    assert!(result.is_err());
}

#[rstest]
fn parse_valid_xml(
    #[values("<html></html>", "<xml></html>", "<project></project>")]
    pom_str: &str) {
    let result = parse_pom(pom_str);
    assert!(result.is_ok());
    let pom = result.unwrap();
    println!("POMSTUFF: {:?}", pom);
}

#[test]
fn parse_minimal_pom() {
    let pom_str = "<project xmlns=\"http://maven.apache.org/POM/4.0.0\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"
    xsi:schemaLocation=\"http://maven.apache.org/POM/4.0.0 http://maven.apache.org/maven-v4_0_0.xsd\">
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.blah</groupId>
    <artifactId>my-artifact</artifactId>
    <packaging>war</packaging>
    <version>1.0-SNAPSHOT</version>
    </project>";

    let result = parse_pom(pom_str);
    assert!(result.is_ok());
    let pom = result.unwrap();
    println!("POM: {:?}", pom);
    assert_eq!(pom.version, "1.0-SNAPSHOT");
    assert_eq!(pom.group_id, "com.blah");
    assert_eq!(pom.artifact_id, "my-artifact");
}

#[test]
fn parse_with_parent(){
    let pom_str = "<project>
    <parent>
      <groupId>com.my.corp</groupId>
      <artifactId>my-parent</artifactId>
      <version>1.2.0-SNAPSHOT</version>
    </parent>
    <artifactId>my-artifact</artifactId>
    <packaging>war</packaging>
    </project>";

    let result = parse_pom(pom_str);
    assert!(result.is_ok());
    let pom = result.unwrap();
    println!("POM: {:?}", pom);
    assert_eq!(pom.version, "1.0-SNAPSHOT");
    assert_eq!(pom.group_id, "com.blah");
    assert_eq!(pom.artifact_id, "my-artifact");
}
