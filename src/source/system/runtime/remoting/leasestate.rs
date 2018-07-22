//enum __declspec(uuid("a2c06560-e728-39d5-8230-7eb08001c79e"))
ENUM!{enum LeaseState
{
    LeaseState_Null = 0,
    LeaseState_Initial = 1,
    LeaseState_Active = 2,
    LeaseState_Renewing = 3,
    LeaseState_Expired = 4,
}}
