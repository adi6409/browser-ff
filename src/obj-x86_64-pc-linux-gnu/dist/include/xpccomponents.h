/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/js/xpconnect/idl/xpccomponents.idl
 */

#ifndef __gen_xpccomponents_h__
#define __gen_xpccomponents_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#include "jspubtd.h"
class xpcIJSWeakReference; /* forward declaration */

class nsIClassInfo; /* forward declaration */

class nsICommandParams; /* forward declaration */

class nsIComponentManager; /* forward declaration */

class nsICycleCollectorListener; /* forward declaration */

class nsIDocumentEncoder; /* forward declaration */

class nsIEditorSpellCheck; /* forward declaration */

class nsIFile; /* forward declaration */

class nsILoadContext; /* forward declaration */

class nsIPersistentProperties; /* forward declaration */

class nsIURI; /* forward declaration */

class nsIPrincipal; /* forward declaration */

class nsIStackFrame; /* forward declaration */

namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIXPCComponents_Interfaces */
#define NS_IXPCCOMPONENTS_INTERFACES_IID_STR "b8c31bba-79db-4a1d-930d-4cdd68713f9e"

#define NS_IXPCCOMPONENTS_INTERFACES_IID \
  {0xb8c31bba, 0x79db, 0x4a1d, \
    { 0x93, 0x0d, 0x4c, 0xdd, 0x68, 0x71, 0x3f, 0x9e }}

class NS_NO_VTABLE nsIXPCComponents_Interfaces : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IXPCCOMPONENTS_INTERFACES_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIXPCComponents_Interfaces;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIXPCComponents_Interfaces, NS_IXPCCOMPONENTS_INTERFACES_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIXPCCOMPONENTS_INTERFACES \
  /* no methods! */

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIXPCCOMPONENTS_INTERFACES \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIXPCCOMPONENTS_INTERFACES(_to) \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIXPCCOMPONENTS_INTERFACES(_to) \
  /* no methods! */


/* starting interface:    nsIXPCComponents_Classes */
#define NS_IXPCCOMPONENTS_CLASSES_IID_STR "978ff520-d26c-11d2-9842-006008962422"

#define NS_IXPCCOMPONENTS_CLASSES_IID \
  {0x978ff520, 0xd26c, 0x11d2, \
    { 0x98, 0x42, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22 }}

class NS_NO_VTABLE nsIXPCComponents_Classes : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IXPCCOMPONENTS_CLASSES_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIXPCComponents_Classes;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIXPCComponents_Classes, NS_IXPCCOMPONENTS_CLASSES_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIXPCCOMPONENTS_CLASSES \
  /* no methods! */

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIXPCCOMPONENTS_CLASSES \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIXPCCOMPONENTS_CLASSES(_to) \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIXPCCOMPONENTS_CLASSES(_to) \
  /* no methods! */


/* starting interface:    nsIXPCComponents_Results */
#define NS_IXPCCOMPONENTS_RESULTS_IID_STR "2fc229a0-5860-11d3-9899-006008962422"

#define NS_IXPCCOMPONENTS_RESULTS_IID \
  {0x2fc229a0, 0x5860, 0x11d3, \
    { 0x98, 0x99, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22 }}

class NS_NO_VTABLE nsIXPCComponents_Results : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IXPCCOMPONENTS_RESULTS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIXPCComponents_Results;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIXPCComponents_Results, NS_IXPCCOMPONENTS_RESULTS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIXPCCOMPONENTS_RESULTS \
  /* no methods! */

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIXPCCOMPONENTS_RESULTS \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIXPCCOMPONENTS_RESULTS(_to) \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIXPCCOMPONENTS_RESULTS(_to) \
  /* no methods! */


/* starting interface:    nsIXPCComponents_ID */
#define NS_IXPCCOMPONENTS_ID_IID_STR "7994a6e0-e028-11d3-8f5d-0010a4e73d9a"

#define NS_IXPCCOMPONENTS_ID_IID \
  {0x7994a6e0, 0xe028, 0x11d3, \
    { 0x8f, 0x5d, 0x00, 0x10, 0xa4, 0xe7, 0x3d, 0x9a }}

class NS_NO_VTABLE nsIXPCComponents_ID : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IXPCCOMPONENTS_ID_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIXPCComponents_ID;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIXPCComponents_ID, NS_IXPCCOMPONENTS_ID_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIXPCCOMPONENTS_ID \
  /* no methods! */

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIXPCCOMPONENTS_ID \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIXPCCOMPONENTS_ID(_to) \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIXPCCOMPONENTS_ID(_to) \
  /* no methods! */


/* starting interface:    nsIXPCComponents_Exception */
#define NS_IXPCCOMPONENTS_EXCEPTION_IID_STR "5bf039c0-e028-11d3-8f5d-0010a4e73d9a"

#define NS_IXPCCOMPONENTS_EXCEPTION_IID \
  {0x5bf039c0, 0xe028, 0x11d3, \
    { 0x8f, 0x5d, 0x00, 0x10, 0xa4, 0xe7, 0x3d, 0x9a }}

class NS_NO_VTABLE nsIXPCComponents_Exception : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IXPCCOMPONENTS_EXCEPTION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIXPCComponents_Exception;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIXPCComponents_Exception, NS_IXPCCOMPONENTS_EXCEPTION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIXPCCOMPONENTS_EXCEPTION \
  /* no methods! */

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIXPCCOMPONENTS_EXCEPTION \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIXPCCOMPONENTS_EXCEPTION(_to) \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIXPCCOMPONENTS_EXCEPTION(_to) \
  /* no methods! */


/* starting interface:    nsIXPCComponents_Constructor */
#define NS_IXPCCOMPONENTS_CONSTRUCTOR_IID_STR "88655640-e028-11d3-8f5d-0010a4e73d9a"

#define NS_IXPCCOMPONENTS_CONSTRUCTOR_IID \
  {0x88655640, 0xe028, 0x11d3, \
    { 0x8f, 0x5d, 0x00, 0x10, 0xa4, 0xe7, 0x3d, 0x9a }}

class NS_NO_VTABLE nsIXPCComponents_Constructor : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IXPCCOMPONENTS_CONSTRUCTOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIXPCComponents_Constructor;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIXPCComponents_Constructor, NS_IXPCCOMPONENTS_CONSTRUCTOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIXPCCOMPONENTS_CONSTRUCTOR \
  /* no methods! */

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIXPCCOMPONENTS_CONSTRUCTOR \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIXPCCOMPONENTS_CONSTRUCTOR(_to) \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIXPCCOMPONENTS_CONSTRUCTOR(_to) \
  /* no methods! */


/* starting interface:    nsIXPCComponents_utils_Sandbox */
#define NS_IXPCCOMPONENTS_UTILS_SANDBOX_IID_STR "4f8ae0dc-d266-4a32-875b-6a9de71a8ce9"

#define NS_IXPCCOMPONENTS_UTILS_SANDBOX_IID \
  {0x4f8ae0dc, 0xd266, 0x4a32, \
    { 0x87, 0x5b, 0x6a, 0x9d, 0xe7, 0x1a, 0x8c, 0xe9 }}

class NS_NO_VTABLE nsIXPCComponents_utils_Sandbox : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IXPCCOMPONENTS_UTILS_SANDBOX_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIXPCComponents_utils_Sandbox;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIXPCComponents_utils_Sandbox, NS_IXPCCOMPONENTS_UTILS_SANDBOX_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIXPCCOMPONENTS_UTILS_SANDBOX \
  /* no methods! */

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIXPCCOMPONENTS_UTILS_SANDBOX \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIXPCCOMPONENTS_UTILS_SANDBOX(_to) \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIXPCCOMPONENTS_UTILS_SANDBOX(_to) \
  /* no methods! */


/* starting interface:    nsIScheduledGCCallback */
#define NS_ISCHEDULEDGCCALLBACK_IID_STR "71000535-b0fd-44d1-8ce0-909760e3953c"

#define NS_ISCHEDULEDGCCALLBACK_IID \
  {0x71000535, 0xb0fd, 0x44d1, \
    { 0x8c, 0xe0, 0x90, 0x97, 0x60, 0xe3, 0x95, 0x3c }}

