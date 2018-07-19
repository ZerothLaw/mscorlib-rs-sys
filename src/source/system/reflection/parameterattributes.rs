//enum __declspec(uuid("688a6ff0-5727-32d2-8228-6e838a822616"))
ENUM!{enum ParameterAttributes {
    ParameterAttributes_None = 0,
    ParameterAttributes_In = 1,
    ParameterAttributes_Out = 2,
    ParameterAttributes_Lcid = 4,
    ParameterAttributes_Retval = 8,
    ParameterAttributes_Optional = 16,
    ParameterAttributes_ReservedMask = 61440,
    ParameterAttributes_HasDefault = 4096,
    ParameterAttributes_HasFieldMarshal = 8192,
    ParameterAttributes_Reserved3 = 16384,
    ParameterAttributes_Reserved4 = 32768,
}}