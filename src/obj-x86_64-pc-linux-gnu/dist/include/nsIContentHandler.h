/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/base/nsIContentHandler.idl
 */

#ifndef __gen_nsIContentHandler_h__
#define __gen_nsIContentHandler_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIRequest; /* forward declaration */

class nsIInterfaceRequestor; /* forward declaration */


/* starting interface:    nsIContentHandler */
#define NS_ICONTENTHANDLER_IID_STR "49439df2-b3d2-441c-bf62-866bdaf56fd2"

#define NS_ICONTENTHANDLER_IID \
  {0x49439df2, 0xb3d2, 0x441c, \
    { 0xbf, 0x62, 0x86, 0x6b, 0xda, 0xf5, 0x6f, 0xd2 }}

class NS_NO_VTABLE nsIContentHandler : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICONTENTHANDLER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIContentHandler;

  /* void handleContent (in string aContentType, in nsIInterfaceRequestor aWindowContext, in nsIRequest aRequest); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleContent(const char * aContentType, nsIInterfaceRequestor *aWindowContext, nsIRequest *aRequest) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIContentHandler, NS_ICONTENTHANDLER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICONTENTHANDLER \
  NS_IMETHOD HandleContent(const char * aContentType, nsIInterfaceRequestor *aWindowContext, nsIRequest *aRequest) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICONTENTHANDLER \
  nsresult HandleContent(const char * aContentType, nsIInterfaceRequestor *aWindowContext, nsIRequest *aRequest); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICONTENTHANDLER(_to) \
  NS_IMETHOD HandleContent(const char * aContentType, nsIInterfaceRequestor *aWindowContext, nsIRequest *aRequest) override { return _to HandleContent(aContentType, aWindowContext, aRequest); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICONTENTHANDLER(_to) \
  NS_IMETHOD HandleContent(const char * aContentType, nsIInterfaceRequestor *aWindowContext, nsIRequest *aRequest) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleContent(aContentType, aWindowContext, aRequest); } 


#endif /* __gen_nsIContentHandler_h__ */
