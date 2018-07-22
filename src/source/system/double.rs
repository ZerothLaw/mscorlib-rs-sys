use winapi::ctypes::c_double;

//struct __declspec(uuid("0f4f147f-4369-3388-8e4b-71e20c96f9ad"))
STRUCT!{struct Double {
    m_value: c_double,
}}
//IComparable, IFormattable, IConvertible
//        , IComparable<Double>, IEquatable<Double>