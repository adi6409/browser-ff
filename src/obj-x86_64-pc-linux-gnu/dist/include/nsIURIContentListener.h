/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/base/nsIURIContentListener.idl
 */

#ifndef __gen_nsIURIContentListener_h__
#define __gen_nsIURIContentListener_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIRequest; /* forward declaration */

class nsIStreamListener; /* forward declaration */

class nsIURI; /* forward declaration */


/* starting interface:    nsIURIContentListener */
#define NS_IURICONTENTLISTENER_IID_STR "10a28f38-32e8-4c63-8aa1-12eaaebc369a"

#define NS_IURICONTENTLISTENER_IID \
  {0x10a28f38, 0x32e8, 0x4c63, \
    { 0x8a, 0xa1, 0x12, 0xea, 0xae, 0xbc, 0x36, 0x9a }}

class NS_NO_VTABLE nsIURIContentListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURICONTENTLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIURIContentListener;

  /* boolean doContent (in ACString aContentType, in boolean aIsContentPreferred, in nsIRequest aRequest, out nsIStreamListener aContentHandler); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DoContent(const nsACString& aContentType, bool aIsContentPreferred, nsIRequest *aRequest, nsIStreamListener **aContentHandler, bool *_retval) = 0;

  /* boolean isPreferred (in string aContentType, out string aDesiredContentType); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsPreferred(const char * aContentType, char * *aDesiredContentType, bool *_retval) = 0;

  /* boolean canHandleContent (in string aContentType, in boolean aIsContentPreferred, out string aDesiredContentType); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CanHandleContent(const char * aContentType, bool aIsContentPreferred, char * *aDesiredContentType, bool *_retval) = 0;

  /* attribute nsISupports loadCookie; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLoadCookie(nsISupports **aLoadCookie) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetLoadCookie(nsISupports *aLoadCookie) = 0;

  /* attribute nsIURIContentListener parentContentListener; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetParentContentListener(nsIURIContentListener **aParentContentListener) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetParentContentListener(nsIURIContentListener *aParentContentListener) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIURIContentListener, NS_IURICONTENTLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURICONTENTLISTENER \
  NS_IMETHOD DoContent(const nsACString& aContentType, bool aIsContentPreferred, nsIRequest *aRequest, nsIStreamListener **aContentHandler, bool *_retval) override; \
  NS_IMETHOD IsPreferred(const char * aContentType, char * *aDesiredContentType, bool *_retval) override; \
  NS_IMETHOD CanHandleContent(const char * aContentType, bool aIsContentPreferred, char * *aDesiredContentType, bool *_retval) override; \
  NS_IMETHOD GetLoadCookie(nsISupports **aLoadCookie) override; \
  NS_IMETHOD SetLoadCookie(nsISupports *aLoadCookie) override; \
  NS_IMETHOD GetParentContentListener(nsIURIContentListener **aParentContentListener) override; \
  NS_IMETHOD SetParentContentListener(nsIURIContentListener *aParentContentListener) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURICONTENTLISTENER \
  nsresult DoContent(const nsACString& aContentType, bool aIsContentPreferred, nsIRequest *aRequest, nsIStreamListener **aContentHandler, bool *_retval); \
  nsresult IsPreferred(const char * aContentType, char * *aDesiredContentType, bool *_retval); \
  nsresult CanHandleContent(const char * aContentType, bool aIsContentPreferred, char * *aDesiredContentType, bool *_retval); \
  nsresult GetLoadCookie(nsISupports **aLoadCookie); \
  nsresult SetLoadCookie(nsISupports *aLoadCookie); \
  nsresult GetParentContentListener(nsIURIContentListener **aParentContentListener); \
  nsresult SetParentContentListener(nsIURIContentListener *aParentContentListener); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURICONTENTLISTENER(_to) \
  NS_IMETHOD DoContent(const nsACString& aContentType, bool aIsContentPreferred, nsIRequest *aRequest, nsIStreamListener **aContentHandler, bool *_retval) override { return _to DoContent(aContentType, aIsContentPreferred, aRequest, aContentHandler, _retval); } \
  NS_IMETHOD IsPreferred(const char * aContentType, char * *aDesiredContentType, bool *_retval) override { return _to IsPreferred(aContentType, aDesiredContentType, _retval); } \
  NS_IMETHOD CanHandleContent(const char * aContentType, bool aIsContentPreferred, char * *aDesiredContentType, bool *_retval) override { return _to CanHandleContent(aContentType, aIsContentPreferred, aDesiredContentType, _retval); } \
  NS_IMETHOD GetLoadCookie(nsISupports **aLoadCookie) override { return _to GetLoadCookie(aLoadCookie); } \
  NS_IMETHOD SetLoadCookie(nsISupports *aLoadCookie) override { return _to SetLoadCookie(aLoadCookie); } \
  NS_IMETHOD GetParentContentListener(nsIURIContentListener **aParentContentListener) override { return _to GetParentContentListener(aParentContentListener); } \
  NS_IMETHOD SetParentContentListener(nsIURIContentListener *aParentContentListener) override { return _to SetParentContentListener(aParentContentListener); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURICONTENTLISTENER(_to) \
  NS_IMETHOD DoContent(const nsACString& aContentType, bool aIsContentPreferred, nsIRequest *aRequest, nsIStreamListener **aContentHandler, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DoContent(aContentType, aIsContentPreferred, aRequest, aContentHandler, _retval); } \
  NS_IMETHOD IsPreferred(const char * aContentType, char * *aDesiredContentType, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsPreferred(aContentType, aDesiredContentType, _retval); } \
  NS_IMETHOD CanHandleContent(const char * aContentType, bool aIsContentPreferred, char * *aDesiredContentType, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CanHandleContent(aContentType, aIsContentPreferred, aDesiredContentType, _retval); } \
  NS_IMETHOD GetLoadCookie(nsISupports **aLoadCookie) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadCookie(aLoadCookie); } \
  NS_IMETHOD SetLoadCookie(nsISupports *aLoadCookie) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLoadCookie(aLoadCookie); } \
  NS_IMETHOD GetParentContentListener(nsIURIContentListener **aParentContentListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetParentContentListener(aParentContentListener); } \
  NS_IMETHOD SetParentContentListener(nsIURIContentListener *aParentContentListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetParentContentListener(aParentContentListener); } 


#endif /* __gen_nsIURIContentListener_h__ */
