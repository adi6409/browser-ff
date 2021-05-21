/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIDebug2.idl
 */

#ifndef __gen_nsIDebug2_h__
#define __gen_nsIDebug2_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIDebug2 */
#define NS_IDEBUG2_IID_STR "9641dc15-10fb-42e3-a285-18be90a5c10b"

#define NS_IDEBUG2_IID \
  {0x9641dc15, 0x10fb, 0x42e3, \
    { 0xa2, 0x85, 0x18, 0xbe, 0x90, 0xa5, 0xc1, 0x0b }}

class NS_NO_VTABLE nsIDebug2 : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDEBUG2_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDebug2;

  /* readonly attribute boolean isDebugBuild; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsDebugBuild(bool *aIsDebugBuild) = 0;

  /* readonly attribute long assertionCount; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAssertionCount(int32_t *aAssertionCount) = 0;

  /* readonly attribute bool isDebuggerAttached; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsDebuggerAttached(bool *aIsDebuggerAttached) = 0;

  /* void assertion (in string aStr, in string aExpr, in string aFile, in long aLine); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Assertion(const char * aStr, const char * aExpr, const char * aFile, int32_t aLine) = 0;

  /* void warning (in string aStr, in string aFile, in long aLine); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Warning(const char * aStr, const char * aFile, int32_t aLine) = 0;

  /* void break (in string aFile, in long aLine); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Break(const char * aFile, int32_t aLine) = 0;

  /* void abort (in string aFile, in long aLine); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Abort(const char * aFile, int32_t aLine) = 0;

  /* void rustPanic (in string aMessage); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RustPanic(const char * aMessage) = 0;

  /* void rustLog (in string aTarget, in string aMessage); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RustLog(const char * aTarget, const char * aMessage) = 0;

  /* void crashWithOOM (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CrashWithOOM(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDebug2, NS_IDEBUG2_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDEBUG2 \
  NS_IMETHOD GetIsDebugBuild(bool *aIsDebugBuild) override; \
  NS_IMETHOD GetAssertionCount(int32_t *aAssertionCount) override; \
  NS_IMETHOD GetIsDebuggerAttached(bool *aIsDebuggerAttached) override; \
  NS_IMETHOD Assertion(const char * aStr, const char * aExpr, const char * aFile, int32_t aLine) override; \
  NS_IMETHOD Warning(const char * aStr, const char * aFile, int32_t aLine) override; \
  NS_IMETHOD Break(const char * aFile, int32_t aLine) override; \
  NS_IMETHOD Abort(const char * aFile, int32_t aLine) override; \
  NS_IMETHOD RustPanic(const char * aMessage) override; \
  NS_IMETHOD RustLog(const char * aTarget, const char * aMessage) override; \
  NS_IMETHOD CrashWithOOM(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDEBUG2 \
  nsresult GetIsDebugBuild(bool *aIsDebugBuild); \
  nsresult GetAssertionCount(int32_t *aAssertionCount); \
  nsresult GetIsDebuggerAttached(bool *aIsDebuggerAttached); \
  nsresult Assertion(const char * aStr, const char * aExpr, const char * aFile, int32_t aLine); \
  nsresult Warning(const char * aStr, const char * aFile, int32_t aLine); \
  nsresult Break(const char * aFile, int32_t aLine); \
  nsresult Abort(const char * aFile, int32_t aLine); \
  nsresult RustPanic(const char * aMessage); \
  nsresult RustLog(const char * aTarget, const char * aMessage); \
  nsresult CrashWithOOM(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDEBUG2(_to) \
  NS_IMETHOD GetIsDebugBuild(bool *aIsDebugBuild) override { return _to GetIsDebugBuild(aIsDebugBuild); } \
  NS_IMETHOD GetAssertionCount(int32_t *aAssertionCount) override { return _to GetAssertionCount(aAssertionCount); } \
  NS_IMETHOD GetIsDebuggerAttached(bool *aIsDebuggerAttached) override { return _to GetIsDebuggerAttached(aIsDebuggerAttached); } \
  NS_IMETHOD Assertion(const char * aStr, const char * aExpr, const char * aFile, int32_t aLine) override { return _to Assertion(aStr, aExpr, aFile, aLine); } \
  NS_IMETHOD Warning(const char * aStr, const char * aFile, int32_t aLine) override { return _to Warning(aStr, aFile, aLine); } \
  NS_IMETHOD Break(const char * aFile, int32_t aLine) override { return _to Break(aFile, aLine); } \
  NS_IMETHOD Abort(const char * aFile, int32_t aLine) override { return _to Abort(aFile, aLine); } \
  NS_IMETHOD RustPanic(const char * aMessage) override { return _to RustPanic(aMessage); } \
  NS_IMETHOD RustLog(const char * aTarget, const char * aMessage) override { return _to RustLog(aTarget, aMessage); } \
  NS_IMETHOD CrashWithOOM(void) override { return _to CrashWithOOM(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDEBUG2(_to) \
  NS_IMETHOD GetIsDebugBuild(bool *aIsDebugBuild) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsDebugBuild(aIsDebugBuild); } \
  NS_IMETHOD GetAssertionCount(int32_t *aAssertionCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAssertionCount(aAssertionCount); } \
  NS_IMETHOD GetIsDebuggerAttached(bool *aIsDebuggerAttached) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsDebuggerAttached(aIsDebuggerAttached); } \
  NS_IMETHOD Assertion(const char * aStr, const char * aExpr, const char * aFile, int32_t aLine) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Assertion(aStr, aExpr, aFile, aLine); } \
  NS_IMETHOD Warning(const char * aStr, const char * aFile, int32_t aLine) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Warning(aStr, aFile, aLine); } \
  NS_IMETHOD Break(const char * aFile, int32_t aLine) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Break(aFile, aLine); } \
  NS_IMETHOD Abort(const char * aFile, int32_t aLine) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Abort(aFile, aLine); } \
  NS_IMETHOD RustPanic(const char * aMessage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RustPanic(aMessage); } \
  NS_IMETHOD RustLog(const char * aTarget, const char * aMessage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RustLog(aTarget, aMessage); } \
  NS_IMETHOD CrashWithOOM(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CrashWithOOM(); } 


#endif /* __gen_nsIDebug2_h__ */
