/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/js/xpconnect/idl/nsIXPConnect.idl
 */

#ifndef __gen_nsIXPConnect_h__
#define __gen_nsIXPConnect_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "mozilla/AlreadyAddRefed.h"
#include "mozilla/Assertions.h"
#include "mozilla/DebugOnly.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#include "jspubtd.h"
#include "js/CompileOptions.h"
#include "js/TypeDecls.h"
#include "mozilla/Attributes.h"
#include "xptinfo.h"
#include "nsCOMPtr.h"
class nsWrapperCache;
class nsIPrincipal; /* forward declaration */

class nsIClassInfo; /* forward declaration */

class nsIVariant; /* forward declaration */

class nsIObjectInputStream; /* forward declaration */

class nsIObjectOutputStream; /* forward declaration */


/* starting interface:    nsIXPConnectJSObjectHolder */
#define NS_IXPCONNECTJSOBJECTHOLDER_IID_STR "73e6ff4a-ab99-4d99-ac00-ba39ccb8e4d7"

#define NS_IXPCONNECTJSOBJECTHOLDER_IID \
  {0x73e6ff4a, 0xab99, 0x4d99, \
    { 0xac, 0x00, 0xba, 0x39, 0xcc, 0xb8, 0xe4, 0xd7 }}

class NS_NO_VTABLE nsIXPConnectJSObjectHolder : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IXPCONNECTJSOBJECTHOLDER_IID)

  /* [nostdcall,notxpcom] JSObjectPtr GetJSObject (); */
  virtual JSObject * GetJSObject(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIXPConnectJSObjectHolder, NS_IXPCONNECTJSOBJECTHOLDER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIXPCONNECTJSOBJECTHOLDER \
  virtual JSObject * GetJSObject(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIXPCONNECTJSOBJECTHOLDER \
  JSObject * GetJSObject(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIXPCONNECTJSOBJECTHOLDER(_to) \
  virtual JSObject * GetJSObject(void) override { return _to GetJSObject(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIXPCONNECTJSOBJECTHOLDER(_to) \
  virtual JSObject * GetJSObject(void) override; 


/* starting interface:    nsIXPConnectWrappedNative */
#define NS_IXPCONNECTWRAPPEDNATIVE_IID_STR "e787be29-db5d-4a45-a3d6-1de1d6b85c30"

#define NS_IXPCONNECTWRAPPEDNATIVE_IID \
  {0xe787be29, 0xdb5d, 0x4a45, \
    { 0xa3, 0xd6, 0x1d, 0xe1, 0xd6, 0xb8, 0x5c, 0x30 }}

class nsIXPConnectWrappedNative : public nsIXPConnectJSObjectHolder {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IXPCONNECTWRAPPEDNATIVE_IID)

  /* void debugDump (in short depth); */
  NS_IMETHOD DebugDump(int16_t depth) = 0;

     /**
     * Faster access to the native object from C++.  Will never return null.
     */
    nsISupports* Native() const { return mIdentity; }
protected:
    nsCOMPtr<nsISupports> mIdentity;
public:
};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIXPConnectWrappedNative, NS_IXPCONNECTWRAPPEDNATIVE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIXPCONNECTWRAPPEDNATIVE \
  NS_IMETHOD DebugDump(int16_t depth) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIXPCONNECTWRAPPEDNATIVE \
  nsresult DebugDump(int16_t depth); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIXPCONNECTWRAPPEDNATIVE(_to) \
  NS_IMETHOD DebugDump(int16_t depth) override { return _to DebugDump(depth); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIXPCONNECTWRAPPEDNATIVE(_to) \
  NS_IMETHOD DebugDump(int16_t depth) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DebugDump(depth); } \


/* starting interface:    nsIXPConnectWrappedJS */
#define NS_IXPCONNECTWRAPPEDJS_IID_STR "3a01b0d6-074b-49ed-bac3-08c76366cae4"

#define NS_IXPCONNECTWRAPPEDJS_IID \
  {0x3a01b0d6, 0x074b, 0x49ed, \
    { 0xba, 0xc3, 0x08, 0xc7, 0x63, 0x66, 0xca, 0xe4 }}

class NS_NO_VTABLE nsIXPConnectWrappedJS : public nsIXPConnectJSObjectHolder {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IXPCONNECTWRAPPEDJS_IID)

  /* readonly attribute nsIIDPtr InterfaceIID; */
  NS_IMETHOD GetInterfaceIID(nsIID * * aInterfaceIID) = 0;

  /* [nostdcall,notxpcom] JSObjectPtr GetJSObjectGlobal (); */
  virtual JSObject * GetJSObjectGlobal(void) = 0;

  /* void debugDump (in short depth); */
  NS_IMETHOD DebugDump(int16_t depth) = 0;

  /* void aggregatedQueryInterface (in nsIIDRef uuid, [iid_is (uuid), retval] out nsQIResult result); */
  NS_IMETHOD AggregatedQueryInterface(const nsIID & uuid, void * * result) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIXPConnectWrappedJS, NS_IXPCONNECTWRAPPEDJS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIXPCONNECTWRAPPEDJS \
  NS_IMETHOD GetInterfaceIID(nsIID * * aInterfaceIID) override; \
  virtual JSObject * GetJSObjectGlobal(void) override; \
  NS_IMETHOD DebugDump(int16_t depth) override; \
  NS_IMETHOD AggregatedQueryInterface(const nsIID & uuid, void * * result) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIXPCONNECTWRAPPEDJS \
  nsresult GetInterfaceIID(nsIID * * aInterfaceIID); \
  JSObject * GetJSObjectGlobal(void); \
  nsresult DebugDump(int16_t depth); \
  nsresult AggregatedQueryInterface(const nsIID & uuid, void * * result); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIXPCONNECTWRAPPEDJS(_to) \
  NS_IMETHOD GetInterfaceIID(nsIID * * aInterfaceIID) override { return _to GetInterfaceIID(aInterfaceIID); } \
  virtual JSObject * GetJSObjectGlobal(void) override { return _to GetJSObjectGlobal(); } \
  NS_IMETHOD DebugDump(int16_t depth) override { return _to DebugDump(depth); } \
  NS_IMETHOD AggregatedQueryInterface(const nsIID & uuid, void * * result) override { return _to AggregatedQueryInterface(uuid, result); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIXPCONNECTWRAPPEDJS(_to) \
  NS_IMETHOD GetInterfaceIID(nsIID * * aInterfaceIID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInterfaceIID(aInterfaceIID); } \
  virtual JSObject * GetJSObjectGlobal(void) override; \
  NS_IMETHOD DebugDump(int16_t depth) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DebugDump(depth); } \
  NS_IMETHOD AggregatedQueryInterface(const nsIID & uuid, void * * result) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AggregatedQueryInterface(uuid, result); } 


/* starting interface:    nsIXPConnectWrappedJSUnmarkGray */
#define NS_IXPCONNECTWRAPPEDJSUNMARKGRAY_IID_STR "c02a0ce6-275f-4ea1-9c23-08494898b070"

#define NS_IXPCONNECTWRAPPEDJSUNMARKGRAY_IID \
  {0xc02a0ce6, 0x275f, 0x4ea1, \
    { 0x9c, 0x23, 0x08, 0x49, 0x48, 0x98, 0xb0, 0x70 }}

class NS_NO_VTABLE nsIXPConnectWrappedJSUnmarkGray : public nsIXPConnectWrappedJS {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IXPCONNECTWRAPPEDJSUNMARKGRAY_IID)

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIXPConnectWrappedJSUnmarkGray, NS_IXPCONNECTWRAPPEDJSUNMARKGRAY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIXPCONNECTWRAPPEDJSUNMARKGRAY \
  /* no methods! */

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIXPCONNECTWRAPPEDJSUNMARKGRAY \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIXPCONNECTWRAPPEDJSUNMARKGRAY(_to) \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIXPCONNECTWRAPPEDJSUNMARKGRAY(_to) \
  /* no methods! */


/* starting interface:    nsIXPConnect */
#define NS_IXPCONNECT_IID_STR "768507b5-b981-40c7-8276-f6a1da502a24"

#define NS_IXPCONNECT_IID \
  {0x768507b5, 0xb981, 0x40c7, \
    { 0x82, 0x76, 0xf6, 0xa1, 0xda, 0x50, 0x2a, 0x24 }}

class nsIXPConnect : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IXPCONNECT_IID)

     // This gets a non-addref'd pointer.
    static nsIXPConnect* XPConnect();
  /* JSObjectPtr wrapNative (in JSContextPtr aJSContext, in JSObjectPtr aScope, in nsISupports aCOMObj, in nsIIDRef aIID); */
  NS_IMETHOD WrapNative(JSContext * aJSContext, JSObject * aScope, nsISupports *aCOMObj, const nsIID & aIID, JSObject * * _retval) = 0;

  /* void wrapNativeToJSVal (in JSContextPtr aJSContext, in JSObjectPtr aScope, in nsISupports aCOMObj, in nsWrapperCachePtr aCache, in nsIIDPtr aIID, in boolean aAllowWrapper, out jsval aVal); */
  NS_IMETHOD WrapNativeToJSVal(JSContext * aJSContext, JSObject * aScope, nsISupports *aCOMObj, nsWrapperCache * aCache, const nsIID * aIID, bool aAllowWrapper, JS::MutableHandleValue aVal) = 0;

  /* void wrapJS (in JSContextPtr aJSContext, in JSObjectPtr aJSObj, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
  NS_IMETHOD WrapJS(JSContext * aJSContext, JSObject * aJSObj, const nsIID & aIID, void * * result) = 0;

  /* nsIVariant jSValToVariant (in JSContextPtr cx, in jsval aJSVal); */
  NS_IMETHOD JSValToVariant(JSContext * cx, JS::HandleValue aJSVal, nsIVariant **_retval) = 0;

  /* nsIXPConnectWrappedNative getWrappedNativeOfJSObject (in JSContextPtr aJSContext, in JSObjectPtr aJSObj); */
  NS_IMETHOD GetWrappedNativeOfJSObject(JSContext * aJSContext, JSObject * aJSObj, nsIXPConnectWrappedNative **_retval) = 0;

  /* void debugDump (in short depth); */
  NS_IMETHOD DebugDump(int16_t depth) = 0;

  /* void debugDumpObject (in nsISupports aCOMObj, in short depth); */
  NS_IMETHOD DebugDumpObject(nsISupports *aCOMObj, int16_t depth) = 0;

  /* void debugDumpJSStack (in boolean showArgs, in boolean showLocals, in boolean showThisProps); */
  NS_IMETHOD DebugDumpJSStack(bool showArgs, bool showLocals, bool showThisProps) = 0;

  /* void wrapJSAggregatedToNative (in nsISupports aOuter, in JSContextPtr aJSContext, in JSObjectPtr aJSObj, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
  NS_IMETHOD WrapJSAggregatedToNative(nsISupports *aOuter, JSContext * aJSContext, JSObject * aJSObj, const nsIID & aIID, void * * result) = 0;

  /* jsval variantToJS (in JSContextPtr ctx, in JSObjectPtr scope, in nsIVariant value); */
  NS_IMETHOD VariantToJS(JSContext * ctx, JSObject * scope, nsIVariant *value, JS::MutableHandleValue _retval) = 0;

  /* nsIVariant JSToVariant (in JSContextPtr ctx, in jsval value); */
  NS_IMETHOD JSToVariant(JSContext * ctx, JS::HandleValue value, nsIVariant **_retval) = 0;

  /* [noscript] JSObjectPtr createSandbox (in JSContextPtr cx, in nsIPrincipal principal); */
  NS_IMETHOD CreateSandbox(JSContext * cx, nsIPrincipal *principal, JSObject * * _retval) = 0;

  /* [noscript] jsval evalInSandboxObject (in AString source, in string filename, in JSContextPtr cx, in JSObjectPtr sandbox); */
  NS_IMETHOD EvalInSandboxObject(const nsAString& source, const char * filename, JSContext * cx, JSObject * sandbox, JS::MutableHandleValue _retval) = 0;

  /* [noscript] void writeScript (in nsIObjectOutputStream aStream, in JSContextPtr aJSContext, in JSScriptPtr aJSScript); */
  NS_IMETHOD WriteScript(nsIObjectOutputStream *aStream, JSContext * aJSContext, JSScript * aJSScript) = 0;

  /* [noscript] JSScriptPtr readScript (in nsIObjectInputStream aStream, in JSContextPtr aJSContext, in const_JSReadOnlyCompileOptionsRef aOptions); */
  NS_IMETHOD ReadScript(nsIObjectInputStream *aStream, JSContext * aJSContext, const JS::ReadOnlyCompileOptions & aOptions, JSScript * * _retval) = 0;

  /* [infallible] readonly attribute boolean isShuttingDown; */
  NS_IMETHOD GetIsShuttingDown(bool *aIsShuttingDown) = 0;
  inline bool  GetIsShuttingDown()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsShuttingDown(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIXPConnect, NS_IXPCONNECT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIXPCONNECT \
  NS_IMETHOD WrapNative(JSContext * aJSContext, JSObject * aScope, nsISupports *aCOMObj, const nsIID & aIID, JSObject * * _retval) override; \
  NS_IMETHOD WrapNativeToJSVal(JSContext * aJSContext, JSObject * aScope, nsISupports *aCOMObj, nsWrapperCache * aCache, const nsIID * aIID, bool aAllowWrapper, JS::MutableHandleValue aVal) override; \
  NS_IMETHOD WrapJS(JSContext * aJSContext, JSObject * aJSObj, const nsIID & aIID, void * * result) override; \
  NS_IMETHOD JSValToVariant(JSContext * cx, JS::HandleValue aJSVal, nsIVariant **_retval) override; \
  NS_IMETHOD GetWrappedNativeOfJSObject(JSContext * aJSContext, JSObject * aJSObj, nsIXPConnectWrappedNative **_retval) override; \
  NS_IMETHOD DebugDump(int16_t depth) override; \
  NS_IMETHOD DebugDumpObject(nsISupports *aCOMObj, int16_t depth) override; \
  NS_IMETHOD DebugDumpJSStack(bool showArgs, bool showLocals, bool showThisProps) override; \
  NS_IMETHOD WrapJSAggregatedToNative(nsISupports *aOuter, JSContext * aJSContext, JSObject * aJSObj, const nsIID & aIID, void * * result) override; \
  NS_IMETHOD VariantToJS(JSContext * ctx, JSObject * scope, nsIVariant *value, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD JSToVariant(JSContext * ctx, JS::HandleValue value, nsIVariant **_retval) override; \
  NS_IMETHOD CreateSandbox(JSContext * cx, nsIPrincipal *principal, JSObject * * _retval) override; \
  NS_IMETHOD EvalInSandboxObject(const nsAString& source, const char * filename, JSContext * cx, JSObject * sandbox, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD WriteScript(nsIObjectOutputStream *aStream, JSContext * aJSContext, JSScript * aJSScript) override; \
  NS_IMETHOD ReadScript(nsIObjectInputStream *aStream, JSContext * aJSContext, const JS::ReadOnlyCompileOptions & aOptions, JSScript * * _retval) override; \
  using nsIXPConnect::GetIsShuttingDown; \
  NS_IMETHOD GetIsShuttingDown(bool *aIsShuttingDown) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIXPCONNECT \
  nsresult WrapNative(JSContext * aJSContext, JSObject * aScope, nsISupports *aCOMObj, const nsIID & aIID, JSObject * * _retval); \
  nsresult WrapNativeToJSVal(JSContext * aJSContext, JSObject * aScope, nsISupports *aCOMObj, nsWrapperCache * aCache, const nsIID * aIID, bool aAllowWrapper, JS::MutableHandleValue aVal); \
  nsresult WrapJS(JSContext * aJSContext, JSObject * aJSObj, const nsIID & aIID, void * * result); \
  nsresult JSValToVariant(JSContext * cx, JS::HandleValue aJSVal, nsIVariant **_retval); \
  nsresult GetWrappedNativeOfJSObject(JSContext * aJSContext, JSObject * aJSObj, nsIXPConnectWrappedNative **_retval); \
  nsresult DebugDump(int16_t depth); \
  nsresult DebugDumpObject(nsISupports *aCOMObj, int16_t depth); \
  nsresult DebugDumpJSStack(bool showArgs, bool showLocals, bool showThisProps); \
  nsresult WrapJSAggregatedToNative(nsISupports *aOuter, JSContext * aJSContext, JSObject * aJSObj, const nsIID & aIID, void * * result); \
  nsresult VariantToJS(JSContext * ctx, JSObject * scope, nsIVariant *value, JS::MutableHandleValue _retval); \
  nsresult JSToVariant(JSContext * ctx, JS::HandleValue value, nsIVariant **_retval); \
  nsresult CreateSandbox(JSContext * cx, nsIPrincipal *principal, JSObject * * _retval); \
  nsresult EvalInSandboxObject(const nsAString& source, const char * filename, JSContext * cx, JSObject * sandbox, JS::MutableHandleValue _retval); \
  nsresult WriteScript(nsIObjectOutputStream *aStream, JSContext * aJSContext, JSScript * aJSScript); \
  nsresult ReadScript(nsIObjectInputStream *aStream, JSContext * aJSContext, const JS::ReadOnlyCompileOptions & aOptions, JSScript * * _retval); \
  using nsIXPConnect::GetIsShuttingDown; \
  nsresult GetIsShuttingDown(bool *aIsShuttingDown); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIXPCONNECT(_to) \
  NS_IMETHOD WrapNative(JSContext * aJSContext, JSObject * aScope, nsISupports *aCOMObj, const nsIID & aIID, JSObject * * _retval) override { return _to WrapNative(aJSContext, aScope, aCOMObj, aIID, _retval); } \
  NS_IMETHOD WrapNativeToJSVal(JSContext * aJSContext, JSObject * aScope, nsISupports *aCOMObj, nsWrapperCache * aCache, const nsIID * aIID, bool aAllowWrapper, JS::MutableHandleValue aVal) override { return _to WrapNativeToJSVal(aJSContext, aScope, aCOMObj, aCache, aIID, aAllowWrapper, aVal); } \
  NS_IMETHOD WrapJS(JSContext * aJSContext, JSObject * aJSObj, const nsIID & aIID, void * * result) override { return _to WrapJS(aJSContext, aJSObj, aIID, result); } \
  NS_IMETHOD JSValToVariant(JSContext * cx, JS::HandleValue aJSVal, nsIVariant **_retval) override { return _to JSValToVariant(cx, aJSVal, _retval); } \
  NS_IMETHOD GetWrappedNativeOfJSObject(JSContext * aJSContext, JSObject * aJSObj, nsIXPConnectWrappedNative **_retval) override { return _to GetWrappedNativeOfJSObject(aJSContext, aJSObj, _retval); } \
  NS_IMETHOD DebugDump(int16_t depth) override { return _to DebugDump(depth); } \
  NS_IMETHOD DebugDumpObject(nsISupports *aCOMObj, int16_t depth) override { return _to DebugDumpObject(aCOMObj, depth); } \
  NS_IMETHOD DebugDumpJSStack(bool showArgs, bool showLocals, bool showThisProps) override { return _to DebugDumpJSStack(showArgs, showLocals, showThisProps); } \
  NS_IMETHOD WrapJSAggregatedToNative(nsISupports *aOuter, JSContext * aJSContext, JSObject * aJSObj, const nsIID & aIID, void * * result) override { return _to WrapJSAggregatedToNative(aOuter, aJSContext, aJSObj, aIID, result); } \
  NS_IMETHOD VariantToJS(JSContext * ctx, JSObject * scope, nsIVariant *value, JS::MutableHandleValue _retval) override { return _to VariantToJS(ctx, scope, value, _retval); } \
  NS_IMETHOD JSToVariant(JSContext * ctx, JS::HandleValue value, nsIVariant **_retval) override { return _to JSToVariant(ctx, value, _retval); } \
  NS_IMETHOD CreateSandbox(JSContext * cx, nsIPrincipal *principal, JSObject * * _retval) override { return _to CreateSandbox(cx, principal, _retval); } \
  NS_IMETHOD EvalInSandboxObject(const nsAString& source, const char * filename, JSContext * cx, JSObject * sandbox, JS::MutableHandleValue _retval) override { return _to EvalInSandboxObject(source, filename, cx, sandbox, _retval); } \
  NS_IMETHOD WriteScript(nsIObjectOutputStream *aStream, JSContext * aJSContext, JSScript * aJSScript) override { return _to WriteScript(aStream, aJSContext, aJSScript); } \
  NS_IMETHOD ReadScript(nsIObjectInputStream *aStream, JSContext * aJSContext, const JS::ReadOnlyCompileOptions & aOptions, JSScript * * _retval) override { return _to ReadScript(aStream, aJSContext, aOptions, _retval); } \
  using nsIXPConnect::GetIsShuttingDown; \
  NS_IMETHOD GetIsShuttingDown(bool *aIsShuttingDown) override { return _to GetIsShuttingDown(aIsShuttingDown); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIXPCONNECT(_to) \
  NS_IMETHOD WrapNative(JSContext * aJSContext, JSObject * aScope, nsISupports *aCOMObj, const nsIID & aIID, JSObject * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WrapNative(aJSContext, aScope, aCOMObj, aIID, _retval); } \
  NS_IMETHOD WrapNativeToJSVal(JSContext * aJSContext, JSObject * aScope, nsISupports *aCOMObj, nsWrapperCache * aCache, const nsIID * aIID, bool aAllowWrapper, JS::MutableHandleValue aVal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WrapNativeToJSVal(aJSContext, aScope, aCOMObj, aCache, aIID, aAllowWrapper, aVal); } \
  NS_IMETHOD WrapJS(JSContext * aJSContext, JSObject * aJSObj, const nsIID & aIID, void * * result) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WrapJS(aJSContext, aJSObj, aIID, result); } \
  NS_IMETHOD JSValToVariant(JSContext * cx, JS::HandleValue aJSVal, nsIVariant **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->JSValToVariant(cx, aJSVal, _retval); } \
  NS_IMETHOD GetWrappedNativeOfJSObject(JSContext * aJSContext, JSObject * aJSObj, nsIXPConnectWrappedNative **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWrappedNativeOfJSObject(aJSContext, aJSObj, _retval); } \
  NS_IMETHOD DebugDump(int16_t depth) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DebugDump(depth); } \
  NS_IMETHOD DebugDumpObject(nsISupports *aCOMObj, int16_t depth) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DebugDumpObject(aCOMObj, depth); } \
  NS_IMETHOD DebugDumpJSStack(bool showArgs, bool showLocals, bool showThisProps) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DebugDumpJSStack(showArgs, showLocals, showThisProps); } \
  NS_IMETHOD WrapJSAggregatedToNative(nsISupports *aOuter, JSContext * aJSContext, JSObject * aJSObj, const nsIID & aIID, void * * result) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WrapJSAggregatedToNative(aOuter, aJSContext, aJSObj, aIID, result); } \
  NS_IMETHOD VariantToJS(JSContext * ctx, JSObject * scope, nsIVariant *value, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->VariantToJS(ctx, scope, value, _retval); } \
  NS_IMETHOD JSToVariant(JSContext * ctx, JS::HandleValue value, nsIVariant **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->JSToVariant(ctx, value, _retval); } \
  NS_IMETHOD CreateSandbox(JSContext * cx, nsIPrincipal *principal, JSObject * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateSandbox(cx, principal, _retval); } \
  NS_IMETHOD EvalInSandboxObject(const nsAString& source, const char * filename, JSContext * cx, JSObject * sandbox, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EvalInSandboxObject(source, filename, cx, sandbox, _retval); } \
  NS_IMETHOD WriteScript(nsIObjectOutputStream *aStream, JSContext * aJSContext, JSScript * aJSScript) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WriteScript(aStream, aJSContext, aJSScript); } \
  NS_IMETHOD ReadScript(nsIObjectInputStream *aStream, JSContext * aJSContext, const JS::ReadOnlyCompileOptions & aOptions, JSScript * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReadScript(aStream, aJSContext, aOptions, _retval); } \
  NS_IMETHOD GetIsShuttingDown(bool *aIsShuttingDown) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsShuttingDown(aIsShuttingDown); } 


#endif /* __gen_nsIXPConnect_h__ */