class NS_NO_VTABLE nsIScheduledGCCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISCHEDULEDGCCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIScheduledGCCallback;

  /* void callback (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Callback(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIScheduledGCCallback, NS_ISCHEDULEDGCCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISCHEDULEDGCCALLBACK \
  NS_IMETHOD Callback(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISCHEDULEDGCCALLBACK \
  nsresult Callback(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISCHEDULEDGCCALLBACK(_to) \
  NS_IMETHOD Callback(void) override { return _to Callback(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISCHEDULEDGCCALLBACK(_to) \
  NS_IMETHOD Callback(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Callback(); } 


/* starting interface:    nsIXPCComponents_Utils */
#define NS_IXPCCOMPONENTS_UTILS_IID_STR "86003fe3-ee9a-4620-91dc-eef8b1e58815"

#define NS_IXPCCOMPONENTS_UTILS_IID \
  {0x86003fe3, 0xee9a, 0x4620, \
    { 0x91, 0xdc, 0xee, 0xf8, 0xb1, 0xe5, 0x88, 0x15 }}

class NS_NO_VTABLE nsIXPCComponents_Utils : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IXPCCOMPONENTS_UTILS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIXPCComponents_Utils;

  /* [implicit_jscontext] void reportError (in jsval error, [optional] in jsval stack); */
  NS_IMETHOD ReportError(JS::HandleValue error, JS::HandleValue stack, JSContext* cx) = 0;

  /* readonly attribute nsIXPCComponents_utils_Sandbox Sandbox; */
  NS_IMETHOD GetSandbox(nsIXPCComponents_utils_Sandbox **aSandbox) = 0;

  /* [implicit_jscontext] jsval createServicesCache (); */
  NS_IMETHOD CreateServicesCache(JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext,optional_argc] jsval evalInSandbox (in AString source, in jsval sandbox, [optional] in jsval version, [optional] in AUTF8String filename, [optional] in long lineNo, [optional] in bool enforceFilenameRestrictions); */
  NS_IMETHOD EvalInSandbox(const nsAString& source, JS::HandleValue sandbox, JS::HandleValue version, const nsACString& filename, int32_t lineNo, bool enforceFilenameRestrictions, JSContext* cx, uint8_t _argc, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] jsval getUAWidgetScope (in nsIPrincipal principal); */
  NS_IMETHOD GetUAWidgetScope(nsIPrincipal *principal, JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] jsval getSandboxMetadata (in jsval sandbox); */
  NS_IMETHOD GetSandboxMetadata(JS::HandleValue sandbox, JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] void setSandboxMetadata (in jsval sandbox, in jsval metadata); */
  NS_IMETHOD SetSandboxMetadata(JS::HandleValue sandbox, JS::HandleValue metadata, JSContext* cx) = 0;

  /* [implicit_jscontext,optional_argc] jsval import (in AUTF8String aResourceURI, [optional] in jsval targetObj); */
  NS_IMETHOD Import(const nsACString& aResourceURI, JS::HandleValue targetObj, JSContext* cx, uint8_t _argc, JS::MutableHandleValue _retval) = 0;

  /* boolean isModuleLoaded (in AUTF8String aResourceURI); */
  NS_IMETHOD IsModuleLoaded(const nsACString& aResourceURI, bool *_retval) = 0;

  /* void unload (in AUTF8String registryLocation); */
  NS_IMETHOD Unload(const nsACString& registryLocation) = 0;

  /* [implicit_jscontext] void importGlobalProperties (in jsval aPropertyList); */
  NS_IMETHOD ImportGlobalProperties(JS::HandleValue aPropertyList, JSContext* cx) = 0;

  /* [implicit_jscontext] xpcIJSWeakReference getWeakReference (in jsval obj); */
  NS_IMETHOD GetWeakReference(JS::HandleValue obj, JSContext* cx, xpcIJSWeakReference **_retval) = 0;

  /* [implicit_jscontext] void forceGC (); */
  NS_IMETHOD ForceGC(JSContext* cx) = 0;

  /* void forceCC ([optional] in nsICycleCollectorListener aListener); */
  NS_IMETHOD ForceCC(nsICycleCollectorListener *aListener) = 0;

  /* nsICycleCollectorListener createCCLogger (); */
  NS_IMETHOD CreateCCLogger(nsICycleCollectorListener **_retval) = 0;

  /* void finishCC (); */
  NS_IMETHOD FinishCC(void) = 0;

  /* void ccSlice (in long long budget); */
  NS_IMETHOD CcSlice(int64_t budget) = 0;

  /* long getMaxCCSliceTimeSinceClear (); */
  NS_IMETHOD GetMaxCCSliceTimeSinceClear(int32_t *_retval) = 0;

  /* void clearMaxCCTime (); */
  NS_IMETHOD ClearMaxCCTime(void) = 0;

  /* [implicit_jscontext] void forceShrinkingGC (); */
  NS_IMETHOD ForceShrinkingGC(JSContext* cx) = 0;

  /* void schedulePreciseGC (in nsIScheduledGCCallback callback); */
  NS_IMETHOD SchedulePreciseGC(nsIScheduledGCCallback *callback) = 0;

  /* void schedulePreciseShrinkingGC (in nsIScheduledGCCallback callback); */
  NS_IMETHOD SchedulePreciseShrinkingGC(nsIScheduledGCCallback *callback) = 0;

  /* void unlinkGhostWindows (); */
  NS_IMETHOD UnlinkGhostWindows(void) = 0;

  /* void intentionallyLeak (); */
  NS_IMETHOD IntentionallyLeak(void) = 0;

  /* [implicit_jscontext] jsval getJSTestingFunctions (); */
  NS_IMETHOD GetJSTestingFunctions(JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] jsval getFunctionSourceLocation (in jsval func); */
  NS_IMETHOD GetFunctionSourceLocation(JS::HandleValue func, JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] jsval callFunctionWithAsyncStack (in jsval function, in nsIStackFrame stack, in AString asyncCause); */
  NS_IMETHOD CallFunctionWithAsyncStack(JS::HandleValue function, nsIStackFrame *stack, const nsAString& asyncCause, JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] jsval getGlobalForObject (in jsval obj); */
  NS_IMETHOD GetGlobalForObject(JS::HandleValue obj, JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] boolean isProxy (in jsval vobject); */
  NS_IMETHOD IsProxy(JS::HandleValue vobject, JSContext* cx, bool *_retval) = 0;

  /* [implicit_jscontext] jsval exportFunction (in jsval vfunction, in jsval vscope, [optional] in jsval voptions); */
  NS_IMETHOD ExportFunction(JS::HandleValue vfunction, JS::HandleValue vscope, JS::HandleValue voptions, JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] jsval createObjectIn (in jsval vobj, [optional] in jsval voptions); */
  NS_IMETHOD CreateObjectIn(JS::HandleValue vobj, JS::HandleValue voptions, JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] void makeObjectPropsNormal (in jsval vobj); */
  NS_IMETHOD MakeObjectPropsNormal(JS::HandleValue vobj, JSContext* cx) = 0;

  /* bool isDeadWrapper (in jsval obj); */
  NS_IMETHOD IsDeadWrapper(JS::HandleValue obj, bool *_retval) = 0;

  /* bool isRemoteProxy (in jsval val); */
  NS_IMETHOD IsRemoteProxy(JS::HandleValue val, bool *_retval) = 0;

  /* [implicit_jscontext] void recomputeWrappers ([optional] in jsval vobj); */
  NS_IMETHOD RecomputeWrappers(JS::HandleValue vobj, JSContext* cx) = 0;

  /* [implicit_jscontext] void setWantXrays (in jsval vscope); */
  NS_IMETHOD SetWantXrays(JS::HandleValue vscope, JSContext* cx) = 0;

  /* [implicit_jscontext] void dispatch (in jsval runnable, [optional] in jsval scope); */
  NS_IMETHOD Dispatch(JS::HandleValue runnable, JS::HandleValue scope, JSContext* cx) = 0;

  /* [implicit_jscontext] attribute boolean strict_mode; */
  NS_IMETHOD GetStrict_mode(JSContext* cx, bool *aStrict_mode) = 0;
  NS_IMETHOD SetStrict_mode(JSContext* cx, bool aStrict_mode) = 0;

  /* readonly attribute boolean isInAutomation; */
  NS_IMETHOD GetIsInAutomation(bool *aIsInAutomation) = 0;

  /* void exitIfInAutomation (); */
  NS_IMETHOD ExitIfInAutomation(void) = 0;

  /* void crashIfNotInAutomation (); */
  NS_IMETHOD CrashIfNotInAutomation(void) = 0;

  /* [implicit_jscontext] void setGCZeal (in long zeal); */
  NS_IMETHOD SetGCZeal(int32_t zeal, JSContext* cx) = 0;

  /* [implicit_jscontext] void nukeSandbox (in jsval obj); */
  NS_IMETHOD NukeSandbox(JS::HandleValue obj, JSContext* cx) = 0;

  /* [implicit_jscontext] void blockScriptForGlobal (in jsval global); */
  NS_IMETHOD BlockScriptForGlobal(JS::HandleValue global, JSContext* cx) = 0;

  /* [implicit_jscontext] void unblockScriptForGlobal (in jsval global); */
  NS_IMETHOD UnblockScriptForGlobal(JS::HandleValue global, JSContext* cx) = 0;

  /* bool isOpaqueWrapper (in jsval obj); */
  NS_IMETHOD IsOpaqueWrapper(JS::HandleValue obj, bool *_retval) = 0;

  /* bool isXrayWrapper (in jsval obj); */
  NS_IMETHOD IsXrayWrapper(JS::HandleValue obj, bool *_retval) = 0;

  /* [implicit_jscontext] jsval waiveXrays (in jsval aVal); */
  NS_IMETHOD WaiveXrays(JS::HandleValue aVal, JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] jsval unwaiveXrays (in jsval aVal); */
  NS_IMETHOD UnwaiveXrays(JS::HandleValue aVal, JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] string getClassName (in jsval aObj, in bool aUnwrap); */
  NS_IMETHOD GetClassName(JS::HandleValue aObj, bool aUnwrap, JSContext* cx, char * *_retval) = 0;

  /* nsIClassInfo getDOMClassInfo (in AString aClassName); */
  NS_IMETHOD GetDOMClassInfo(const nsAString& aClassName, nsIClassInfo **_retval) = 0;

  /* [implicit_jscontext] jsval getIncumbentGlobal ([optional] in jsval callback); */
  NS_IMETHOD GetIncumbentGlobal(JS::HandleValue callback, JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] nsISupports generateXPCWrappedJS (in jsval obj, [optional] in jsval scope); */
  NS_IMETHOD GenerateXPCWrappedJS(JS::HandleValue obj, JS::HandleValue scope, JSContext* cx, nsISupports **_retval) = 0;

  /* PRTime getWatchdogTimestamp (in AString aCategory); */
  NS_IMETHOD GetWatchdogTimestamp(const nsAString& aCategory, PRTime *_retval) = 0;

  /* [implicit_jscontext] jsval getJSEngineTelemetryValue (); */
  NS_IMETHOD GetJSEngineTelemetryValue(JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] jsval cloneInto (in jsval value, in jsval scope, [optional] in jsval options); */
  NS_IMETHOD CloneInto(JS::HandleValue value, JS::HandleValue scope, JS::HandleValue options, JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* nsIPrincipal getWebIDLCallerPrincipal (); */
  NS_IMETHOD GetWebIDLCallerPrincipal(nsIPrincipal **_retval) = 0;

  /* [implicit_jscontext] nsIPrincipal getObjectPrincipal (in jsval obj); */
  NS_IMETHOD GetObjectPrincipal(JS::HandleValue obj, JSContext* cx, nsIPrincipal **_retval) = 0;

  /* [implicit_jscontext] ACString getRealmLocation (in jsval obj); */
  NS_IMETHOD GetRealmLocation(JS::HandleValue obj, JSContext* cx, nsACString& _retval) = 0;

  /* double now (); */
  NS_IMETHOD Now(double *_retval) = 0;

  /* AUTF8String readUTF8File (in nsIFile file); */
  NS_IMETHOD ReadUTF8File(nsIFile *file, nsACString& _retval) = 0;

  /* AUTF8String readUTF8URI (in nsIURI url); */
  NS_IMETHOD ReadUTF8URI(nsIURI *url, nsACString& _retval) = 0;

  /* nsIEditorSpellCheck createSpellChecker (); */
  NS_IMETHOD CreateSpellChecker(nsIEditorSpellCheck **_retval) = 0;

  /* nsISupports createCommandLine ([optional] in nsIFile aWorkingDir); */
  NS_IMETHOD CreateCommandLine(nsIFile *aWorkingDir, nsISupports **_retval) = 0;

  /* nsICommandParams createCommandParams (); */
  NS_IMETHOD CreateCommandParams(nsICommandParams **_retval) = 0;

  /* nsILoadContext createLoadContext (); */
  NS_IMETHOD CreateLoadContext(nsILoadContext **_retval) = 0;

  /* nsILoadContext createPrivateLoadContext (); */
  NS_IMETHOD CreatePrivateLoadContext(nsILoadContext **_retval) = 0;

  /* nsIPersistentProperties createPersistentProperties (); */
  NS_IMETHOD CreatePersistentProperties(nsIPersistentProperties **_retval) = 0;

  /* nsIDocumentEncoder createDocumentEncoder (in string contentType); */
  NS_IMETHOD CreateDocumentEncoder(const char * contentType, nsIDocumentEncoder **_retval) = 0;

  /* nsIDocumentEncoder createHTMLCopyEncoder (); */
  NS_IMETHOD CreateHTMLCopyEncoder(nsIDocumentEncoder **_retval) = 0;

  /* readonly attribute Array<ACString> loadedModules; */
  NS_IMETHOD GetLoadedModules(nsTArray<nsCString >& aLoadedModules) = 0;

  /* readonly attribute Array<ACString> loadedComponents; */
  NS_IMETHOD GetLoadedComponents(nsTArray<nsCString >& aLoadedComponents) = 0;

  /* ACString getModuleImportStack (in AUTF8String aLocation); */
  NS_IMETHOD GetModuleImportStack(const nsACString& aLocation, nsACString& _retval) = 0;

  /* ACString getComponentLoadStack (in AUTF8String aLocation); */
  NS_IMETHOD GetComponentLoadStack(const nsACString& aLocation, nsACString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIXPCComponents_Utils, NS_IXPCCOMPONENTS_UTILS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIXPCCOMPONENTS_UTILS \
  NS_IMETHOD ReportError(JS::HandleValue error, JS::HandleValue stack, JSContext* cx) override; \
  NS_IMETHOD GetSandbox(nsIXPCComponents_utils_Sandbox **aSandbox) override; \
  NS_IMETHOD CreateServicesCache(JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD EvalInSandbox(const nsAString& source, JS::HandleValue sandbox, JS::HandleValue version, const nsACString& filename, int32_t lineNo, bool enforceFilenameRestrictions, JSContext* cx, uint8_t _argc, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetUAWidgetScope(nsIPrincipal *principal, JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetSandboxMetadata(JS::HandleValue sandbox, JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD SetSandboxMetadata(JS::HandleValue sandbox, JS::HandleValue metadata, JSContext* cx) override; \
  NS_IMETHOD Import(const nsACString& aResourceURI, JS::HandleValue targetObj, JSContext* cx, uint8_t _argc, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD IsModuleLoaded(const nsACString& aResourceURI, bool *_retval) override; \
  NS_IMETHOD Unload(const nsACString& registryLocation) override; \
  NS_IMETHOD ImportGlobalProperties(JS::HandleValue aPropertyList, JSContext* cx) override; \
  NS_IMETHOD GetWeakReference(JS::HandleValue obj, JSContext* cx, xpcIJSWeakReference **_retval) override; \
  NS_IMETHOD ForceGC(JSContext* cx) override; \
  NS_IMETHOD ForceCC(nsICycleCollectorListener *aListener) override; \
  NS_IMETHOD CreateCCLogger(nsICycleCollectorListener **_retval) override; \
  NS_IMETHOD FinishCC(void) override; \
  NS_IMETHOD CcSlice(int64_t budget) override; \
  NS_IMETHOD GetMaxCCSliceTimeSinceClear(int32_t *_retval) override; \
  NS_IMETHOD ClearMaxCCTime(void) override; \
  NS_IMETHOD ForceShrinkingGC(JSContext* cx) override; \
  NS_IMETHOD SchedulePreciseGC(nsIScheduledGCCallback *callback) override; \
  NS_IMETHOD SchedulePreciseShrinkingGC(nsIScheduledGCCallback *callback) override; \
  NS_IMETHOD UnlinkGhostWindows(void) override; \
  NS_IMETHOD IntentionallyLeak(void) override; \
  NS_IMETHOD GetJSTestingFunctions(JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetFunctionSourceLocation(JS::HandleValue func, JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD CallFunctionWithAsyncStack(JS::HandleValue function, nsIStackFrame *stack, const nsAString& asyncCause, JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetGlobalForObject(JS::HandleValue obj, JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD IsProxy(JS::HandleValue vobject, JSContext* cx, bool *_retval) override; \
  NS_IMETHOD ExportFunction(JS::HandleValue vfunction, JS::HandleValue vscope, JS::HandleValue voptions, JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD CreateObjectIn(JS::HandleValue vobj, JS::HandleValue voptions, JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD MakeObjectPropsNormal(JS::HandleValue vobj, JSContext* cx) override; \
  NS_IMETHOD IsDeadWrapper(JS::HandleValue obj, bool *_retval) override; \
  NS_IMETHOD IsRemoteProxy(JS::HandleValue val, bool *_retval) override; \
  NS_IMETHOD RecomputeWrappers(JS::HandleValue vobj, JSContext* cx) override; \
  NS_IMETHOD SetWantXrays(JS::HandleValue vscope, JSContext* cx) override; \
  NS_IMETHOD Dispatch(JS::HandleValue runnable, JS::HandleValue scope, JSContext* cx) override; \
  NS_IMETHOD GetStrict_mode(JSContext* cx, bool *aStrict_mode) override; \
  NS_IMETHOD SetStrict_mode(JSContext* cx, bool aStrict_mode) override; \
  NS_IMETHOD GetIsInAutomation(bool *aIsInAutomation) override; \
  NS_IMETHOD ExitIfInAutomation(void) override; \
  NS_IMETHOD CrashIfNotInAutomation(void) override; \
  NS_IMETHOD SetGCZeal(int32_t zeal, JSContext* cx) override; \
  NS_IMETHOD NukeSandbox(JS::HandleValue obj, JSContext* cx) override; \
  NS_IMETHOD BlockScriptForGlobal(JS::HandleValue global, JSContext* cx) override; \
  NS_IMETHOD UnblockScriptForGlobal(JS::HandleValue global, JSContext* cx) override; \
  NS_IMETHOD IsOpaqueWrapper(JS::HandleValue obj, bool *_retval) override; \
  NS_IMETHOD IsXrayWrapper(JS::HandleValue obj, bool *_retval) override; \
  NS_IMETHOD WaiveXrays(JS::HandleValue aVal, JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD UnwaiveXrays(JS::HandleValue aVal, JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetClassName(JS::HandleValue aObj, bool aUnwrap, JSContext* cx, char * *_retval) override; \
  NS_IMETHOD GetDOMClassInfo(const nsAString& aClassName, nsIClassInfo **_retval) override; \
  NS_IMETHOD GetIncumbentGlobal(JS::HandleValue callback, JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GenerateXPCWrappedJS(JS::HandleValue obj, JS::HandleValue scope, JSContext* cx, nsISupports **_retval) override; \
  NS_IMETHOD GetWatchdogTimestamp(const nsAString& aCategory, PRTime *_retval) override; \
  NS_IMETHOD GetJSEngineTelemetryValue(JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD CloneInto(JS::HandleValue value, JS::HandleValue scope, JS::HandleValue options, JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetWebIDLCallerPrincipal(nsIPrincipal **_retval) override; \
  NS_IMETHOD GetObjectPrincipal(JS::HandleValue obj, JSContext* cx, nsIPrincipal **_retval) override; \
  NS_IMETHOD GetRealmLocation(JS::HandleValue obj, JSContext* cx, nsACString& _retval) override; \
  NS_IMETHOD Now(double *_retval) override; \
  NS_IMETHOD ReadUTF8File(nsIFile *file, nsACString& _retval) override; \
  NS_IMETHOD ReadUTF8URI(nsIURI *url, nsACString& _retval) override; \
  NS_IMETHOD CreateSpellChecker(nsIEditorSpellCheck **_retval) override; \
  NS_IMETHOD CreateCommandLine(nsIFile *aWorkingDir, nsISupports **_retval) override; \
  NS_IMETHOD CreateCommandParams(nsICommandParams **_retval) override; \
  NS_IMETHOD CreateLoadContext(nsILoadContext **_retval) override; \
  NS_IMETHOD CreatePrivateLoadContext(nsILoadContext **_retval) override; \
  NS_IMETHOD CreatePersistentProperties(nsIPersistentProperties **_retval) override; \
  NS_IMETHOD CreateDocumentEncoder(const char * contentType, nsIDocumentEncoder **_retval) override; \
  NS_IMETHOD CreateHTMLCopyEncoder(nsIDocumentEncoder **_retval) override; \
  NS_IMETHOD GetLoadedModules(nsTArray<nsCString >& aLoadedModules) override; \
  NS_IMETHOD GetLoadedComponents(nsTArray<nsCString >& aLoadedComponents) override; \
  NS_IMETHOD GetModuleImportStack(const nsACString& aLocation, nsACString& _retval) override; \
  NS_IMETHOD GetComponentLoadStack(const nsACString& aLocation, nsACString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIXPCCOMPONENTS_UTILS \
  nsresult ReportError(JS::HandleValue error, JS::HandleValue stack, JSContext* cx); \
  nsresult GetSandbox(nsIXPCComponents_utils_Sandbox **aSandbox); \
  nsresult CreateServicesCache(JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult EvalInSandbox(const nsAString& source, JS::HandleValue sandbox, JS::HandleValue version, const nsACString& filename, int32_t lineNo, bool enforceFilenameRestrictions, JSContext* cx, uint8_t _argc, JS::MutableHandleValue _retval); \
  nsresult GetUAWidgetScope(nsIPrincipal *principal, JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult GetSandboxMetadata(JS::HandleValue sandbox, JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult SetSandboxMetadata(JS::HandleValue sandbox, JS::HandleValue metadata, JSContext* cx); \
  nsresult Import(const nsACString& aResourceURI, JS::HandleValue targetObj, JSContext* cx, uint8_t _argc, JS::MutableHandleValue _retval); \
  nsresult IsModuleLoaded(const nsACString& aResourceURI, bool *_retval); \
  nsresult Unload(const nsACString& registryLocation); \
  nsresult ImportGlobalProperties(JS::HandleValue aPropertyList, JSContext* cx); \
  nsresult GetWeakReference(JS::HandleValue obj, JSContext* cx, xpcIJSWeakReference **_retval); \
  nsresult ForceGC(JSContext* cx); \
  nsresult ForceCC(nsICycleCollectorListener *aListener); \
  nsresult CreateCCLogger(nsICycleCollectorListener **_retval); \
  nsresult FinishCC(void); \
  nsresult CcSlice(int64_t budget); \
  nsresult GetMaxCCSliceTimeSinceClear(int32_t *_retval); \
  nsresult ClearMaxCCTime(void); \
  nsresult ForceShrinkingGC(JSContext* cx); \
  nsresult SchedulePreciseGC(nsIScheduledGCCallback *callback); \
  nsresult SchedulePreciseShrinkingGC(nsIScheduledGCCallback *callback); \
  nsresult UnlinkGhostWindows(void); \
  nsresult IntentionallyLeak(void); \
  nsresult GetJSTestingFunctions(JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult GetFunctionSourceLocation(JS::HandleValue func, JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult CallFunctionWithAsyncStack(JS::HandleValue function, nsIStackFrame *stack, const nsAString& asyncCause, JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult GetGlobalForObject(JS::HandleValue obj, JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult IsProxy(JS::HandleValue vobject, JSContext* cx, bool *_retval); \
  nsresult ExportFunction(JS::HandleValue vfunction, JS::HandleValue vscope, JS::HandleValue voptions, JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult CreateObjectIn(JS::HandleValue vobj, JS::HandleValue voptions, JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult MakeObjectPropsNormal(JS::HandleValue vobj, JSContext* cx); \
  nsresult IsDeadWrapper(JS::HandleValue obj, bool *_retval); \
  nsresult IsRemoteProxy(JS::HandleValue val, bool *_retval); \
  nsresult RecomputeWrappers(JS::HandleValue vobj, JSContext* cx); \
  nsresult SetWantXrays(JS::HandleValue vscope, JSContext* cx); \
  nsresult Dispatch(JS::HandleValue runnable, JS::HandleValue scope, JSContext* cx); \
  nsresult GetStrict_mode(JSContext* cx, bool *aStrict_mode); \
  nsresult SetStrict_mode(JSContext* cx, bool aStrict_mode); \
  nsresult GetIsInAutomation(bool *aIsInAutomation); \
  nsresult ExitIfInAutomation(void); \
  nsresult CrashIfNotInAutomation(void); \
  nsresult SetGCZeal(int32_t zeal, JSContext* cx); \
  nsresult NukeSandbox(JS::HandleValue obj, JSContext* cx); \
  nsresult BlockScriptForGlobal(JS::HandleValue global, JSContext* cx); \
  nsresult UnblockScriptForGlobal(JS::HandleValue global, JSContext* cx); \
  nsresult IsOpaqueWrapper(JS::HandleValue obj, bool *_retval); \
  nsresult IsXrayWrapper(JS::HandleValue obj, bool *_retval); \
  nsresult WaiveXrays(JS::HandleValue aVal, JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult UnwaiveXrays(JS::HandleValue aVal, JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult GetClassName(JS::HandleValue aObj, bool aUnwrap, JSContext* cx, char * *_retval); \
  nsresult GetDOMClassInfo(const nsAString& aClassName, nsIClassInfo **_retval); \
  nsresult GetIncumbentGlobal(JS::HandleValue callback, JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult GenerateXPCWrappedJS(JS::HandleValue obj, JS::HandleValue scope, JSContext* cx, nsISupports **_retval); \
  nsresult GetWatchdogTimestamp(const nsAString& aCategory, PRTime *_retval); \
  nsresult GetJSEngineTelemetryValue(JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult CloneInto(JS::HandleValue value, JS::HandleValue scope, JS::HandleValue options, JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult GetWebIDLCallerPrincipal(nsIPrincipal **_retval); \
  nsresult GetObjectPrincipal(JS::HandleValue obj, JSContext* cx, nsIPrincipal **_retval); \
  nsresult GetRealmLocation(JS::HandleValue obj, JSContext* cx, nsACString& _retval); \
  nsresult Now(double *_retval); \
  nsresult ReadUTF8File(nsIFile *file, nsACString& _retval); \
  nsresult ReadUTF8URI(nsIURI *url, nsACString& _retval); \
  nsresult CreateSpellChecker(nsIEditorSpellCheck **_retval); \
  nsresult CreateCommandLine(nsIFile *aWorkingDir, nsISupports **_retval); \
  nsresult CreateCommandParams(nsICommandParams **_retval); \
  nsresult CreateLoadContext(nsILoadContext **_retval); \
  nsresult CreatePrivateLoadContext(nsILoadContext **_retval); \
  nsresult CreatePersistentProperties(nsIPersistentProperties **_retval); \
  nsresult CreateDocumentEncoder(const char * contentType, nsIDocumentEncoder **_retval); \
  nsresult CreateHTMLCopyEncoder(nsIDocumentEncoder **_retval); \
  nsresult GetLoadedModules(nsTArray<nsCString >& aLoadedModules); \
  nsresult GetLoadedComponents(nsTArray<nsCString >& aLoadedComponents); \
  nsresult GetModuleImportStack(const nsACString& aLocation, nsACString& _retval); \
  nsresult GetComponentLoadStack(const nsACString& aLocation, nsACString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIXPCCOMPONENTS_UTILS(_to) \
  NS_IMETHOD ReportError(JS::HandleValue error, JS::HandleValue stack, JSContext* cx) override { return _to ReportError(error, stack, cx); } \
  NS_IMETHOD GetSandbox(nsIXPCComponents_utils_Sandbox **aSandbox) override { return _to GetSandbox(aSandbox); } \
  NS_IMETHOD CreateServicesCache(JSContext* cx, JS::MutableHandleValue _retval) override { return _to CreateServicesCache(cx, _retval); } \
  NS_IMETHOD EvalInSandbox(const nsAString& source, JS::HandleValue sandbox, JS::HandleValue version, const nsACString& filename, int32_t lineNo, bool enforceFilenameRestrictions, JSContext* cx, uint8_t _argc, JS::MutableHandleValue _retval) override { return _to EvalInSandbox(source, sandbox, version, filename, lineNo, enforceFilenameRestrictions, cx, _argc, _retval); } \
  NS_IMETHOD GetUAWidgetScope(nsIPrincipal *principal, JSContext* cx, JS::MutableHandleValue _retval) override { return _to GetUAWidgetScope(principal, cx, _retval); } \
  NS_IMETHOD GetSandboxMetadata(JS::HandleValue sandbox, JSContext* cx, JS::MutableHandleValue _retval) override { return _to GetSandboxMetadata(sandbox, cx, _retval); } \
  NS_IMETHOD SetSandboxMetadata(JS::HandleValue sandbox, JS::HandleValue metadata, JSContext* cx) override { return _to SetSandboxMetadata(sandbox, metadata, cx); } \
  NS_IMETHOD Import(const nsACString& aResourceURI, JS::HandleValue targetObj, JSContext* cx, uint8_t _argc, JS::MutableHandleValue _retval) override { return _to Import(aResourceURI, targetObj, cx, _argc, _retval); } \
  NS_IMETHOD IsModuleLoaded(const nsACString& aResourceURI, bool *_retval) override { return _to IsModuleLoaded(aResourceURI, _retval); } \
  NS_IMETHOD Unload(const nsACString& registryLocation) override { return _to Unload(registryLocation); } \
  NS_IMETHOD ImportGlobalProperties(JS::HandleValue aPropertyList, JSContext* cx) override { return _to ImportGlobalProperties(aPropertyList, cx); } \
  NS_IMETHOD GetWeakReference(JS::HandleValue obj, JSContext* cx, xpcIJSWeakReference **_retval) override { return _to GetWeakReference(obj, cx, _retval); } \
  NS_IMETHOD ForceGC(JSContext* cx) override { return _to ForceGC(cx); } \
  NS_IMETHOD ForceCC(nsICycleCollectorListener *aListener) override { return _to ForceCC(aListener); } \
  NS_IMETHOD CreateCCLogger(nsICycleCollectorListener **_retval) override { return _to CreateCCLogger(_retval); } \
  NS_IMETHOD FinishCC(void) override { return _to FinishCC(); } \
  NS_IMETHOD CcSlice(int64_t budget) override { return _to CcSlice(budget); } \
  NS_IMETHOD GetMaxCCSliceTimeSinceClear(int32_t *_retval) override { return _to GetMaxCCSliceTimeSinceClear(_retval); } \
  NS_IMETHOD ClearMaxCCTime(void) override { return _to ClearMaxCCTime(); } \
  NS_IMETHOD ForceShrinkingGC(JSContext* cx) override { return _to ForceShrinkingGC(cx); } \
  NS_IMETHOD SchedulePreciseGC(nsIScheduledGCCallback *callback) override { return _to SchedulePreciseGC(callback); } \
  NS_IMETHOD SchedulePreciseShrinkingGC(nsIScheduledGCCallback *callback) override { return _to SchedulePreciseShrinkingGC(callback); } \
  NS_IMETHOD UnlinkGhostWindows(void) override { return _to UnlinkGhostWindows(); } \
  NS_IMETHOD IntentionallyLeak(void) override { return _to IntentionallyLeak(); } \
  NS_IMETHOD GetJSTestingFunctions(JSContext* cx, JS::MutableHandleValue _retval) override { return _to GetJSTestingFunctions(cx, _retval); } \
  NS_IMETHOD GetFunctionSourceLocation(JS::HandleValue func, JSContext* cx, JS::MutableHandleValue _retval) override { return _to GetFunctionSourceLocation(func, cx, _retval); } \
  NS_IMETHOD CallFunctionWithAsyncStack(JS::HandleValue function, nsIStackFrame *stack, const nsAString& asyncCause, JSContext* cx, JS::MutableHandleValue _retval) override { return _to CallFunctionWithAsyncStack(function, stack, asyncCause, cx, _retval); } \
  NS_IMETHOD GetGlobalForObject(JS::HandleValue obj, JSContext* cx, JS::MutableHandleValue _retval) override { return _to GetGlobalForObject(obj, cx, _retval); } \
  NS_IMETHOD IsProxy(JS::HandleValue vobject, JSContext* cx, bool *_retval) override { return _to IsProxy(vobject, cx, _retval); } \
  NS_IMETHOD ExportFunction(JS::HandleValue vfunction, JS::HandleValue vscope, JS::HandleValue voptions, JSContext* cx, JS::MutableHandleValue _retval) override { return _to ExportFunction(vfunction, vscope, voptions, cx, _retval); } \
  NS_IMETHOD CreateObjectIn(JS::HandleValue vobj, JS::HandleValue voptions, JSContext* cx, JS::MutableHandleValue _retval) override { return _to CreateObjectIn(vobj, voptions, cx, _retval); } \
  NS_IMETHOD MakeObjectPropsNormal(JS::HandleValue vobj, JSContext* cx) override { return _to MakeObjectPropsNormal(vobj, cx); } \
  NS_IMETHOD IsDeadWrapper(JS::HandleValue obj, bool *_retval) override { return _to IsDeadWrapper(obj, _retval); } \
  NS_IMETHOD IsRemoteProxy(JS::HandleValue val, bool *_retval) override { return _to IsRemoteProxy(val, _retval); } \
  NS_IMETHOD RecomputeWrappers(JS::HandleValue vobj, JSContext* cx) override { return _to RecomputeWrappers(vobj, cx); } \
  NS_IMETHOD SetWantXrays(JS::HandleValue vscope, JSContext* cx) override { return _to SetWantXrays(vscope, cx); } \
  NS_IMETHOD Dispatch(JS::HandleValue runnable, JS::HandleValue scope, JSContext* cx) override { return _to Dispatch(runnable, scope, cx); } \
  NS_IMETHOD GetStrict_mode(JSContext* cx, bool *aStrict_mode) override { return _to GetStrict_mode(cx, aStrict_mode); } \
  NS_IMETHOD SetStrict_mode(JSContext* cx, bool aStrict_mode) override { return _to SetStrict_mode(cx, aStrict_mode); } \
  NS_IMETHOD GetIsInAutomation(bool *aIsInAutomation) override { return _to GetIsInAutomation(aIsInAutomation); } \
  NS_IMETHOD ExitIfInAutomation(void) override { return _to ExitIfInAutomation(); } \
  NS_IMETHOD CrashIfNotInAutomation(void) override { return _to CrashIfNotInAutomation(); } \
  NS_IMETHOD SetGCZeal(int32_t zeal, JSContext* cx) override { return _to SetGCZeal(zeal, cx); } \
  NS_IMETHOD NukeSandbox(JS::HandleValue obj, JSContext* cx) override { return _to NukeSandbox(obj, cx); } \
  NS_IMETHOD BlockScriptForGlobal(JS::HandleValue global, JSContext* cx) override { return _to BlockScriptForGlobal(global, cx); } \
  NS_IMETHOD UnblockScriptForGlobal(JS::HandleValue global, JSContext* cx) override { return _to UnblockScriptForGlobal(global, cx); } \
  NS_IMETHOD IsOpaqueWrapper(JS::HandleValue obj, bool *_retval) override { return _to IsOpaqueWrapper(obj, _retval); } \
  NS_IMETHOD IsXrayWrapper(JS::HandleValue obj, bool *_retval) override { return _to IsXrayWrapper(obj, _retval); } \
  NS_IMETHOD WaiveXrays(JS::HandleValue aVal, JSContext* cx, JS::MutableHandleValue _retval) override { return _to WaiveXrays(aVal, cx, _retval); } \
  NS_IMETHOD UnwaiveXrays(JS::HandleValue aVal, JSContext* cx, JS::MutableHandleValue _retval) override { return _to UnwaiveXrays(aVal, cx, _retval); } \
  NS_IMETHOD GetClassName(JS::HandleValue aObj, bool aUnwrap, JSContext* cx, char * *_retval) override { return _to GetClassName(aObj, aUnwrap, cx, _retval); } \
  NS_IMETHOD GetDOMClassInfo(const nsAString& aClassName, nsIClassInfo **_retval) override { return _to GetDOMClassInfo(aClassName, _retval); } \
  NS_IMETHOD GetIncumbentGlobal(JS::HandleValue callback, JSContext* cx, JS::MutableHandleValue _retval) override { return _to GetIncumbentGlobal(callback, cx, _retval); } \
  NS_IMETHOD GenerateXPCWrappedJS(JS::HandleValue obj, JS::HandleValue scope, JSContext* cx, nsISupports **_retval) override { return _to GenerateXPCWrappedJS(obj, scope, cx, _retval); } \
  NS_IMETHOD GetWatchdogTimestamp(const nsAString& aCategory, PRTime *_retval) override { return _to GetWatchdogTimestamp(aCategory, _retval); } \
  NS_IMETHOD GetJSEngineTelemetryValue(JSContext* cx, JS::MutableHandleValue _retval) override { return _to GetJSEngineTelemetryValue(cx, _retval); } \
  NS_IMETHOD CloneInto(JS::HandleValue value, JS::HandleValue scope, JS::HandleValue options, JSContext* cx, JS::MutableHandleValue _retval) override { return _to CloneInto(value, scope, options, cx, _retval); } \
  NS_IMETHOD GetWebIDLCallerPrincipal(nsIPrincipal **_retval) override { return _to GetWebIDLCallerPrincipal(_retval); } \
  NS_IMETHOD GetObjectPrincipal(JS::HandleValue obj, JSContext* cx, nsIPrincipal **_retval) override { return _to GetObjectPrincipal(obj, cx, _retval); } \
  NS_IMETHOD GetRealmLocation(JS::HandleValue obj, JSContext* cx, nsACString& _retval) override { return _to GetRealmLocation(obj, cx, _retval); } \
  NS_IMETHOD Now(double *_retval) override { return _to Now(_retval); } \
  NS_IMETHOD ReadUTF8File(nsIFile *file, nsACString& _retval) override { return _to ReadUTF8File(file, _retval); } \
  NS_IMETHOD ReadUTF8URI(nsIURI *url, nsACString& _retval) override { return _to ReadUTF8URI(url, _retval); } \
  NS_IMETHOD CreateSpellChecker(nsIEditorSpellCheck **_retval) override { return _to CreateSpellChecker(_retval); } \
  NS_IMETHOD CreateCommandLine(nsIFile *aWorkingDir, nsISupports **_retval) override { return _to CreateCommandLine(aWorkingDir, _retval); } \
  NS_IMETHOD CreateCommandParams(nsICommandParams **_retval) override { return _to CreateCommandParams(_retval); } \
  NS_IMETHOD CreateLoadContext(nsILoadContext **_retval) override { return _to CreateLoadContext(_retval); } \
  NS_IMETHOD CreatePrivateLoadContext(nsILoadContext **_retval) override { return _to CreatePrivateLoadContext(_retval); } \
  NS_IMETHOD CreatePersistentProperties(nsIPersistentProperties **_retval) override { return _to CreatePersistentProperties(_retval); } \
  NS_IMETHOD CreateDocumentEncoder(const char * contentType, nsIDocumentEncoder **_retval) override { return _to CreateDocumentEncoder(contentType, _retval); } \
  NS_IMETHOD CreateHTMLCopyEncoder(nsIDocumentEncoder **_retval) override { return _to CreateHTMLCopyEncoder(_retval); } \
  NS_IMETHOD GetLoadedModules(nsTArray<nsCString >& aLoadedModules) override { return _to GetLoadedModules(aLoadedModules); } \
  NS_IMETHOD GetLoadedComponents(nsTArray<nsCString >& aLoadedComponents) override { return _to GetLoadedComponents(aLoadedComponents); } \
  NS_IMETHOD GetModuleImportStack(const nsACString& aLocation, nsACString& _retval) override { return _to GetModuleImportStack(aLocation, _retval); } \
  NS_IMETHOD GetComponentLoadStack(const nsACString& aLocation, nsACString& _retval) override { return _to GetComponentLoadStack(aLocation, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIXPCCOMPONENTS_UTILS(_to) \
  NS_IMETHOD ReportError(JS::HandleValue error, JS::HandleValue stack, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReportError(error, stack, cx); } \
  NS_IMETHOD GetSandbox(nsIXPCComponents_utils_Sandbox **aSandbox) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSandbox(aSandbox); } \
  NS_IMETHOD CreateServicesCache(JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateServicesCache(cx, _retval); } \
  NS_IMETHOD EvalInSandbox(const nsAString& source, JS::HandleValue sandbox, JS::HandleValue version, const nsACString& filename, int32_t lineNo, bool enforceFilenameRestrictions, JSContext* cx, uint8_t _argc, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EvalInSandbox(source, sandbox, version, filename, lineNo, enforceFilenameRestrictions, cx, _argc, _retval); } \
  NS_IMETHOD GetUAWidgetScope(nsIPrincipal *principal, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUAWidgetScope(principal, cx, _retval); } \
  NS_IMETHOD GetSandboxMetadata(JS::HandleValue sandbox, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSandboxMetadata(sandbox, cx, _retval); } \
  NS_IMETHOD SetSandboxMetadata(JS::HandleValue sandbox, JS::HandleValue metadata, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSandboxMetadata(sandbox, metadata, cx); } \
  NS_IMETHOD Import(const nsACString& aResourceURI, JS::HandleValue targetObj, JSContext* cx, uint8_t _argc, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Import(aResourceURI, targetObj, cx, _argc, _retval); } \
  NS_IMETHOD IsModuleLoaded(const nsACString& aResourceURI, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsModuleLoaded(aResourceURI, _retval); } \
  NS_IMETHOD Unload(const nsACString& registryLocation) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Unload(registryLocation); } \
  NS_IMETHOD ImportGlobalProperties(JS::HandleValue aPropertyList, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ImportGlobalProperties(aPropertyList, cx); } \
  NS_IMETHOD GetWeakReference(JS::HandleValue obj, JSContext* cx, xpcIJSWeakReference **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWeakReference(obj, cx, _retval); } \
  NS_IMETHOD ForceGC(JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ForceGC(cx); } \
  NS_IMETHOD ForceCC(nsICycleCollectorListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ForceCC(aListener); } \
  NS_IMETHOD CreateCCLogger(nsICycleCollectorListener **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateCCLogger(_retval); } \
  NS_IMETHOD FinishCC(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FinishCC(); } \
  NS_IMETHOD CcSlice(int64_t budget) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CcSlice(budget); } \
  NS_IMETHOD GetMaxCCSliceTimeSinceClear(int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMaxCCSliceTimeSinceClear(_retval); } \
  NS_IMETHOD ClearMaxCCTime(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearMaxCCTime(); } \
  NS_IMETHOD ForceShrinkingGC(JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ForceShrinkingGC(cx); } \
  NS_IMETHOD SchedulePreciseGC(nsIScheduledGCCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SchedulePreciseGC(callback); } \
  NS_IMETHOD SchedulePreciseShrinkingGC(nsIScheduledGCCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SchedulePreciseShrinkingGC(callback); } \
  NS_IMETHOD UnlinkGhostWindows(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnlinkGhostWindows(); } \
  NS_IMETHOD IntentionallyLeak(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IntentionallyLeak(); } \
  NS_IMETHOD GetJSTestingFunctions(JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetJSTestingFunctions(cx, _retval); } \
  NS_IMETHOD GetFunctionSourceLocation(JS::HandleValue func, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFunctionSourceLocation(func, cx, _retval); } \
  NS_IMETHOD CallFunctionWithAsyncStack(JS::HandleValue function, nsIStackFrame *stack, const nsAString& asyncCause, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CallFunctionWithAsyncStack(function, stack, asyncCause, cx, _retval); } \
  NS_IMETHOD GetGlobalForObject(JS::HandleValue obj, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetGlobalForObject(obj, cx, _retval); } \
  NS_IMETHOD IsProxy(JS::HandleValue vobject, JSContext* cx, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsProxy(vobject, cx, _retval); } \
  NS_IMETHOD ExportFunction(JS::HandleValue vfunction, JS::HandleValue vscope, JS::HandleValue voptions, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ExportFunction(vfunction, vscope, voptions, cx, _retval); } \
  NS_IMETHOD CreateObjectIn(JS::HandleValue vobj, JS::HandleValue voptions, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateObjectIn(vobj, voptions, cx, _retval); } \
  NS_IMETHOD MakeObjectPropsNormal(JS::HandleValue vobj, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MakeObjectPropsNormal(vobj, cx); } \
  NS_IMETHOD IsDeadWrapper(JS::HandleValue obj, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsDeadWrapper(obj, _retval); } \
  NS_IMETHOD IsRemoteProxy(JS::HandleValue val, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsRemoteProxy(val, _retval); } \
  NS_IMETHOD RecomputeWrappers(JS::HandleValue vobj, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RecomputeWrappers(vobj, cx); } \
  NS_IMETHOD SetWantXrays(JS::HandleValue vscope, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetWantXrays(vscope, cx); } \
  NS_IMETHOD Dispatch(JS::HandleValue runnable, JS::HandleValue scope, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Dispatch(runnable, scope, cx); } \
  NS_IMETHOD GetStrict_mode(JSContext* cx, bool *aStrict_mode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStrict_mode(cx, aStrict_mode); } \
  NS_IMETHOD SetStrict_mode(JSContext* cx, bool aStrict_mode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetStrict_mode(cx, aStrict_mode); } \
  NS_IMETHOD GetIsInAutomation(bool *aIsInAutomation) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsInAutomation(aIsInAutomation); } \
  NS_IMETHOD ExitIfInAutomation(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ExitIfInAutomation(); } \
  NS_IMETHOD CrashIfNotInAutomation(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CrashIfNotInAutomation(); } \
  NS_IMETHOD SetGCZeal(int32_t zeal, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetGCZeal(zeal, cx); } \
  NS_IMETHOD NukeSandbox(JS::HandleValue obj, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NukeSandbox(obj, cx); } \
  NS_IMETHOD BlockScriptForGlobal(JS::HandleValue global, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->BlockScriptForGlobal(global, cx); } \
  NS_IMETHOD UnblockScriptForGlobal(JS::HandleValue global, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnblockScriptForGlobal(global, cx); } \
  NS_IMETHOD IsOpaqueWrapper(JS::HandleValue obj, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsOpaqueWrapper(obj, _retval); } \
  NS_IMETHOD IsXrayWrapper(JS::HandleValue obj, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsXrayWrapper(obj, _retval); } \
  NS_IMETHOD WaiveXrays(JS::HandleValue aVal, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WaiveXrays(aVal, cx, _retval); } \
  NS_IMETHOD UnwaiveXrays(JS::HandleValue aVal, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnwaiveXrays(aVal, cx, _retval); } \
  NS_IMETHOD GetClassName(JS::HandleValue aObj, bool aUnwrap, JSContext* cx, char * *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetClassName(aObj, aUnwrap, cx, _retval); } \
  NS_IMETHOD GetDOMClassInfo(const nsAString& aClassName, nsIClassInfo **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDOMClassInfo(aClassName, _retval); } \
  NS_IMETHOD GetIncumbentGlobal(JS::HandleValue callback, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIncumbentGlobal(callback, cx, _retval); } \
  NS_IMETHOD GenerateXPCWrappedJS(JS::HandleValue obj, JS::HandleValue scope, JSContext* cx, nsISupports **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GenerateXPCWrappedJS(obj, scope, cx, _retval); } \
  NS_IMETHOD GetWatchdogTimestamp(const nsAString& aCategory, PRTime *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWatchdogTimestamp(aCategory, _retval); } \
  NS_IMETHOD GetJSEngineTelemetryValue(JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetJSEngineTelemetryValue(cx, _retval); } \
  NS_IMETHOD CloneInto(JS::HandleValue value, JS::HandleValue scope, JS::HandleValue options, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CloneInto(value, scope, options, cx, _retval); } \
  NS_IMETHOD GetWebIDLCallerPrincipal(nsIPrincipal **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWebIDLCallerPrincipal(_retval); } \
  NS_IMETHOD GetObjectPrincipal(JS::HandleValue obj, JSContext* cx, nsIPrincipal **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetObjectPrincipal(obj, cx, _retval); } \
  NS_IMETHOD GetRealmLocation(JS::HandleValue obj, JSContext* cx, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRealmLocation(obj, cx, _retval); } \
  NS_IMETHOD Now(double *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Now(_retval); } \
  NS_IMETHOD ReadUTF8File(nsIFile *file, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReadUTF8File(file, _retval); } \
  NS_IMETHOD ReadUTF8URI(nsIURI *url, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReadUTF8URI(url, _retval); } \
  NS_IMETHOD CreateSpellChecker(nsIEditorSpellCheck **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateSpellChecker(_retval); } \
  NS_IMETHOD CreateCommandLine(nsIFile *aWorkingDir, nsISupports **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateCommandLine(aWorkingDir, _retval); } \
  NS_IMETHOD CreateCommandParams(nsICommandParams **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateCommandParams(_retval); } \
  NS_IMETHOD CreateLoadContext(nsILoadContext **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateLoadContext(_retval); } \
  NS_IMETHOD CreatePrivateLoadContext(nsILoadContext **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreatePrivateLoadContext(_retval); } \
  NS_IMETHOD CreatePersistentProperties(nsIPersistentProperties **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreatePersistentProperties(_retval); } \
  NS_IMETHOD CreateDocumentEncoder(const char * contentType, nsIDocumentEncoder **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateDocumentEncoder(contentType, _retval); } \
  NS_IMETHOD CreateHTMLCopyEncoder(nsIDocumentEncoder **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateHTMLCopyEncoder(_retval); } \
  NS_IMETHOD GetLoadedModules(nsTArray<nsCString >& aLoadedModules) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadedModules(aLoadedModules); } \
  NS_IMETHOD GetLoadedComponents(nsTArray<nsCString >& aLoadedComponents) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadedComponents(aLoadedComponents); } \
  NS_IMETHOD GetModuleImportStack(const nsACString& aLocation, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetModuleImportStack(aLocation, _retval); } \
  NS_IMETHOD GetComponentLoadStack(const nsACString& aLocation, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetComponentLoadStack(aLocation, _retval); } 


/* starting interface:    nsIXPCComponents */
#define NS_IXPCCOMPONENTS_IID_STR "aa28aaf6-70ce-4b03-9514-afe43c7dfda8"

#define NS_IXPCCOMPONENTS_IID \
  {0xaa28aaf6, 0x70ce, 0x4b03, \
    { 0x95, 0x14, 0xaf, 0xe4, 0x3c, 0x7d, 0xfd, 0xa8 }}

class NS_NO_VTABLE nsIXPCComponents : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IXPCCOMPONENTS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIXPCComponents;

  /* readonly attribute nsIXPCComponents_Interfaces interfaces; */
  NS_IMETHOD GetInterfaces(nsIXPCComponents_Interfaces **aInterfaces) = 0;

  /* readonly attribute nsIXPCComponents_Results results; */
  NS_IMETHOD GetResults(nsIXPCComponents_Results **aResults) = 0;

  /* boolean isSuccessCode (in nsresult result); */
  NS_IMETHOD IsSuccessCode(nsresult result, bool *_retval) = 0;

  /* readonly attribute nsIXPCComponents_Classes classes; */
  NS_IMETHOD GetClasses(nsIXPCComponents_Classes **aClasses) = 0;

  /* readonly attribute nsIStackFrame stack; */
  NS_IMETHOD GetStack(nsIStackFrame **aStack) = 0;

  /* readonly attribute nsIComponentManager manager; */
  NS_IMETHOD GetManager(nsIComponentManager **aManager) = 0;

  /* readonly attribute nsIXPCComponents_Utils utils; */
  NS_IMETHOD GetUtils(nsIXPCComponents_Utils **aUtils) = 0;

  /* readonly attribute nsIXPCComponents_ID ID; */
  NS_IMETHOD GetID(nsIXPCComponents_ID **aID) = 0;

  /* readonly attribute nsIXPCComponents_Exception Exception; */
  NS_IMETHOD GetException(nsIXPCComponents_Exception **aException) = 0;

  /* readonly attribute nsIXPCComponents_Constructor Constructor; */
  NS_IMETHOD GetConstructor(nsIXPCComponents_Constructor **aConstructor) = 0;

  /* [implicit_jscontext] attribute jsval returnCode; */
  NS_IMETHOD GetReturnCode(JSContext* cx, JS::MutableHandleValue aReturnCode) = 0;
  NS_IMETHOD SetReturnCode(JSContext* cx, JS::HandleValue aReturnCode) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIXPCComponents, NS_IXPCCOMPONENTS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIXPCCOMPONENTS \
  NS_IMETHOD GetInterfaces(nsIXPCComponents_Interfaces **aInterfaces) override; \
  NS_IMETHOD GetResults(nsIXPCComponents_Results **aResults) override; \
  NS_IMETHOD IsSuccessCode(nsresult result, bool *_retval) override; \
  NS_IMETHOD GetClasses(nsIXPCComponents_Classes **aClasses) override; \
  NS_IMETHOD GetStack(nsIStackFrame **aStack) override; \
  NS_IMETHOD GetManager(nsIComponentManager **aManager) override; \
  NS_IMETHOD GetUtils(nsIXPCComponents_Utils **aUtils) override; \
  NS_IMETHOD GetID(nsIXPCComponents_ID **aID) override; \
  NS_IMETHOD GetException(nsIXPCComponents_Exception **aException) override; \
  NS_IMETHOD GetConstructor(nsIXPCComponents_Constructor **aConstructor) override; \
  NS_IMETHOD GetReturnCode(JSContext* cx, JS::MutableHandleValue aReturnCode) override; \
  NS_IMETHOD SetReturnCode(JSContext* cx, JS::HandleValue aReturnCode) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIXPCCOMPONENTS \
  nsresult GetInterfaces(nsIXPCComponents_Interfaces **aInterfaces); \
  nsresult GetResults(nsIXPCComponents_Results **aResults); \
  nsresult IsSuccessCode(nsresult result, bool *_retval); \
  nsresult GetClasses(nsIXPCComponents_Classes **aClasses); \
  nsresult GetStack(nsIStackFrame **aStack); \
  nsresult GetManager(nsIComponentManager **aManager); \
  nsresult GetUtils(nsIXPCComponents_Utils **aUtils); \
  nsresult GetID(nsIXPCComponents_ID **aID); \
  nsresult GetException(nsIXPCComponents_Exception **aException); \
  nsresult GetConstructor(nsIXPCComponents_Constructor **aConstructor); \
  nsresult GetReturnCode(JSContext* cx, JS::MutableHandleValue aReturnCode); \
  nsresult SetReturnCode(JSContext* cx, JS::HandleValue aReturnCode); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIXPCCOMPONENTS(_to) \
  NS_IMETHOD GetInterfaces(nsIXPCComponents_Interfaces **aInterfaces) override { return _to GetInterfaces(aInterfaces); } \
  NS_IMETHOD GetResults(nsIXPCComponents_Results **aResults) override { return _to GetResults(aResults); } \
  NS_IMETHOD IsSuccessCode(nsresult result, bool *_retval) override { return _to IsSuccessCode(result, _retval); } \
  NS_IMETHOD GetClasses(nsIXPCComponents_Classes **aClasses) override { return _to GetClasses(aClasses); } \
  NS_IMETHOD GetStack(nsIStackFrame **aStack) override { return _to GetStack(aStack); } \
  NS_IMETHOD GetManager(nsIComponentManager **aManager) override { return _to GetManager(aManager); } \
  NS_IMETHOD GetUtils(nsIXPCComponents_Utils **aUtils) override { return _to GetUtils(aUtils); } \
  NS_IMETHOD GetID(nsIXPCComponents_ID **aID) override { return _to GetID(aID); } \
  NS_IMETHOD GetException(nsIXPCComponents_Exception **aException) override { return _to GetException(aException); } \
  NS_IMETHOD GetConstructor(nsIXPCComponents_Constructor **aConstructor) override { return _to GetConstructor(aConstructor); } \
  NS_IMETHOD GetReturnCode(JSContext* cx, JS::MutableHandleValue aReturnCode) override { return _to GetReturnCode(cx, aReturnCode); } \
  NS_IMETHOD SetReturnCode(JSContext* cx, JS::HandleValue aReturnCode) override { return _to SetReturnCode(cx, aReturnCode); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIXPCCOMPONENTS(_to) \
  NS_IMETHOD GetInterfaces(nsIXPCComponents_Interfaces **aInterfaces) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInterfaces(aInterfaces); } \
  NS_IMETHOD GetResults(nsIXPCComponents_Results **aResults) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResults(aResults); } \
  NS_IMETHOD IsSuccessCode(nsresult result, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsSuccessCode(result, _retval); } \
  NS_IMETHOD GetClasses(nsIXPCComponents_Classes **aClasses) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetClasses(aClasses); } \
  NS_IMETHOD GetStack(nsIStackFrame **aStack) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStack(aStack); } \
  NS_IMETHOD GetManager(nsIComponentManager **aManager) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetManager(aManager); } \
  NS_IMETHOD GetUtils(nsIXPCComponents_Utils **aUtils) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUtils(aUtils); } \
  NS_IMETHOD GetID(nsIXPCComponents_ID **aID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetID(aID); } \
  NS_IMETHOD GetException(nsIXPCComponents_Exception **aException) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetException(aException); } \
  NS_IMETHOD GetConstructor(nsIXPCComponents_Constructor **aConstructor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetConstructor(aConstructor); } \
  NS_IMETHOD GetReturnCode(JSContext* cx, JS::MutableHandleValue aReturnCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetReturnCode(cx, aReturnCode); } \
  NS_IMETHOD SetReturnCode(JSContext* cx, JS::HandleValue aReturnCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetReturnCode(cx, aReturnCode); } 


#endif /* __gen_xpccomponents_h__ */
