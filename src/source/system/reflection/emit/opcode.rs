//    Copyright 2018 Tyler Laing
// 
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
// 
//        http://www.apache.org/licenses/LICENSE-2.0
// 
//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.

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