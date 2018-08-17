// MIT License
// Copyright (c) 2018 Tyler Laing (ZerothLaw)

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
mod arraylist;
mod bitarray;
mod caseinsensitivecomparer;
mod caseinsensitivehashcodeprovider;
mod collectionbase;
mod comparer;
mod dictionarybase;
mod dictionaryentry;
mod hashtable;
mod icollection;
mod icomparer;
mod idictionary;
mod idictionaryenumerator;
mod ienumerable;
mod ienumerator;
mod iequalitycomparer;
mod ihashcodeprovider;
mod ilist;
mod queue;
mod readonlycollectionbase;
mod sortedlist;
mod stack;

//This is done in order to mirror the namespacing of these items in the reference source
pub use self::arraylist::*;
pub use self::bitarray::*;
pub use self::caseinsensitivecomparer::*;
pub use self::caseinsensitivehashcodeprovider::*;
pub use self::collectionbase::*;
pub use self::comparer::*;
pub use self::dictionarybase::*;
pub use self::dictionaryentry::*;
pub use self::hashtable::*;
pub use self::icollection::*;
pub use self::icomparer::*;
pub use self::idictionary::*;
pub use self::idictionaryenumerator::*;
pub use self::ienumerable::*;
pub use self::ienumerator::*;
pub use self::iequalitycomparer::*;
pub use self::ihashcodeprovider::*;
pub use self::ilist::*;
pub use self::queue::*;
pub use self::readonlycollectionbase::*;
pub use self::sortedlist::*;
pub use self::stack::*;
