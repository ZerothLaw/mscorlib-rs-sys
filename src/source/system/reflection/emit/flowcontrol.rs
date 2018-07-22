//enum __declspec(uuid("75a7861c-767e-3a5e-a57b-6ec136009654"))
ENUM!{enum FlowControl
{
    FlowControl_Branch = 0,
    FlowControl_Break = 1,
    FlowControl_Call = 2,
    FlowControl_Cond_Branch = 3,
    FlowControl_Meta = 4,
    FlowControl_Next = 5,
    FlowControl_Phi = 6,
    FlowControl_Return = 7,
    FlowControl_Throw = 8,
}}