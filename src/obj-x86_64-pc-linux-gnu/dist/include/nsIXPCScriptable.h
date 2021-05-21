/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/js/xpconnect/idl/nsIXPCScriptable.idl
 */

#ifndef __gen_nsIXPCScriptable_h__
#define __gen_nsIXPCScriptable_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIClassInfo_h__
#include "nsIClassInfo.h"
#endif

#include "js/Value.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#ifdef XP_WIN
#undef GetClassName
#endif
#include "js/TypeDecls.h"
namespace JS {
class CallArgs;
}
class nsIXPConnectWrappedNative; /* forward declaration */

    // nsIXPCScriptable flags (only 32 bits available!). They are defined via
    // #defines so they can be used in #ifndef guards in xpc_map_end.h.
    #define XPC_SCRIPTABLE_WANT_PRECREATE                   (1 <<  0)
    // (1 << 1) is unused
    // (1 << 2) is unused
    // (1 << 3) is unused
    #define XPC_SCRIPTABLE_WANT_NEWENUMERATE                (1 <<  4)
    #define XPC_SCRIPTABLE_WANT_RESOLVE                     (1 <<  5)
    #define XPC_SCRIPTABLE_WANT_FINALIZE                    (1 <<  6)
    #define XPC_SCRIPTABLE_WANT_CALL                        (1 <<  7)
    #define XPC_SCRIPTABLE_WANT_CONSTRUCT                   (1 <<  8)
    #define XPC_SCRIPTABLE_WANT_HASINSTANCE                 (1 <<  9)
    #define XPC_SCRIPTABLE_USE_JSSTUB_FOR_ADDPROPERTY       (1 << 10)
    #define XPC_SCRIPTABLE_USE_JSSTUB_FOR_DELPROPERTY       (1 << 11)
    // (1 << 12) is unused
    #define XPC_SCRIPTABLE_DONT_ENUM_QUERY_INTERFACE        (1 << 13)
    // (1 << 14) is unused
    // (1 << 15) is unused
    #define XPC_SCRIPTABLE_ALLOW_PROP_MODS_DURING_RESOLVE   (1 << 16)
    // (1 << 17) is unused
    #define XPC_SCRIPTABLE_IS_GLOBAL_OBJECT                 (1 << 18)
    #define XPC_SCRIPTABLE_DONT_REFLECT_INTERFACE_NAMES     (1 << 19)

/* starting interface:    nsIXPCScriptable */
#define NS_IXPCSCRIPTABLE_IID_STR "19b70b26-7c3f-437f-a04a-2a8f9e28b617"

#define NS_IXPCSCRIPTABLE_IID \
  {0x19b70b26, 0x7c3f, 0x437f, \
    { 0xa0, 0x4a, 0x2a, 0x8f, 0x9e, 0x28, 0xb6, 0x17 }}

