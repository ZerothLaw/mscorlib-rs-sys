use system::intptr::IntPtr;
//__declspec(uuid("06ad02b5-c5a4-3eec-b7ba-b0af7860d36a"))
STRUCT!{struct TypedReference {
    val: IntPtr,
    Type: IntPtr,
}}