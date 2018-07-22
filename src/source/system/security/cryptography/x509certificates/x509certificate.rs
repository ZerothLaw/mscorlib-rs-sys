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