// datetimestyles.rs - MIT License
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

//enum __declspec(uuid("f62ff05f-99ce-30db-8344-2b2c26f5765c"))
ENUM!{enum DateTimeStyles
{
    DateTimeStyles_None = 0,
    DateTimeStyles_AllowLeadingWhite = 1,
    DateTimeStyles_AllowTrailingWhite = 2,
    DateTimeStyles_AllowInnerWhite = 4,
    DateTimeStyles_AllowWhiteSpaces = 7,
    DateTimeStyles_NoCurrentDateDefault = 8,
    DateTimeStyles_AdjustToUniversal = 16,
    DateTimeStyles_AssumeLocal = 32,
    DateTimeStyles_AssumeUniversal = 64,
    DateTimeStyles_RoundTripKind = 128,
}}