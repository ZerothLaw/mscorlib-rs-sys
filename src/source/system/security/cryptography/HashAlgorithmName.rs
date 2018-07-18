//No exported type, but reimplementing here for type safety


#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct HashAlgorithmName {
    name: String
}

impl HashAlgorithmName {
    fn name(&self) -> &str {
        &self.name
    }

    fn md5() -> HashAlgorithmName {
        HashAlgorithmName{
            name: String::from("MD5")
        }
    }

    fn sha1() -> HashAlgorithmName {
        HashAlgorithmName{
            name: String::from("SHA1")
        }
    }

    fn sha256() -> HashAlgorithmName {
        HashAlgorithmName{
            name: String::from("SHA256")
        }
    }

    fn sha384() -> HashAlgorithmName {
        HashAlgorithmName{
            name: String::from("SHA384")
        }
    }

    fn sha512() -> HashAlgorithmName {
        HashAlgorithmName{
            name: String::from("SHA512")
        }
    }
}