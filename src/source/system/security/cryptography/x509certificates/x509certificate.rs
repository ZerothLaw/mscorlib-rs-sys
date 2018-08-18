// x509certificate.rs - MIT License
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

use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//enum __declspec(uuid("70446b90-f93b-3578-9b7b-95d05a12da60"))
ENUM!{enum X509ContentType
{
    X509ContentType_Unknown = 0,
    X509ContentType_Cert = 1,
    X509ContentType_SerializedCert = 2,
    X509ContentType_Pfx = 3,
    X509ContentType_Pkcs12 = 3,
    X509ContentType_SerializedStore = 4,
    X509ContentType_Pkcs7 = 5,
    X509ContentType_Authenticode = 6,
}}


//enum __declspec(uuid("2530ee1e-6d70-3a79-a864-7cc0e2120da1"))
ENUM!{enum X509KeyStorageFlags
{
    X509KeyStorageFlags_DefaultKeySet = 0,
    X509KeyStorageFlags_UserKeySet = 1,
    X509KeyStorageFlags_MachineKeySet = 2,
    X509KeyStorageFlags_Exportable = 4,
    X509KeyStorageFlags_UserProtected = 8,
    X509KeyStorageFlags_PersistKeySet = 16,
}}

RIDL!{#[uuid(0x68fd6f14, 0xa7b2, 0x36c8, 0xa7, 0x24, 0xd0, 0x1f, 0x90, 0xd7, 0x34, 0x77)]
interface _X509Certificate(_X509CertificateVtbl): IDispatch(IDispatchVtbl)  
{}} //IDisposable, IDeserializationCallback, ISerializable