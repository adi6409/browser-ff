/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/exthandler/nsIExternalURLHandlerService.idl
 */

#ifndef __gen_nsIExternalURLHandlerService_h__
#define __gen_nsIExternalURLHandlerService_h__


#ifndef __gen_nsIMIMEInfo_h__
#include "nsIMIMEInfo.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIExternalURLHandlerService */
#define NS_IEXTERNALURLHANDLERSERVICE_IID_STR "56c5c7d3-6fd3-43f8-9429-4397e111453a"

#define NS_IEXTERNALURLHANDLERSERVICE_IID \
  {0x56c5c7d3, 0x6fd3, 0x43f8, \
    { 0x94, 0x29, 0x43, 0x97, 0xe1, 0x11, 0x45, 0x3a }}

class NS_NO_VTABLE nsIExternalURLHandlerService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IEXTERNALURLHANDLERSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIExternalURLHandlerService;

  /* nsIHandlerInfo getURLHandlerInfoFromOS (in nsIURI aURL, out boolean aFound); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetURLHandlerInfoFromOS(nsIURI *aURL, bool *aFound, nsIHandlerInfo **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIExternalURLHandlerService, NS_IEXTERNALURLHANDLERSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIEXTERNALURLHANDLERSERVICE \
  NS_IMETHOD GetURLHandlerInfoFromOS(nsIURI *aURL, bool *aFound, nsIHandlerInfo **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIEXTERNALURLHANDLERSERVICE \
  nsresult GetURLHandlerInfoFromOS(nsIURI *aURL, bool *aFound, nsIHandlerInfo **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIEXTERNALURLHANDLERSERVICE(_to) \
  NS_IMETHOD GetURLHandlerInfoFromOS(nsIURI *aURL, bool *aFound, nsIHandlerInfo **_retval) override { return _to GetURLHandlerInfoFromOS(aURL, aFound, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIEXTERNALURLHANDLERSERVICE(_to) \
  NS_IMETHOD GetURLHandlerInfoFromOS(nsIURI *aURL, bool *aFound, nsIHandlerInfo **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetURLHandlerInfoFromOS(aURL, aFound, _retval); } 


#endif /* __gen_nsIExternalURLHandlerService_h__ */
