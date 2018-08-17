//    Copyright 2018 Tyler Laing
// 
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
// 
//        http://www.apache.org/licenses/LICENSE-2.0
// 
//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.

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