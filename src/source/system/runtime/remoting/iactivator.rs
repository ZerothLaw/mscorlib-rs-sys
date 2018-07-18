use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::BSTR;
use winapi::um::oaidl::SAFEARRAY;

use source::system::collections::ilist::IList;
use source::system::reflection::cominterfaces::_Type;

RIDL!{#[uuid(0xc02bbb79, 0x5aa8, 0x390d, 0x92, 0x7f, 0x71, 0x7b, 0x7b, 0xff, 0x06, 0xa1)]
interface IActivator(IActivatorVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_NextActivator(
		pRetVal: *mut *mut  IActivator ,
	) -> HRESULT,
    fn putref_Activator(
        pRetVal: *mut IActivator,
    ) -> HRESULT,
    fn Activate(
		msg: *mut  IConstructionCallMessage ,
		pRetVal: *mut *mut  IConstructionReturnMessage ,
	) -> HRESULT,
    fn get_level(
        pRetVal: ActivatorLevel ,
    ) -> HRESULT,
}}

//enum __declspec(uuid("b946ac61-dd6b-39f3-bbe1-e4c1540f16ea"))
ENUM!{enum ActivatorLevel
{
    ActivatorLevel_Construction = 4,
    ActivatorLevel_Context = 8,
    ActivatorLevel_AppDomain = 12,
    ActivatorLevel_Process = 16,
    ActivatorLevel_Machine = 20,
}}

RIDL!{#[uuid(0xfa28e3af, 0x7d09, 0x31d5, 0xbe, 0xeb, 0x7f, 0x26, 0x26, 0x49, 0x7c, 0xde)]
interface IConstructionCallMessage(IConstructionCallMessageVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_Activator(
		pRetVal: *mut *mut  IActivator ,
	) -> HRESULT,
    fn putref_Activator(
        pRetVal: *mut IActivator,
    ) -> HRESULT,
    fn get_CallSiteActivationAttributes(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn get_ActivationTypeName(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn get_ActivationType(
		pRetVal: *mut *mut  _Type ,
	) -> HRESULT,
    fn get_ContextProperties(
		pRetVal: *mut *mut  IList ,
	) -> HRESULT,
}} //IMethodCallMessage inheritance

RIDL!{#[uuid(0xca0ab564, 0xf5e9, 0x3a7f, 0xa8, 0x0b, 0xeb, 0x0a, 0xee, 0xfa, 0x44, 0xe9)]
interface IConstructionReturnMessage(IConstructionReturnMessageVtbl): IDispatch(IDispatchVtbl)  
{}}
