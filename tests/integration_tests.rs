use pomutil::parse_pom;

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
    assert_eq!(pom.groupId, "com.blah");
    assert_eq!(pom.artifactId, "my-artifact");
}
