
use winapi::um::oaidl::SAFEARRAY;

use system::configuration::assemblies::assemblyhashalgorithm::AssemblyHashAlgorithm;
//struct __declspec(uuid("42a66664-072f-3a67-a189-7d440709a77e"))
STRUCT!{struct AssemblyHash
{
    _Algorithm: AssemblyHashAlgorithm,
    _value: *mut SAFEARRAY,
}}
//deprecated
//implements ICloneable