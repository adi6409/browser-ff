/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/prefetch/nsIPrefetchService.idl
 */

#ifndef __gen_nsIPrefetchService_h__
#define __gen_nsIPrefetchService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIContentPolicy_h__
#include "nsIContentPolicy.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsISimpleEnumerator; /* forward declaration */

class nsIReferrerInfo; /* forward declaration */

class nsINode; /* webidl Node */


/* starting interface:    nsIPrefetchService */
#define NS_IPREFETCHSERVICE_IID_STR "422a1807-4e7f-463d-b8d7-ca2ceb9b5d53"

#define NS_IPREFETCHSERVICE_IID \
  {0x422a1807, 0x4e7f, 0x463d, \
    { 0xb8, 0xd7, 0xca, 0x2c, 0xeb, 0x9b, 0x5d, 0x53 }}

class NS_NO_VTABLE nsIPrefetchService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPREFETCHSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPrefetchService;

  /* void prefetchURI (in nsIURI aURI, in nsIReferrerInfo aReferrerInfo, in Node aSource, in boolean aExplicit); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PrefetchURI(nsIURI *aURI, nsIReferrerInfo *aReferrerInfo, nsINode *aSource, bool aExplicit) = 0;

  /* void preloadURI (in nsIURI aURI, in nsIReferrerInfo aReferrerInfo, in Node aSource, in nsContentPolicyType aPolicyType); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PreloadURI(nsIURI *aURI, nsIReferrerInfo *aReferrerInfo, nsINode *aSource, nsContentPolicyType aPolicyType) = 0;

  /* boolean hasMoreElements (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HasMoreElements(bool *_retval) = 0;

  /* void cancelPrefetchPreloadURI (in nsIURI aURI, in Node aSource); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CancelPrefetchPreloadURI(nsIURI *aURI, nsINode *aSource) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPrefetchService, NS_IPREFETCHSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPREFETCHSERVICE \
  NS_IMETHOD PrefetchURI(nsIURI *aURI, nsIReferrerInfo *aReferrerInfo, nsINode *aSource, bool aExplicit) override; \
  NS_IMETHOD PreloadURI(nsIURI *aURI, nsIReferrerInfo *aReferrerInfo, nsINode *aSource, nsContentPolicyType aPolicyType) override; \
  NS_IMETHOD HasMoreElements(bool *_retval) override; \
  NS_IMETHOD CancelPrefetchPreloadURI(nsIURI *aURI, nsINode *aSource) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPREFETCHSERVICE \
  nsresult PrefetchURI(nsIURI *aURI, nsIReferrerInfo *aReferrerInfo, nsINode *aSource, bool aExplicit); \
  nsresult PreloadURI(nsIURI *aURI, nsIReferrerInfo *aReferrerInfo, nsINode *aSource, nsContentPolicyType aPolicyType); \
  nsresult HasMoreElements(bool *_retval); \
  nsresult CancelPrefetchPreloadURI(nsIURI *aURI, nsINode *aSource); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPREFETCHSERVICE(_to) \
  NS_IMETHOD PrefetchURI(nsIURI *aURI, nsIReferrerInfo *aReferrerInfo, nsINode *aSource, bool aExplicit) override { return _to PrefetchURI(aURI, aReferrerInfo, aSource, aExplicit); } \
  NS_IMETHOD PreloadURI(nsIURI *aURI, nsIReferrerInfo *aReferrerInfo, nsINode *aSource, nsContentPolicyType aPolicyType) override { return _to PreloadURI(aURI, aReferrerInfo, aSource, aPolicyType); } \
  NS_IMETHOD HasMoreElements(bool *_retval) override { return _to HasMoreElements(_retval); } \
  NS_IMETHOD CancelPrefetchPreloadURI(nsIURI *aURI, nsINode *aSource) override { return _to CancelPrefetchPreloadURI(aURI, aSource); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPREFETCHSERVICE(_to) \
  NS_IMETHOD PrefetchURI(nsIURI *aURI, nsIReferrerInfo *aReferrerInfo, nsINode *aSource, bool aExplicit) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PrefetchURI(aURI, aReferrerInfo, aSource, aExplicit); } \
  NS_IMETHOD PreloadURI(nsIURI *aURI, nsIReferrerInfo *aReferrerInfo, nsINode *aSource, nsContentPolicyType aPolicyType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PreloadURI(aURI, aReferrerInfo, aSource, aPolicyType); } \
  NS_IMETHOD HasMoreElements(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasMoreElements(_retval); } \
  NS_IMETHOD CancelPrefetchPreloadURI(nsIURI *aURI, nsINode *aSource) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CancelPrefetchPreloadURI(aURI, aSource); } 


#endif /* __gen_nsIPrefetchService_h__ */
