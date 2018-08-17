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

//enum __declspec(uuid("d25ed092-a7a8-3bbe-820c-42f5a4604768"))
ENUM!{enum StackBehaviour
{
    StackBehaviour_Pop0 = 0,
    StackBehaviour_Pop1 = 1,
    StackBehaviour_Pop1_pop1 = 2,
    StackBehaviour_Popi = 3,
    StackBehaviour_Popi_pop1 = 4,
    StackBehaviour_Popi_popi = 5,
    StackBehaviour_Popi_popi8 = 6,
    StackBehaviour_Popi_popi_popi = 7,
    StackBehaviour_Popi_popr4 = 8,
    StackBehaviour_Popi_popr8 = 9,
    StackBehaviour_Popref = 10,
    StackBehaviour_Popref_pop1 = 11,
    StackBehaviour_Popref_popi = 12,
    StackBehaviour_Popref_popi_popi = 13,
    StackBehaviour_Popref_popi_popi8 = 14,
    StackBehaviour_Popref_popi_popr4 = 15,
    StackBehaviour_Popref_popi_popr8 = 16,
    StackBehaviour_Popref_popi_popref = 17,
    StackBehaviour_Push0 = 18,
    StackBehaviour_Push1 = 19,
    StackBehaviour_Push1_push1 = 20,
    StackBehaviour_Pushi = 21,
    StackBehaviour_Pushi8 = 22,
    StackBehaviour_Pushr4 = 23,
    StackBehaviour_Pushr8 = 24,
    StackBehaviour_Pushref = 25,
    StackBehaviour_Varpop = 26,
    StackBehaviour_Varpush = 27,
    StackBehaviour_Popref_popi_pop1 = 28,
}}