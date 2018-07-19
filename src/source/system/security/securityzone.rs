//enum __declspec(uuid("902a6b65-41bd-32f1-a233-075f009d459c"))
SIGNED_ENUM!{enum SecurityZone
{
    SecurityZone_MyComputer = 0,
    SecurityZone_Intranet = 1,
    SecurityZone_Trusted = 2,
    SecurityZone_Internet = 3,
    SecurityZone_Untrusted = 4,
    SecurityZone_NoZone = -1,
}}