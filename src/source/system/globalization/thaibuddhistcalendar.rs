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

use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
RIDL!{#[uuid(0xc70c8ae8, 0x925b, 0x37ce, 0x89, 0x44, 0x34, 0xf1, 0x5f, 0xf9, 0x43, 0x07)]
interface _ThaiBuddhistCalendar(_ThaiBuddhistCalendarVtbl): IDispatch(IDispatchVtbl)  
{}}