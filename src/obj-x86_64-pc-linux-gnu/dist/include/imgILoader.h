/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/image/imgILoader.idl
 */

#ifndef __gen_imgILoader_h__
#define __gen_imgILoader_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIRequest_h__
#include "nsIRequest.h"
#endif

#ifndef __gen_nsIContentPolicy_h__
#include "nsIContentPolicy.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class imgINotificationObserver; /* forward declaration */

class imgIRequest; /* forward declaration */

class nsIChannel; /* forward declaration */

class nsILoadGroup; /* forward declaration */

class nsIPrincipal; /* forward declaration */

class nsIStreamListener; /* forward declaration */

class nsIURI; /* forward declaration */

class nsIReferrerInfo; /* forward declaration */

class nsISimpleEnumerator; /* forward declaration */

namespace mozilla {
namespace dom {
class Document; /* webidl Document */
} // namespace dom
} // namespace mozilla


/* starting interface:    imgILoader */
#define IMGILOADER_IID_STR "e61377d2-910e-4c65-a64b-428d150e1fd1"

#define IMGILOADER_IID \
  {0xe61377d2, 0x910e, 0x4c65, \
    { 0xa6, 0x4b, 0x42, 0x8d, 0x15, 0x0e, 0x1f, 0xd1 }}

class NS_NO_VTABLE imgILoader : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(IMGILOADER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = imgILoader;

  enum {
    LOAD_CORS_ANONYMOUS = 65536U,
    LOAD_CORS_USE_CREDENTIALS = 131072U
  };

  /* imgIRequest loadImageXPCOM (in nsIURI aURI, in nsIURI aInitialDocumentURL, in nsIReferrerInfo aReferrerInfo, in nsIPrincipal aLoadingPrincipal, in nsILoadGroup aLoadGroup, in imgINotificationObserver aObserver, in Document aLoadingDocument, in nsLoadFlags aLoadFlags, in nsISupports cacheKey, [optional] in nsContentPolicyType aContentPolicyType); */
  NS_IMETHOD LoadImageXPCOM(nsIURI *aURI, nsIURI *aInitialDocumentURL, nsIReferrerInfo *aReferrerInfo, nsIPrincipal *aLoadingPrincipal, nsILoadGroup *aLoadGroup, imgINotificationObserver *aObserver, mozilla::dom::Document *aLoadingDocument, nsLoadFlags aLoadFlags, nsISupports *cacheKey, nsContentPolicyType aContentPolicyType, imgIRequest **_retval) = 0;

  /* imgIRequest loadImageWithChannelXPCOM (in nsIChannel aChannel, in imgINotificationObserver aObserver, in Document aLoadingDocument, out nsIStreamListener aListener); */
  NS_IMETHOD LoadImageWithChannelXPCOM(nsIChannel *aChannel, imgINotificationObserver *aObserver, mozilla::dom::Document *aLoadingDocument, nsIStreamListener **aListener, imgIRequest **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(imgILoader, IMGILOADER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_IMGILOADER \
  NS_IMETHOD LoadImageXPCOM(nsIURI *aURI, nsIURI *aInitialDocumentURL, nsIReferrerInfo *aReferrerInfo, nsIPrincipal *aLoadingPrincipal, nsILoadGroup *aLoadGroup, imgINotificationObserver *aObserver, mozilla::dom::Document *aLoadingDocument, nsLoadFlags aLoadFlags, nsISupports *cacheKey, nsContentPolicyType aContentPolicyType, imgIRequest **_retval) override; \
  NS_IMETHOD LoadImageWithChannelXPCOM(nsIChannel *aChannel, imgINotificationObserver *aObserver, mozilla::dom::Document *aLoadingDocument, nsIStreamListener **aListener, imgIRequest **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_IMGILOADER \
  nsresult LoadImageXPCOM(nsIURI *aURI, nsIURI *aInitialDocumentURL, nsIReferrerInfo *aReferrerInfo, nsIPrincipal *aLoadingPrincipal, nsILoadGroup *aLoadGroup, imgINotificationObserver *aObserver, mozilla::dom::Document *aLoadingDocument, nsLoadFlags aLoadFlags, nsISupports *cacheKey, nsContentPolicyType aContentPolicyType, imgIRequest **_retval); \
  nsresult LoadImageWithChannelXPCOM(nsIChannel *aChannel, imgINotificationObserver *aObserver, mozilla::dom::Document *aLoadingDocument, nsIStreamListener **aListener, imgIRequest **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_IMGILOADER(_to) \
  NS_IMETHOD LoadImageXPCOM(nsIURI *aURI, nsIURI *aInitialDocumentURL, nsIReferrerInfo *aReferrerInfo, nsIPrincipal *aLoadingPrincipal, nsILoadGroup *aLoadGroup, imgINotificationObserver *aObserver, mozilla::dom::Document *aLoadingDocument, nsLoadFlags aLoadFlags, nsISupports *cacheKey, nsContentPolicyType aContentPolicyType, imgIRequest **_retval) override { return _to LoadImageXPCOM(aURI, aInitialDocumentURL, aReferrerInfo, aLoadingPrincipal, aLoadGroup, aObserver, aLoadingDocument, aLoadFlags, cacheKey, aContentPolicyType, _retval); } \
  NS_IMETHOD LoadImageWithChannelXPCOM(nsIChannel *aChannel, imgINotificationObserver *aObserver, mozilla::dom::Document *aLoadingDocument, nsIStreamListener **aListener, imgIRequest **_retval) override { return _to LoadImageWithChannelXPCOM(aChannel, aObserver, aLoadingDocument, aListener, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_IMGILOADER(_to) \
  NS_IMETHOD LoadImageXPCOM(nsIURI *aURI, nsIURI *aInitialDocumentURL, nsIReferrerInfo *aReferrerInfo, nsIPrincipal *aLoadingPrincipal, nsILoadGroup *aLoadGroup, imgINotificationObserver *aObserver, mozilla::dom::Document *aLoadingDocument, nsLoadFlags aLoadFlags, nsISupports *cacheKey, nsContentPolicyType aContentPolicyType, imgIRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LoadImageXPCOM(aURI, aInitialDocumentURL, aReferrerInfo, aLoadingPrincipal, aLoadGroup, aObserver, aLoadingDocument, aLoadFlags, cacheKey, aContentPolicyType, _retval); } \
  NS_IMETHOD LoadImageWithChannelXPCOM(nsIChannel *aChannel, imgINotificationObserver *aObserver, mozilla::dom::Document *aLoadingDocument, nsIStreamListener **aListener, imgIRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LoadImageWithChannelXPCOM(aChannel, aObserver, aLoadingDocument, aListener, _retval); } 


#endif /* __gen_imgILoader_h__ */
