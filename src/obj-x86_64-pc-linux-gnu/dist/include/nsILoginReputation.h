/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/reputationservice/nsILoginReputation.idl
 */

#ifndef __gen_nsILoginReputation_h__
#define __gen_nsILoginReputation_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

namespace mozilla {
namespace dom {
class HTMLInputElement; /* webidl HTMLInputElement */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsILoginReputationVerdictType */
#define NS_ILOGINREPUTATIONVERDICTTYPE_IID_STR "6219f9da-297e-446d-8d47-ccdd8e72a1d5"

#define NS_ILOGINREPUTATIONVERDICTTYPE_IID \
  {0x6219f9da, 0x297e, 0x446d, \
    { 0x8d, 0x47, 0xcc, 0xdd, 0x8e, 0x72, 0xa1, 0xd5 }}

class NS_NO_VTABLE nsILoginReputationVerdictType : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ILOGINREPUTATIONVERDICTTYPE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsILoginReputationVerdictType;

  enum {
    UNSPECIFIED = 0U,
    SAFE = 1U,
    LOW_REPUTATION = 2U,
    PHISHING = 3U
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsILoginReputationVerdictType, NS_ILOGINREPUTATIONVERDICTTYPE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSILOGINREPUTATIONVERDICTTYPE \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSILOGINREPUTATIONVERDICTTYPE \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSILOGINREPUTATIONVERDICTTYPE(_to) \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSILOGINREPUTATIONVERDICTTYPE(_to) \


/* starting interface:    nsILoginReputationQuery */
#define NS_ILOGINREPUTATIONQUERY_IID_STR "c21ffe59-595f-46c8-9052-fefb639e196e"

#define NS_ILOGINREPUTATIONQUERY_IID \
  {0xc21ffe59, 0x595f, 0x46c8, \
    { 0x90, 0x52, 0xfe, 0xfb, 0x63, 0x9e, 0x19, 0x6e }}

class NS_NO_VTABLE nsILoginReputationQuery : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ILOGINREPUTATIONQUERY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsILoginReputationQuery;

  /* readonly attribute nsIURI formURI; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFormURI(nsIURI **aFormURI) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsILoginReputationQuery, NS_ILOGINREPUTATIONQUERY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSILOGINREPUTATIONQUERY \
  NS_IMETHOD GetFormURI(nsIURI **aFormURI) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSILOGINREPUTATIONQUERY \
  nsresult GetFormURI(nsIURI **aFormURI); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSILOGINREPUTATIONQUERY(_to) \
  NS_IMETHOD GetFormURI(nsIURI **aFormURI) override { return _to GetFormURI(aFormURI); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSILOGINREPUTATIONQUERY(_to) \
  NS_IMETHOD GetFormURI(nsIURI **aFormURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFormURI(aFormURI); } 


/* starting interface:    nsILoginReputationQueryCallback */
#define NS_ILOGINREPUTATIONQUERYCALLBACK_IID_STR "b527be1e-8fbb-41d9-bee4-267a71236368"

#define NS_ILOGINREPUTATIONQUERYCALLBACK_IID \
  {0xb527be1e, 0x8fbb, 0x41d9, \
    { 0xbe, 0xe4, 0x26, 0x7a, 0x71, 0x23, 0x63, 0x68 }}

class NS_NO_VTABLE nsILoginReputationQueryCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ILOGINREPUTATIONQUERYCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsILoginReputationQueryCallback;

  /* void onComplete (in nsresult aStatus, in unsigned long aVerdict); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnComplete(nsresult aStatus, uint32_t aVerdict) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsILoginReputationQueryCallback, NS_ILOGINREPUTATIONQUERYCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSILOGINREPUTATIONQUERYCALLBACK \
  NS_IMETHOD OnComplete(nsresult aStatus, uint32_t aVerdict) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSILOGINREPUTATIONQUERYCALLBACK \
  nsresult OnComplete(nsresult aStatus, uint32_t aVerdict); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSILOGINREPUTATIONQUERYCALLBACK(_to) \
  NS_IMETHOD OnComplete(nsresult aStatus, uint32_t aVerdict) override { return _to OnComplete(aStatus, aVerdict); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSILOGINREPUTATIONQUERYCALLBACK(_to) \
  NS_IMETHOD OnComplete(nsresult aStatus, uint32_t aVerdict) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnComplete(aStatus, aVerdict); } 


/* starting interface:    nsILoginReputationService */
#define NS_ILOGINREPUTATIONSERVICE_IID_STR "1b3f1dfe-ce3a-486b-953e-ce5ac863eff9"

#define NS_ILOGINREPUTATIONSERVICE_IID \
  {0x1b3f1dfe, 0xce3a, 0x486b, \
    { 0x95, 0x3e, 0xce, 0x5a, 0xc8, 0x63, 0xef, 0xf9 }}

class NS_NO_VTABLE nsILoginReputationService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ILOGINREPUTATIONSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsILoginReputationService;

  /* void init (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(void) = 0;

  /* void queryReputationAsync (in HTMLInputElement aInput, in nsILoginReputationQueryCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD QueryReputationAsync(mozilla::dom::HTMLInputElement *aInput, nsILoginReputationQueryCallback *aCallback) = 0;

  /* void queryReputation (in nsILoginReputationQuery aQuery, in nsILoginReputationQueryCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD QueryReputation(nsILoginReputationQuery *aQuery, nsILoginReputationQueryCallback *aCallback) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsILoginReputationService, NS_ILOGINREPUTATIONSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSILOGINREPUTATIONSERVICE \
  NS_IMETHOD Init(void) override; \
  NS_IMETHOD QueryReputationAsync(mozilla::dom::HTMLInputElement *aInput, nsILoginReputationQueryCallback *aCallback) override; \
  NS_IMETHOD QueryReputation(nsILoginReputationQuery *aQuery, nsILoginReputationQueryCallback *aCallback) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSILOGINREPUTATIONSERVICE \
  nsresult Init(void); \
  nsresult QueryReputationAsync(mozilla::dom::HTMLInputElement *aInput, nsILoginReputationQueryCallback *aCallback); \
  nsresult QueryReputation(nsILoginReputationQuery *aQuery, nsILoginReputationQueryCallback *aCallback); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSILOGINREPUTATIONSERVICE(_to) \
  NS_IMETHOD Init(void) override { return _to Init(); } \
  NS_IMETHOD QueryReputationAsync(mozilla::dom::HTMLInputElement *aInput, nsILoginReputationQueryCallback *aCallback) override { return _to QueryReputationAsync(aInput, aCallback); } \
  NS_IMETHOD QueryReputation(nsILoginReputationQuery *aQuery, nsILoginReputationQueryCallback *aCallback) override { return _to QueryReputation(aQuery, aCallback); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSILOGINREPUTATIONSERVICE(_to) \
  NS_IMETHOD Init(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(); } \
  NS_IMETHOD QueryReputationAsync(mozilla::dom::HTMLInputElement *aInput, nsILoginReputationQueryCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->QueryReputationAsync(aInput, aCallback); } \
  NS_IMETHOD QueryReputation(nsILoginReputationQuery *aQuery, nsILoginReputationQueryCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->QueryReputation(aQuery, aCallback); } 


#endif /* __gen_nsILoginReputation_h__ */
