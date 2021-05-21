/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/simpledb/nsISDBResults.idl
 */

#ifndef __gen_nsISDBResults_h__
#define __gen_nsISDBResults_h__


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

/* starting interface:    nsISDBResult */
#define NS_ISDBRESULT_IID_STR "bca19e01-b34e-4a48-8875-2f4cb871febf"

#define NS_ISDBRESULT_IID \
  {0xbca19e01, 0xb34e, 0x4a48, \
    { 0x88, 0x75, 0x2f, 0x4c, 0xb8, 0x71, 0xfe, 0xbf }}

class NS_NO_VTABLE nsISDBResult : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISDBRESULT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISDBResult;

  /* [must_use] Array<uint8_t> getAsArray (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetAsArray(nsTArray<uint8_t >& _retval) = 0;

  /* [implicit_jscontext,must_use] jsval getAsArrayBuffer (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetAsArrayBuffer(JSContext* cx, JS::MutableHandleValue _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISDBResult, NS_ISDBRESULT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISDBRESULT \
  [[nodiscard]] NS_IMETHOD GetAsArray(nsTArray<uint8_t >& _retval) override; \
  [[nodiscard]] NS_IMETHOD GetAsArrayBuffer(JSContext* cx, JS::MutableHandleValue _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISDBRESULT \
  [[nodiscard]] nsresult GetAsArray(nsTArray<uint8_t >& _retval); \
  [[nodiscard]] nsresult GetAsArrayBuffer(JSContext* cx, JS::MutableHandleValue _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISDBRESULT(_to) \
  [[nodiscard]] NS_IMETHOD GetAsArray(nsTArray<uint8_t >& _retval) override { return _to GetAsArray(_retval); } \
  [[nodiscard]] NS_IMETHOD GetAsArrayBuffer(JSContext* cx, JS::MutableHandleValue _retval) override { return _to GetAsArrayBuffer(cx, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISDBRESULT(_to) \
  [[nodiscard]] NS_IMETHOD GetAsArray(nsTArray<uint8_t >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsArray(_retval); } \
  [[nodiscard]] NS_IMETHOD GetAsArrayBuffer(JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsArrayBuffer(cx, _retval); } 


#endif /* __gen_nsISDBResults_h__ */
