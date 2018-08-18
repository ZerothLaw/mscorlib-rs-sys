// HashAlgorithmName.rs - MIT License
//  MIT License
//  Copyright (c) 2018 Tyler Laing (ZerothLaw)
// 
//  Permission is hereby granted, free of charge, to any person obtaining a copy
//  of this software and associated documentation files (the "Software"), to deal
//  in the Software without restriction, including without limitation the rights
//  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//  copies of the Software, and to permit persons to whom the Software is
//  furnished to do so, subject to the following conditions:
// 
//  The above copyright notice and this permission notice shall be included in all
//  copies or substantial portions of the Software.
// 
//  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//  SOFTWARE.

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