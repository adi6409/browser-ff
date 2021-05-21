/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/places/nsITaggingService.idl
 */

#ifndef __gen_nsITaggingService_h__
#define __gen_nsITaggingService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsIVariant; /* forward declaration */


/* starting interface:    nsITaggingService */
#define NS_ITAGGINGSERVICE_IID_STR "9759bd0e-78e2-4421-9ed1-c676e1af3513"

#define NS_ITAGGINGSERVICE_IID \
  {0x9759bd0e, 0x78e2, 0x4421, \
    { 0x9e, 0xd1, 0xc6, 0x76, 0xe1, 0xaf, 0x35, 0x13 }}

class NS_NO_VTABLE nsITaggingService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITAGGINGSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITaggingService;

  /* void tagURI (in nsIURI aURI, in nsIVariant aTags, [optional] in unsigned short aSource); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD TagURI(nsIURI *aURI, nsIVariant *aTags, uint16_t aSource) = 0;

  /* void untagURI (in nsIURI aURI, in nsIVariant aTags, [optional] in unsigned short aSource); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UntagURI(nsIURI *aURI, nsIVariant *aTags, uint16_t aSource) = 0;

  /* Array<AString> getTagsForURI (in nsIURI aURI); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTagsForURI(nsIURI *aURI, nsTArray<nsString >& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITaggingService, NS_ITAGGINGSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITAGGINGSERVICE \
  NS_IMETHOD TagURI(nsIURI *aURI, nsIVariant *aTags, uint16_t aSource) override; \
  NS_IMETHOD UntagURI(nsIURI *aURI, nsIVariant *aTags, uint16_t aSource) override; \
  NS_IMETHOD GetTagsForURI(nsIURI *aURI, nsTArray<nsString >& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITAGGINGSERVICE \
  nsresult TagURI(nsIURI *aURI, nsIVariant *aTags, uint16_t aSource); \
  nsresult UntagURI(nsIURI *aURI, nsIVariant *aTags, uint16_t aSource); \
  nsresult GetTagsForURI(nsIURI *aURI, nsTArray<nsString >& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITAGGINGSERVICE(_to) \
  NS_IMETHOD TagURI(nsIURI *aURI, nsIVariant *aTags, uint16_t aSource) override { return _to TagURI(aURI, aTags, aSource); } \
  NS_IMETHOD UntagURI(nsIURI *aURI, nsIVariant *aTags, uint16_t aSource) override { return _to UntagURI(aURI, aTags, aSource); } \
  NS_IMETHOD GetTagsForURI(nsIURI *aURI, nsTArray<nsString >& _retval) override { return _to GetTagsForURI(aURI, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITAGGINGSERVICE(_to) \
  NS_IMETHOD TagURI(nsIURI *aURI, nsIVariant *aTags, uint16_t aSource) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TagURI(aURI, aTags, aSource); } \
  NS_IMETHOD UntagURI(nsIURI *aURI, nsIVariant *aTags, uint16_t aSource) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UntagURI(aURI, aTags, aSource); } \
  NS_IMETHOD GetTagsForURI(nsIURI *aURI, nsTArray<nsString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTagsForURI(aURI, _retval); } 


#define TAGGING_SERVICE_CID "@mozilla.org/browser/tagging-service;1"

#endif /* __gen_nsITaggingService_h__ */
