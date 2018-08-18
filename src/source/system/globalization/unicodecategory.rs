// unicodecategory.rs - MIT License
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

//enum __declspec(uuid("299e2a7d-6551-3ed1-b4a0-a51cb56eefe7"))
ENUM!{enum UnicodeCategory
{
    UnicodeCategory_UppercaseLetter = 0,
    UnicodeCategory_LowercaseLetter = 1,
    UnicodeCategory_TitlecaseLetter = 2,
    UnicodeCategory_ModifierLetter = 3,
    UnicodeCategory_OtherLetter = 4,
    UnicodeCategory_NonSpacingMark = 5,
    UnicodeCategory_SpacingCombiningMark = 6,
    UnicodeCategory_EnclosingMark = 7,
    UnicodeCategory_DecimalDigitNumber = 8,
    UnicodeCategory_LetterNumber = 9,
    UnicodeCategory_OtherNumber = 10,
    UnicodeCategory_SpaceSeparator = 11,
    UnicodeCategory_LineSeparator = 12,
    UnicodeCategory_ParagraphSeparator = 13,
    UnicodeCategory_Control = 14,
    UnicodeCategory_Format = 15,
    UnicodeCategory_Surrogate = 16,
    UnicodeCategory_PrivateUse = 17,
    UnicodeCategory_ConnectorPunctuation = 18,
    UnicodeCategory_DashPunctuation = 19,
    UnicodeCategory_OpenPunctuation = 20,
    UnicodeCategory_ClosePunctuation = 21,
    UnicodeCategory_InitialQuotePunctuation = 22,
    UnicodeCategory_FinalQuotePunctuation = 23,
    UnicodeCategory_OtherPunctuation = 24,
    UnicodeCategory_MathSymbol = 25,
    UnicodeCategory_CurrencySymbol = 26,
    UnicodeCategory_ModifierSymbol = 27,
    UnicodeCategory_OtherSymbol = 28,
    UnicodeCategory_OtherNotAssigned = 29,
}}