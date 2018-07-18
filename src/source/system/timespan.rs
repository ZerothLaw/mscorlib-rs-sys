use winapi::ctypes::c_long;
//struct __declspec(uuid("94942670-4acf-3572-92d1-0916cd777e00"))
STRUCT!{struct TimeSpan {
    _ticks: c_long,
}}
//IComparable, IComparable<TimeSpan>, IEquatable<TimeSpan>, IFormattable