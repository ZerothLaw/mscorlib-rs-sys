use winapi::ctypes::c_long;
use winapi::shared::minwindef::UCHAR;
use winapi::shared::wtypes::BSTR;

use system::reflection::emit::flowcontrol::FlowControl;
use system::reflection::emit::stackbehavior::StackBehaviour;
use system::reflection::emit::operandtype::OperandType;
use system::reflection::emit::opcodetype::OpCodeType;

//struct __declspec(uuid("a7ed05c6-fecf-3c35-ba3b-84163ac1d5e5"))
STRUCT!{struct OpCode
{
    m_stringname: BSTR,
    m_pop: StackBehaviour,
    m_push: StackBehaviour,
    m_operand: OperandType,
    m_type: OpCodeType,
    m_size: c_long,
    m_s1: UCHAR,
    m_s2: UCHAR,
    m_ctrl: FlowControl,
    m_endsUncondJmpBlk: c_long,
    m_stackChange: c_long,
}}