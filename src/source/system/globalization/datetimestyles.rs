//enum __declspec(uuid("f62ff05f-99ce-30db-8344-2b2c26f5765c"))
ENUM!{enum DateTimeStyles
{
    DateTimeStyles_None = 0,
    DateTimeStyles_AllowLeadingWhite = 1,
    DateTimeStyles_AllowTrailingWhite = 2,
    DateTimeStyles_AllowInnerWhite = 4,
    DateTimeStyles_AllowWhiteSpaces = 7,
    DateTimeStyles_NoCurrentDateDefault = 8,
    DateTimeStyles_AdjustToUniversal = 16,
    DateTimeStyles_AssumeLocal = 32,
    DateTimeStyles_AssumeUniversal = 64,
    DateTimeStyles_RoundTripKind = 128,
}}