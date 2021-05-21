/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/browser/components/newtab/nsIAboutNewTabService.idl
 */

#ifndef __gen_nsIAboutNewTabService_h__
#define __gen_nsIAboutNewTabService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIChannel; /* forward declaration */

class nsIURI; /* forward declaration */

class nsILoadInfo; /* forward declaration */


/* starting interface:    nsIAboutNewTabService */
#define NS_IABOUTNEWTABSERVICE_IID_STR "dfcd2adc-7867-4d3a-ba70-17501f208142"

#define NS_IABOUTNEWTABSERVICE_IID \
  {0xdfcd2adc, 0x7867, 0x4d3a, \
    { 0xba, 0x70, 0x17, 0x50, 0x1f, 0x20, 0x81, 0x42 }}

class NS_NO_VTABLE nsIAboutNewTabService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IABOUTNEWTABSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAboutNewTabService;

  /* readonly attribute ACString defaultURL; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDefaultURL(nsACString& aDefaultURL) = 0;

  /* nsIChannel aboutHomeChannel (in nsIURI aURI, in nsILoadInfo aLoadInfo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AboutHomeChannel(nsIURI *aURI, nsILoadInfo *aLoadInfo, nsIChannel **_retval) = 0;

  /* readonly attribute ACString welcomeURL; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetWelcomeURL(nsACString& aWelcomeURL) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAboutNewTabService, NS_IABOUTNEWTABSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIABOUTNEWTABSERVICE \
  NS_IMETHOD GetDefaultURL(nsACString& aDefaultURL) override; \
  NS_IMETHOD AboutHomeChannel(nsIURI *aURI, nsILoadInfo *aLoadInfo, nsIChannel **_retval) override; \
  NS_IMETHOD GetWelcomeURL(nsACString& aWelcomeURL) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIABOUTNEWTABSERVICE \
  nsresult GetDefaultURL(nsACString& aDefaultURL); \
  nsresult AboutHomeChannel(nsIURI *aURI, nsILoadInfo *aLoadInfo, nsIChannel **_retval); \
  nsresult GetWelcomeURL(nsACString& aWelcomeURL); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIABOUTNEWTABSERVICE(_to) \
  NS_IMETHOD GetDefaultURL(nsACString& aDefaultURL) override { return _to GetDefaultURL(aDefaultURL); } \
  NS_IMETHOD AboutHomeChannel(nsIURI *aURI, nsILoadInfo *aLoadInfo, nsIChannel **_retval) override { return _to AboutHomeChannel(aURI, aLoadInfo, _retval); } \
  NS_IMETHOD GetWelcomeURL(nsACString& aWelcomeURL) override { return _to GetWelcomeURL(aWelcomeURL); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIABOUTNEWTABSERVICE(_to) \
  NS_IMETHOD GetDefaultURL(nsACString& aDefaultURL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDefaultURL(aDefaultURL); } \
  NS_IMETHOD AboutHomeChannel(nsIURI *aURI, nsILoadInfo *aLoadInfo, nsIChannel **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AboutHomeChannel(aURI, aLoadInfo, _retval); } \
  NS_IMETHOD GetWelcomeURL(nsACString& aWelcomeURL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWelcomeURL(aWelcomeURL); } 


#endif /* __gen_nsIAboutNewTabService_h__ */
