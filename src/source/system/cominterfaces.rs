//.Net Framework 4.7.2 Reference Source - mscorlib { system.cominterfaces.cs }

//RIDL!{#[uuid()]}
//"([\w\d]{8})-([\w\d]{4})-([\w\d]{4})-([\w\d]{2})([\w\d]{2})-([\w\d]{2})([\w\d]{2})([\w\d]{2})([\w\d]{2})([\w\d]{2})([\w\d]{2})"
//0x$1, 0x$2, 0x$3, 0x$4, 0x$5, 0x$6, 0x$7, 0x$8, 0x$9, 0x$10, 0x$11

use winapi::shared::guiddef::REFIID;
use winapi::shared::minwindef::{WORD,UINT};
use winapi::shared::winerror::HRESULT;

use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

use system::intptr::IntPtr;

RIDL!{#[uuid(0x03973551, 0x57A1, 0x3900, 0xA2, 0xB5, 0x90, 0x83, 0xE3, 0xFF, 0x29, 0x43)]
interface _Activator(_ActivatorVtbl): IUnknown(IUnknown){
    fn GetTypeInfoCount(
        pcTInfo: *mut UINT, 
    ) -> HRESULT, 
    fn GetTypeInfo(
        iTInfo: UINT, 
        lcid: UINT, 
        ppTInfo: IntPtr, 
    ) -> HRESULT, 
    fn GetIDsOfNames(
        riid: REFIID, 
        rgszNames: IntPtr, 
        cNames: UINT, 
        lcid: UINT, 
        rgDispId: IntPtr,
    ) -> HRESULT,
    fn Invoke(
        dispIdMember: UINT,
        riid: REFIID, 
        wFlags: WORD, 
        pDispParams: IntPtr,
        pVarResult: IntPtr, 
        pExcepInfo: IntPtr,
        puArgError: IntPtr,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x917B14D0, 0x2D9E, 0x38B8, 0x92, 0xA9, 0x38, 0x1A, 0xCF, 0x52, 0xF7, 0xC0)]
interface _Attribute(_AttributeVtbl): IUnknown(IUnknownVtbl){
    fn GetTypeInfoCount(
        pcTInfo: *mut UINT, 
    ) -> HRESULT, 
    fn GetTypeInfo(
        iTInfo: UINT, 
        lcid: UINT, 
        ppTInfo: IntPtr, 
    ) -> HRESULT, 
    fn GetIDsOfNames(
        riid: REFIID, 
        rgszNames: IntPtr, 
        cNames: UINT, 
        lcid: UINT, 
        rgDispId: IntPtr,
    ) -> HRESULT,
    fn Invoke(
        dispIdMember: UINT,
        riid: REFIID, 
        wFlags: WORD, 
        pDispParams: IntPtr,
        pVarResult: IntPtr, 
        pExcepInfo: IntPtr,
        puArgError: IntPtr,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xC281C7F1, 0x4AA9, 0x3517, 0x96, 0x1A, 0x46, 0x3C, 0xFE, 0xD5, 0x7E, 0x75)]
interface _Thread(_ThreadVtbl): IUnknown(IUnknownVtbl){
    fn GetTypeInfoCount(
        pcTInfo: *mut UINT, 
    ) -> HRESULT, 
    fn GetTypeInfo(
        iTInfo: UINT, 
        lcid: UINT, 
        ppTInfo: IntPtr, 
    ) -> HRESULT, 
    fn GetIDsOfNames(
        riid: REFIID, 
        rgszNames: IntPtr, 
        cNames: UINT, 
        lcid: UINT, 
        rgDispId: IntPtr,
    ) -> HRESULT,
    fn Invoke(
        dispIdMember: UINT,
        riid: REFIID, 
        wFlags: WORD, 
        pDispParams: IntPtr,
        pVarResult: IntPtr, 
        pExcepInfo: IntPtr,
        puArgError: IntPtr,
    ) -> HRESULT,
}}