class nsIXPCScriptable : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IXPCSCRIPTABLE_IID)

  /* readonly attribute AUTF8String className; */
  NS_IMETHOD GetClassName(nsACString& aClassName) = 0;

  /* [nostdcall,notxpcom] uint32_t getScriptableFlags (); */
  virtual uint32_t GetScriptableFlags(void) = 0;

  /* [nostdcall,notxpcom] JSClassPtr getJSClass (); */
  virtual const JSClass * GetJSClass(void) = 0;

  /* void preCreate (in nsISupports nativeObj, in JSContextPtr cx, in JSObjectPtr globalObj, out JSObjectPtr parentObj); */
  NS_IMETHOD PreCreate(nsISupports *nativeObj, JSContext * cx, JSObject * globalObj, JSObject * * parentObj) = 0;

  /* boolean newEnumerate (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj, in JSMutableHandleIdVector properties, in boolean enumerableOnly); */
  NS_IMETHOD NewEnumerate(nsIXPConnectWrappedNative *wrapper, JSContext * cx, JSObject * obj, JS::MutableHandleIdVector properties, bool enumerableOnly, bool *_retval) = 0;

  /* boolean resolve (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj, in jsid id, out boolean resolvedp); */
  NS_IMETHOD Resolve(nsIXPConnectWrappedNative *wrapper, JSContext * cx, JSObject * obj, jsid id, bool *resolvedp, bool *_retval) = 0;

  /* void finalize (in nsIXPConnectWrappedNative wrapper, in JSFreeOpPtr fop, in JSObjectPtr obj); */
  NS_IMETHOD Finalize(nsIXPConnectWrappedNative *wrapper, JSFreeOp * fop, JSObject * obj) = 0;

  /* boolean call (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj, in JSCallArgsRef args); */
  NS_IMETHOD Call(nsIXPConnectWrappedNative *wrapper, JSContext * cx, JSObject * obj, const JS::CallArgs & args, bool *_retval) = 0;

  /* boolean construct (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj, in JSCallArgsRef args); */
  NS_IMETHOD Construct(nsIXPConnectWrappedNative *wrapper, JSContext * cx, JSObject * obj, const JS::CallArgs & args, bool *_retval) = 0;

  /* boolean hasInstance (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj, in jsval val, out boolean bp); */
  NS_IMETHOD HasInstance(nsIXPConnectWrappedNative *wrapper, JSContext * cx, JSObject * obj, JS::HandleValue val, bool *bp, bool *_retval) = 0;

     #define GET_IT(f_, c_) \
    bool f_() { \
        return 0 != (GetScriptableFlags() & XPC_SCRIPTABLE_##c_); \
    }
    GET_IT(WantPreCreate,                WANT_PRECREATE)
    GET_IT(WantNewEnumerate,             WANT_NEWENUMERATE)
    GET_IT(WantResolve,                  WANT_RESOLVE)
    GET_IT(WantFinalize,                 WANT_FINALIZE)
    GET_IT(WantCall,                     WANT_CALL)
    GET_IT(WantConstruct,                WANT_CONSTRUCT)
    GET_IT(WantHasInstance,              WANT_HASINSTANCE)
    GET_IT(UseJSStubForAddProperty,      USE_JSSTUB_FOR_ADDPROPERTY)
    GET_IT(UseJSStubForDelProperty,      USE_JSSTUB_FOR_DELPROPERTY)
    GET_IT(DontEnumQueryInterface,       DONT_ENUM_QUERY_INTERFACE)
    GET_IT(AllowPropModsDuringResolve,   ALLOW_PROP_MODS_DURING_RESOLVE)
    GET_IT(IsGlobalObject,               IS_GLOBAL_OBJECT)
    GET_IT(DontReflectInterfaceNames,    DONT_REFLECT_INTERFACE_NAMES)
    #undef GET_IT
};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIXPCScriptable, NS_IXPCSCRIPTABLE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIXPCSCRIPTABLE \
  NS_IMETHOD GetClassName(nsACString& aClassName) override; \
  virtual uint32_t GetScriptableFlags(void) override; \
  virtual const JSClass * GetJSClass(void) override; \
  NS_IMETHOD PreCreate(nsISupports *nativeObj, JSContext * cx, JSObject * globalObj, JSObject * * parentObj) override; \
  NS_IMETHOD NewEnumerate(nsIXPConnectWrappedNative *wrapper, JSContext * cx, JSObject * obj, JS::MutableHandleIdVector properties, bool enumerableOnly, bool *_retval) override; \
  NS_IMETHOD Resolve(nsIXPConnectWrappedNative *wrapper, JSContext * cx, JSObject * obj, jsid id, bool *resolvedp, bool *_retval) override; \
  NS_IMETHOD Finalize(nsIXPConnectWrappedNative *wrapper, JSFreeOp * fop, JSObject * obj) override; \
  NS_IMETHOD Call(nsIXPConnectWrappedNative *wrapper, JSContext * cx, JSObject * obj, const JS::CallArgs & args, bool *_retval) override; \
  NS_IMETHOD Construct(nsIXPConnectWrappedNative *wrapper, JSContext * cx, JSObject * obj, const JS::CallArgs & args, bool *_retval) override; \
  NS_IMETHOD HasInstance(nsIXPConnectWrappedNative *wrapper, JSContext * cx, JSObject * obj, JS::HandleValue val, bool *bp, bool *_retval) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIXPCSCRIPTABLE \
  nsresult GetClassName(nsACString& aClassName); \
  uint32_t GetScriptableFlags(void); \
  const JSClass * GetJSClass(void); \
  nsresult PreCreate(nsISupports *nativeObj, JSContext * cx, JSObject * globalObj, JSObject * * parentObj); \
  nsresult NewEnumerate(nsIXPConnectWrappedNative *wrapper, JSContext * cx, JSObject * obj, JS::MutableHandleIdVector properties, bool enumerableOnly, bool *_retval); \
  nsresult Resolve(nsIXPConnectWrappedNative *wrapper, JSContext * cx, JSObject * obj, jsid id, bool *resolvedp, bool *_retval); \
  nsresult Finalize(nsIXPConnectWrappedNative *wrapper, JSFreeOp * fop, JSObject * obj); \
  nsresult Call(nsIXPConnectWrappedNative *wrapper, JSContext * cx, JSObject * obj, const JS::CallArgs & args, bool *_retval); \
  nsresult Construct(nsIXPConnectWrappedNative *wrapper, JSContext * cx, JSObject * obj, const JS::CallArgs & args, bool *_retval); \
  nsresult HasInstance(nsIXPConnectWrappedNative *wrapper, JSContext * cx, JSObject * obj, JS::HandleValue val, bool *bp, bool *_retval); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIXPCSCRIPTABLE(_to) \
  NS_IMETHOD GetClassName(nsACString& aClassName) override { return _to GetClassName(aClassName); } \
  virtual uint32_t GetScriptableFlags(void) override { return _to GetScriptableFlags(); } \
  virtual const JSClass * GetJSClass(void) override { return _to GetJSClass(); } \
  NS_IMETHOD PreCreate(nsISupports *nativeObj, JSContext * cx, JSObject * globalObj, JSObject * * parentObj) override { return _to PreCreate(nativeObj, cx, globalObj, parentObj); } \
  NS_IMETHOD NewEnumerate(nsIXPConnectWrappedNative *wrapper, JSContext * cx, JSObject * obj, JS::MutableHandleIdVector properties, bool enumerableOnly, bool *_retval) override { return _to NewEnumerate(wrapper, cx, obj, properties, enumerableOnly, _retval); } \
  NS_IMETHOD Resolve(nsIXPConnectWrappedNative *wrapper, JSContext * cx, JSObject * obj, jsid id, bool *resolvedp, bool *_retval) override { return _to Resolve(wrapper, cx, obj, id, resolvedp, _retval); } \
  NS_IMETHOD Finalize(nsIXPConnectWrappedNative *wrapper, JSFreeOp * fop, JSObject * obj) override { return _to Finalize(wrapper, fop, obj); } \
  NS_IMETHOD Call(nsIXPConnectWrappedNative *wrapper, JSContext * cx, JSObject * obj, const JS::CallArgs & args, bool *_retval) override { return _to Call(wrapper, cx, obj, args, _retval); } \
  NS_IMETHOD Construct(nsIXPConnectWrappedNative *wrapper, JSContext * cx, JSObject * obj, const JS::CallArgs & args, bool *_retval) override { return _to Construct(wrapper, cx, obj, args, _retval); } \
  NS_IMETHOD HasInstance(nsIXPConnectWrappedNative *wrapper, JSContext * cx, JSObject * obj, JS::HandleValue val, bool *bp, bool *_retval) override { return _to HasInstance(wrapper, cx, obj, val, bp, _retval); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIXPCSCRIPTABLE(_to) \
  NS_IMETHOD GetClassName(nsACString& aClassName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetClassName(aClassName); } \
  virtual uint32_t GetScriptableFlags(void) override; \
  virtual const JSClass * GetJSClass(void) override; \
  NS_IMETHOD PreCreate(nsISupports *nativeObj, JSContext * cx, JSObject * globalObj, JSObject * * parentObj) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PreCreate(nativeObj, cx, globalObj, parentObj); } \
  NS_IMETHOD NewEnumerate(nsIXPConnectWrappedNative *wrapper, JSContext * cx, JSObject * obj, JS::MutableHandleIdVector properties, bool enumerableOnly, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewEnumerate(wrapper, cx, obj, properties, enumerableOnly, _retval); } \
  NS_IMETHOD Resolve(nsIXPConnectWrappedNative *wrapper, JSContext * cx, JSObject * obj, jsid id, bool *resolvedp, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Resolve(wrapper, cx, obj, id, resolvedp, _retval); } \
  NS_IMETHOD Finalize(nsIXPConnectWrappedNative *wrapper, JSFreeOp * fop, JSObject * obj) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Finalize(wrapper, fop, obj); } \
  NS_IMETHOD Call(nsIXPConnectWrappedNative *wrapper, JSContext * cx, JSObject * obj, const JS::CallArgs & args, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Call(wrapper, cx, obj, args, _retval); } \
  NS_IMETHOD Construct(nsIXPConnectWrappedNative *wrapper, JSContext * cx, JSObject * obj, const JS::CallArgs & args, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Construct(wrapper, cx, obj, args, _retval); } \
  NS_IMETHOD HasInstance(nsIXPConnectWrappedNative *wrapper, JSContext * cx, JSObject * obj, JS::HandleValue val, bool *bp, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasInstance(wrapper, cx, obj, val, bp, _retval); } \


#endif /* __gen_nsIXPCScriptable_h__ */
