/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/quota/nsIQuotaRequests.idl
 */

#ifndef __gen_nsIQuotaRequests_h__
#define __gen_nsIQuotaRequests_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIPrincipal; /* forward declaration */

class nsIQuotaCallback; /* forward declaration */

class nsIQuotaUsageCallback; /* forward declaration */

class nsIVariant; /* forward declaration */


/* starting interface:    nsIQuotaRequestBase */
#define NS_IQUOTAREQUESTBASE_IID_STR "9af54222-0407-48fd-a4ab-9457c986fc49"

#define NS_IQUOTAREQUESTBASE_IID \
  {0x9af54222, 0x0407, 0x48fd, \
    { 0xa4, 0xab, 0x94, 0x57, 0xc9, 0x86, 0xfc, 0x49 }}

class NS_NO_VTABLE nsIQuotaRequestBase : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IQUOTAREQUESTBASE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIQuotaRequestBase;

  /* readonly attribute nsIPrincipal principal; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) = 0;

  /* [must_use] readonly attribute nsresult resultCode; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetResultCode(nsresult *aResultCode) = 0;

  /* [must_use] readonly attribute ACString resultName; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetResultName(nsACString& aResultName) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIQuotaRequestBase, NS_IQUOTAREQUESTBASE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIQUOTAREQUESTBASE \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override; \
  [[nodiscard]] NS_IMETHOD GetResultCode(nsresult *aResultCode) override; \
  [[nodiscard]] NS_IMETHOD GetResultName(nsACString& aResultName) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIQUOTAREQUESTBASE \
  nsresult GetPrincipal(nsIPrincipal **aPrincipal); \
  [[nodiscard]] nsresult GetResultCode(nsresult *aResultCode); \
  [[nodiscard]] nsresult GetResultName(nsACString& aResultName); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIQUOTAREQUESTBASE(_to) \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override { return _to GetPrincipal(aPrincipal); } \
  [[nodiscard]] NS_IMETHOD GetResultCode(nsresult *aResultCode) override { return _to GetResultCode(aResultCode); } \
  [[nodiscard]] NS_IMETHOD GetResultName(nsACString& aResultName) override { return _to GetResultName(aResultName); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIQUOTAREQUESTBASE(_to) \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrincipal(aPrincipal); } \
  [[nodiscard]] NS_IMETHOD GetResultCode(nsresult *aResultCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResultCode(aResultCode); } \
  [[nodiscard]] NS_IMETHOD GetResultName(nsACString& aResultName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResultName(aResultName); } 


/* starting interface:    nsIQuotaUsageRequest */
#define NS_IQUOTAUSAGEREQUEST_IID_STR "166e28e6-cf6d-4927-a6d7-b51bca9d3469"

#define NS_IQUOTAUSAGEREQUEST_IID \
  {0x166e28e6, 0xcf6d, 0x4927, \
    { 0xa6, 0xd7, 0xb5, 0x1b, 0xca, 0x9d, 0x34, 0x69 }}

class NS_NO_VTABLE nsIQuotaUsageRequest : public nsIQuotaRequestBase {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IQUOTAUSAGEREQUEST_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIQuotaUsageRequest;

  /* [must_use] readonly attribute nsIVariant result; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetResult(nsIVariant **aResult) = 0;

  /* attribute nsIQuotaUsageCallback callback; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCallback(nsIQuotaUsageCallback **aCallback) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCallback(nsIQuotaUsageCallback *aCallback) = 0;

  /* [must_use] void cancel (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD Cancel(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIQuotaUsageRequest, NS_IQUOTAUSAGEREQUEST_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIQUOTAUSAGEREQUEST \
  [[nodiscard]] NS_IMETHOD GetResult(nsIVariant **aResult) override; \
  NS_IMETHOD GetCallback(nsIQuotaUsageCallback **aCallback) override; \
  NS_IMETHOD SetCallback(nsIQuotaUsageCallback *aCallback) override; \
  [[nodiscard]] NS_IMETHOD Cancel(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIQUOTAUSAGEREQUEST \
  [[nodiscard]] nsresult GetResult(nsIVariant **aResult); \
  nsresult GetCallback(nsIQuotaUsageCallback **aCallback); \
  nsresult SetCallback(nsIQuotaUsageCallback *aCallback); \
  [[nodiscard]] nsresult Cancel(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIQUOTAUSAGEREQUEST(_to) \
  [[nodiscard]] NS_IMETHOD GetResult(nsIVariant **aResult) override { return _to GetResult(aResult); } \
  NS_IMETHOD GetCallback(nsIQuotaUsageCallback **aCallback) override { return _to GetCallback(aCallback); } \
  NS_IMETHOD SetCallback(nsIQuotaUsageCallback *aCallback) override { return _to SetCallback(aCallback); } \
  [[nodiscard]] NS_IMETHOD Cancel(void) override { return _to Cancel(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIQUOTAUSAGEREQUEST(_to) \
  [[nodiscard]] NS_IMETHOD GetResult(nsIVariant **aResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResult(aResult); } \
  NS_IMETHOD GetCallback(nsIQuotaUsageCallback **aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCallback(aCallback); } \
  NS_IMETHOD SetCallback(nsIQuotaUsageCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCallback(aCallback); } \
  [[nodiscard]] NS_IMETHOD Cancel(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Cancel(); } 


/* starting interface:    nsIQuotaRequest */
#define NS_IQUOTAREQUEST_IID_STR "22890e3e-ff25-4372-9684-d901060e2f6c"

#define NS_IQUOTAREQUEST_IID \
  {0x22890e3e, 0xff25, 0x4372, \
    { 0x96, 0x84, 0xd9, 0x01, 0x06, 0x0e, 0x2f, 0x6c }}

class NS_NO_VTABLE nsIQuotaRequest : public nsIQuotaRequestBase {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IQUOTAREQUEST_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIQuotaRequest;

  /* [must_use] readonly attribute nsIVariant result; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetResult(nsIVariant **aResult) = 0;

  /* attribute nsIQuotaCallback callback; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCallback(nsIQuotaCallback **aCallback) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCallback(nsIQuotaCallback *aCallback) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIQuotaRequest, NS_IQUOTAREQUEST_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIQUOTAREQUEST \
  [[nodiscard]] NS_IMETHOD GetResult(nsIVariant **aResult) override; \
  NS_IMETHOD GetCallback(nsIQuotaCallback **aCallback) override; \
  NS_IMETHOD SetCallback(nsIQuotaCallback *aCallback) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIQUOTAREQUEST \
  [[nodiscard]] nsresult GetResult(nsIVariant **aResult); \
  nsresult GetCallback(nsIQuotaCallback **aCallback); \
  nsresult SetCallback(nsIQuotaCallback *aCallback); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIQUOTAREQUEST(_to) \
  [[nodiscard]] NS_IMETHOD GetResult(nsIVariant **aResult) override { return _to GetResult(aResult); } \
  NS_IMETHOD GetCallback(nsIQuotaCallback **aCallback) override { return _to GetCallback(aCallback); } \
  NS_IMETHOD SetCallback(nsIQuotaCallback *aCallback) override { return _to SetCallback(aCallback); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIQUOTAREQUEST(_to) \
  [[nodiscard]] NS_IMETHOD GetResult(nsIVariant **aResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResult(aResult); } \
  NS_IMETHOD GetCallback(nsIQuotaCallback **aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCallback(aCallback); } \
  NS_IMETHOD SetCallback(nsIQuotaCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCallback(aCallback); } 


#endif /* __gen_nsIQuotaRequests_h__ */
