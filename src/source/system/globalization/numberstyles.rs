// numberstyles.rs - MIT License
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

//enum __declspec(uuid("00d1aca9-41f2-3340-816e-330175414a56"))
ENUM!{enum NumberStyles
{
    NumberStyles_None = 0,
    NumberStyles_AllowLeadingWhite = 1,
    NumberStyles_AllowTrailingWhite = 2,
    NumberStyles_AllowLeadingSign = 4,
    NumberStyles_AllowTrailingSign = 8,
    NumberStyles_AllowParentheses = 16,
    NumberStyles_AllowDecimalPoint = 32,
    NumberStyles_AllowThousands = 64,
    NumberStyles_AllowExponent = 128,
    NumberStyles_AllowCurrencySymbol = 256,
    NumberStyles_AllowHexSpecifier = 512,
    NumberStyles_Integer = 7,
    NumberStyles_HexNumber = 515,
    NumberStyles_Number = 111,
    NumberStyles_Float = 167,
    NumberStyles_Currency = 383,
    NumberStyles_Any = 511,
}}