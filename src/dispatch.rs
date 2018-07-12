//std

//3rd party

use winapi::ctypes::c_long;

use winapi::shared::ntdef::{HRESULT};
use winapi::shared::wtypes::{VARIANT_BOOL};

use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::um::oaidl::{VARIANT};

//self
use enums::*;
use unknown::*;

RIDL!{#[uuid(0xdeb0e770, 0x91fd, 0x3cf6, 0x9a, 0x6c, 0xe6, 0xa3, 0x65, 0x6f, 0x39, 0x65)]
interface IComparable(IComparableVtbl): IDispatch(IDispatchVtbl){

    fn CompareTo(
        obj: VARIANT, 
        pRetVal: *mut c_long, 
    ) -> HRESULT,

}}

RIDL!{#[uuid(0x0cb251a7, 0x3ab3, 0x3b5c, 0xa0, 0xb8, 0x9d, 0xdf, 0x88, 0x82, 0x4b, 0x85)]
interface ICloneable(ICloneableVtbl): IDispatch(IDispatchVtbl){

    fn Clone( 
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,

}}

RIDL!{#[uuid(0x496b0abe, 0xcdee, 0x11d3, 0x88, 0xe8, 0x00, 0x90, 0x27, 0x54, 0xc4, 0x3a)]
interface IEnumerable(IEnumerableVtbl): IDispatch(IDispatchVtbl){

    fn GetEnumerator( 
        pRetVal: *mut *mut IEnumVARIANT, 
    ) -> HRESULT,

}}

RIDL!{#[uuid(0x7bcfa00f, 0xf764, 0x3113, 0x91, 0x40, 0x3b, 0xbd, 0x12, 0x7a, 0x96, 0xbb)]
interface IList(IListVtbl) : IDispatch(IDispatchVtbl){

    fn get_Item(
        index: c_long, 
        pRetVal: *mut VARIANT,
    ) -> HRESULT,

    fn putref_Item(
        index: c_long, 
        pRetVal: VARIANT, 
    ) -> HRESULT,

    fn Add(
        value: VARIANT, 
        pRetVal: *mut c_long,
    ) -> HRESULT,

    fn Contains(
        value: VARIANT, 
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,

    fn Clear() -> HRESULT,

    fn get_IsReadOnly(
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,

    fn get_IsFixedSize(
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,

    fn IndexOf(
        value: VARIANT, 
        pRetVal: *mut c_long, 
    ) -> HRESULT,

    fn Insert(
        index: c_long, 
        value: VARIANT, 
    ) -> HRESULT,

    fn Remove(
        value: VARIANT, 
    ) -> HRESULT,

    fn RemoveAt(
        index: c_long,
    ) -> HRESULT,

}}

RIDL!{#[uuid(0x496b0abf, 0xcdee, 0x11d3, 0x88, 0xe8, 0x00, 0x90, 0x27, 0x54, 0xc4, 0x3a)]
interface IEnumerator(IEnumeratorVtbl) : IDispatch(IDispatchVtbl){
    
    fn MoveNext(
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,

    fn get_Current(
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,

    fn Reset() -> HRESULT,

}}

RIDL!{#[uuid(0x496b0abf, 0xcdee, 0x11d3, 0x88, 0xe8, 0x00, 0x90, 0x27, 0x54, 0xc4, 0x3a)]
interface IDisposable(IDisposableVtbl) : IDispatch(IDispatchVtbl){

    fn Dispose() -> HRESULT,

}}

RIDL!{#[uuid(0xc20fd3eb, 0x7022, 0x3d14, 0x84, 0x77, 0x76, 0x0f, 0xab, 0x54, 0xe5, 0x0d)]
interface IComparer(IComparerVtbl) : IDispatch(IDispatchVtbl)
{
    fn Compare(
        x: VARIANT, 
        y: VARIANT, 
        pRetVal: *mut c_long,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xaab7c6ea, 0xcab0, 0x3adb, 0x82, 0xaa, 0xcf, 0x32, 0xe2, 0x9a, 0xf2, 0x3)]
interface IEqualityComparer(IEqualityComparerVtbl) : IDispatch(IDispatchVtbl){
    fn Equals(
        x: VARIANT, 
        y: VARIANT, 
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,

    fn GetHashCode(
        obj: VARIANT, 
        pRetVal: *mut c_long,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xab3f47e4, 0xc227, 0x3b05, 0xbf, 0x9f, 0x94, 0x64, 0x9b, 0xef, 0x98, 0x88)]
interface IDeserializationCallback(IDeserializationCallbackVtbl) : IDispatch(IDispatchVtbl)
{
    fn OnDeserialization(
        sender: VARIANT, 
    ) -> HRESULT,
}}

//line 3846 - ThreadState enum