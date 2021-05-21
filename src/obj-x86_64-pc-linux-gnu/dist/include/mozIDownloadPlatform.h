/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/downloads/mozIDownloadPlatform.idl
 */

#ifndef __gen_mozIDownloadPlatform_h__
#define __gen_mozIDownloadPlatform_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsIFile; /* forward declaration */


/* starting interface:    mozIDownloadPlatform */
#define MOZIDOWNLOADPLATFORM_IID_STR "9f556e4a-d9b3-46c3-9f8f-d0db1ac6c8c1"

#define MOZIDOWNLOADPLATFORM_IID \
  {0x9f556e4a, 0xd9b3, 0x46c3, \
    { 0x9f, 0x8f, 0xd0, 0xdb, 0x1a, 0xc6, 0xc8, 0xc1 }}

class NS_NO_VTABLE mozIDownloadPlatform : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZIDOWNLOADPLATFORM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIDownloadPlatform;

  /* [implicit_jscontext] Promise downloadDone (in nsIURI aSource, in nsIURI aReferrer, in nsIFile aTarget, in ACString aContentType, in boolean aIsPrivate); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DownloadDone(nsIURI *aSource, nsIURI *aReferrer, nsIFile *aTarget, const nsACString& aContentType, bool aIsPrivate, JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  enum {
    ZONE_MY_COMPUTER = 0U,
    ZONE_INTRANET = 1U,
    ZONE_TRUSTED = 2U,
    ZONE_INTERNET = 3U,
    ZONE_RESTRICTED = 4U
  };

  /* unsigned long mapUrlToZone (in AString aURL); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MapUrlToZone(const nsAString& aURL, uint32_t *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIDownloadPlatform, MOZIDOWNLOADPLATFORM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZIDOWNLOADPLATFORM \
  NS_IMETHOD DownloadDone(nsIURI *aSource, nsIURI *aReferrer, nsIFile *aTarget, const nsACString& aContentType, bool aIsPrivate, JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD MapUrlToZone(const nsAString& aURL, uint32_t *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZIDOWNLOADPLATFORM \
  nsresult DownloadDone(nsIURI *aSource, nsIURI *aReferrer, nsIFile *aTarget, const nsACString& aContentType, bool aIsPrivate, JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  nsresult MapUrlToZone(const nsAString& aURL, uint32_t *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZIDOWNLOADPLATFORM(_to) \
  NS_IMETHOD DownloadDone(nsIURI *aSource, nsIURI *aReferrer, nsIFile *aTarget, const nsACString& aContentType, bool aIsPrivate, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to DownloadDone(aSource, aReferrer, aTarget, aContentType, aIsPrivate, cx, _retval); } \
  NS_IMETHOD MapUrlToZone(const nsAString& aURL, uint32_t *_retval) override { return _to MapUrlToZone(aURL, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZIDOWNLOADPLATFORM(_to) \
  NS_IMETHOD DownloadDone(nsIURI *aSource, nsIURI *aReferrer, nsIFile *aTarget, const nsACString& aContentType, bool aIsPrivate, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DownloadDone(aSource, aReferrer, aTarget, aContentType, aIsPrivate, cx, _retval); } \
  NS_IMETHOD MapUrlToZone(const nsAString& aURL, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MapUrlToZone(aURL, _retval); } 


#endif /* __gen_mozIDownloadPlatform_h__ */
