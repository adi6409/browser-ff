/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/simpledb/nsISDBRequest.idl
 */

#ifndef __gen_nsISDBRequest_h__
#define __gen_nsISDBRequest_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsISDBCallback; /* forward declaration */

class nsIVariant; /* forward declaration */


/* starting interface:    nsISDBRequest */
#define NS_ISDBREQUEST_IID_STR "13f05bcf-715c-427e-aac8-df9b2c1ec1e3"

#define NS_ISDBREQUEST_IID \
  {0x13f05bcf, 0x715c, 0x427e, \
    { 0xaa, 0xc8, 0xdf, 0x9b, 0x2c, 0x1e, 0xc1, 0xe3 }}

class NS_NO_VTABLE nsISDBRequest : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISDBREQUEST_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISDBRequest;

  /* [must_use] readonly attribute nsIVariant result; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetResult(nsIVariant **aResult) = 0;

  /* [must_use] readonly attribute nsresult resultCode; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetResultCode(nsresult *aResultCode) = 0;

  /* attribute nsISDBCallback callback; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCallback(nsISDBCallback **aCallback) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCallback(nsISDBCallback *aCallback) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISDBRequest, NS_ISDBREQUEST_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISDBREQUEST \
  [[nodiscard]] NS_IMETHOD GetResult(nsIVariant **aResult) override; \
  [[nodiscard]] NS_IMETHOD GetResultCode(nsresult *aResultCode) override; \
  NS_IMETHOD GetCallback(nsISDBCallback **aCallback) override; \
  NS_IMETHOD SetCallback(nsISDBCallback *aCallback) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISDBREQUEST \
  [[nodiscard]] nsresult GetResult(nsIVariant **aResult); \
  [[nodiscard]] nsresult GetResultCode(nsresult *aResultCode); \
  nsresult GetCallback(nsISDBCallback **aCallback); \
  nsresult SetCallback(nsISDBCallback *aCallback); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISDBREQUEST(_to) \
  [[nodiscard]] NS_IMETHOD GetResult(nsIVariant **aResult) override { return _to GetResult(aResult); } \
  [[nodiscard]] NS_IMETHOD GetResultCode(nsresult *aResultCode) override { return _to GetResultCode(aResultCode); } \
  NS_IMETHOD GetCallback(nsISDBCallback **aCallback) override { return _to GetCallback(aCallback); } \
  NS_IMETHOD SetCallback(nsISDBCallback *aCallback) override { return _to SetCallback(aCallback); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISDBREQUEST(_to) \
  [[nodiscard]] NS_IMETHOD GetResult(nsIVariant **aResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResult(aResult); } \
  [[nodiscard]] NS_IMETHOD GetResultCode(nsresult *aResultCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResultCode(aResultCode); } \
  NS_IMETHOD GetCallback(nsISDBCallback **aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCallback(aCallback); } \
  NS_IMETHOD SetCallback(nsISDBCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCallback(aCallback); } 


#endif /* __gen_nsISDBRequest_h__ */
