// stackbehavior.rs - MIT License
//  MIT License
//  Copyright (c) 2018 Tyler Laing (ZerothLaw)
// 
//  Permission is hereby granted, free of charge, to any person obtaining a copy
//  of this software and associated documentation files (the "Software"), to deal
//  in the Software without restriction, including without limitation the rights
//  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//  copies of the Software, and to permit persons to whom the Software is
//  furnished to do so, subject to the following conditions:
// 
//  The above copyright notice and this permission notice shall be included in all
//  copies or substantial portions of the Software.
// 
//  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//  SOFTWARE.

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