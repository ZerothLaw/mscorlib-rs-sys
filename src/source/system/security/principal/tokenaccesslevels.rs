//enum __declspec(uuid("10a8b906-2f7a-327c-87ab-1a95a9b5e23e"))
ENUM!{enum TokenAccessLevels
{
    TokenAccessLevels_AssignPrimary = 1,
    TokenAccessLevels_Duplicate = 2,
    TokenAccessLevels_Impersonate = 4,
    TokenAccessLevels_Query = 8,
    TokenAccessLevels_QuerySource = 16,
    TokenAccessLevels_AdjustPrivileges = 32,
    TokenAccessLevels_AdjustGroups = 64,
    TokenAccessLevels_AdjustDefault = 128,
    TokenAccessLevels_AdjustSessionId = 256,
    TokenAccessLevels_Read = 131080,
    TokenAccessLevels_Write = 131296,
    TokenAccessLevels_AllAccess = 983551,
    TokenAccessLevels_MaximumAllowed = 33554432,
}}