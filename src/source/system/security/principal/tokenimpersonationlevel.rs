//enum __declspec(uuid("3e82fb4a-7f30-35b7-b8b1-6d717b3b5db0"))
ENUM!{enum TokenImpersonationLevel
{
    TokenImpersonationLevel_None = 0,
    TokenImpersonationLevel_Anonymous = 1,
    TokenImpersonationLevel_Identification = 2,
    TokenImpersonationLevel_Impersonation = 3,
    TokenImpersonationLevel_Delegation = 4,
}}