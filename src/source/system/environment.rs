// environment.rs - MIT License
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

use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//enum __declspec(uuid("3b1774cd-34e0-3c00-aabd-168b38c62fd9"))
ENUM!{enum EnvironmentVariableTarget {
    EnvironmentVariableTarget_Process = 0,
    EnvironmentVariableTarget_User = 1,
    EnvironmentVariableTarget_Machine = 2,
}}

RIDL!{#[uuid(0x29dc56cf, 0xb981, 0x3432, 0x97, 0xc8, 0x36, 0x80, 0xab, 0x6d, 0x86, 0x2d)]
interface _Environment(_EnvironmentVtbl): IDispatch(IDispatchVtbl)  
{}}


//enum __declspec(uuid("2e05a70a-1bbe-31df-b2a8-b8fa0f130915"))
ENUM!{enum SpecialFolder
{
    SpecialFolder_ApplicationData = 26,
    SpecialFolder_CommonApplicationData = 35,
    SpecialFolder_LocalApplicationData = 28,
    SpecialFolder_Cookies = 33,
    SpecialFolder_Desktop = 0,
    SpecialFolder_Favorites = 6,
    SpecialFolder_History = 34,
    SpecialFolder_InternetCache = 32,
    SpecialFolder_Programs = 2,
    SpecialFolder_MyComputer = 17,
    SpecialFolder_MyMusic = 13,
    SpecialFolder_MyPictures = 39,
    SpecialFolder_MyVideos = 14,
    SpecialFolder_Recent = 8,
    SpecialFolder_SendTo = 9,
    SpecialFolder_StartMenu = 11,
    SpecialFolder_Startup = 7,
    SpecialFolder_System = 37,
    SpecialFolder_Templates = 21,
    SpecialFolder_DesktopDirectory = 16,
    SpecialFolder_Personal = 5,
    SpecialFolder_MyDocuments = 5,
    SpecialFolder_ProgramFiles = 38,
    SpecialFolder_CommonProgramFiles = 43,
    SpecialFolder_AdminTools = 48,
    SpecialFolder_CDBurning = 59,
    SpecialFolder_CommonAdminTools = 47,
    SpecialFolder_CommonDocuments = 46,
    SpecialFolder_CommonMusic = 53,
    SpecialFolder_CommonOemLinks = 58,
    SpecialFolder_CommonPictures = 54,
    SpecialFolder_CommonStartMenu = 22,
    SpecialFolder_CommonPrograms = 23,
    SpecialFolder_CommonStartup = 24,
    SpecialFolder_CommonDesktopDirectory = 25,
    SpecialFolder_CommonTemplates = 45,
    SpecialFolder_CommonVideos = 55,
    SpecialFolder_Fonts = 20,
    SpecialFolder_NetworkShortcuts = 19,
    SpecialFolder_PrinterShortcuts = 27,
    SpecialFolder_UserProfile = 40,
    SpecialFolder_CommonProgramFilesX86 = 44,
    SpecialFolder_ProgramFilesX86 = 42,
    SpecialFolder_Resources = 56,
    SpecialFolder_LocalizedResources = 57,
    SpecialFolder_SystemX86 = 41,
    SpecialFolder_Windows = 36,
}}