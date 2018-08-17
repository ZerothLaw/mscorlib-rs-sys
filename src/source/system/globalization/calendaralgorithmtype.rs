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

//enum __declspec(uuid("f680a48a-2d6c-33f1-aff7-6273b785b035"))
ENUM!{enum CalendarAlgorithmType
{
    CalendarAlgorithmType_Unknown = 0,
    CalendarAlgorithmType_SolarCalendar = 1,
    CalendarAlgorithmType_LunarCalendar = 2,
    CalendarAlgorithmType_LunisolarCalendar = 3,
}}