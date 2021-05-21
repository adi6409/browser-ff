/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/exthandler/nsIExternalProtocolService.idl
 */

#ifndef __gen_nsIExternalProtocolService_h__
#define __gen_nsIExternalProtocolService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsIFile; /* forward declaration */

class nsIPrincipal; /* forward declaration */

class nsIInterfaceRequestor; /* forward declaration */

class nsIHandlerInfo; /* forward declaration */

namespace mozilla {
namespace dom {
class BrowsingContext; /* webidl BrowsingContext */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIExternalProtocolService */
#define NS_IEXTERNALPROTOCOLSERVICE_IID_STR "70f93b7a-3ec6-4bcb-b093-92d9984c9f83"

#define NS_IEXTERNALPROTOCOLSERVICE_IID \
  {0x70f93b7a, 0x3ec6, 0x4bcb, \
    { 0xb0, 0x93, 0x92, 0xd9, 0x98, 0x4c, 0x9f, 0x83 }}

class NS_NO_VTABLE nsIExternalProtocolService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IEXTERNALPROTOCOLSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIExternalProtocolService;

  /* boolean externalProtocolHandlerExists (in string aProtocolScheme); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ExternalProtocolHandlerExists(const char * aProtocolScheme, bool *_retval) = 0;

  /* boolean isExposedProtocol (in string aProtocolScheme); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsExposedProtocol(const char * aProtocolScheme, bool *_retval) = 0;

  /* nsIHandlerInfo getProtocolHandlerInfo (in ACString aProtocolScheme); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetProtocolHandlerInfo(const nsACString& aProtocolScheme, nsIHandlerInfo **_retval) = 0;

  /* nsIHandlerInfo getProtocolHandlerInfoFromOS (in ACString aProtocolScheme, out boolean aFound); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetProtocolHandlerInfoFromOS(const nsACString& aProtocolScheme, bool *aFound, nsIHandlerInfo **_retval) = 0;

  /* void setProtocolHandlerDefaults (in nsIHandlerInfo aHandlerInfo, in boolean aOSHandlerExists); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetProtocolHandlerDefaults(nsIHandlerInfo *aHandlerInfo, bool aOSHandlerExists) = 0;

  /* void loadURI (in nsIURI aURI, [optional] in nsIPrincipal aTriggeringPrincipal, [optional] in BrowsingContext aBrowsingContext); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD LoadURI(nsIURI *aURI, nsIPrincipal *aTriggeringPrincipal, mozilla::dom::BrowsingContext *aBrowsingContext) = 0;

  /* AString getApplicationDescription (in AUTF8String aScheme); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetApplicationDescription(const nsACString& aScheme, nsAString& _retval) = 0;

  /* bool isCurrentAppOSDefaultForProtocol (in AUTF8String aScheme); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsCurrentAppOSDefaultForProtocol(const nsACString& aScheme, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIExternalProtocolService, NS_IEXTERNALPROTOCOLSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIEXTERNALPROTOCOLSERVICE \
  NS_IMETHOD ExternalProtocolHandlerExists(const char * aProtocolScheme, bool *_retval) override; \
  NS_IMETHOD IsExposedProtocol(const char * aProtocolScheme, bool *_retval) override; \
  NS_IMETHOD GetProtocolHandlerInfo(const nsACString& aProtocolScheme, nsIHandlerInfo **_retval) override; \
  NS_IMETHOD GetProtocolHandlerInfoFromOS(const nsACString& aProtocolScheme, bool *aFound, nsIHandlerInfo **_retval) override; \
  NS_IMETHOD SetProtocolHandlerDefaults(nsIHandlerInfo *aHandlerInfo, bool aOSHandlerExists) override; \
  NS_IMETHOD LoadURI(nsIURI *aURI, nsIPrincipal *aTriggeringPrincipal, mozilla::dom::BrowsingContext *aBrowsingContext) override; \
  NS_IMETHOD GetApplicationDescription(const nsACString& aScheme, nsAString& _retval) override; \
  NS_IMETHOD IsCurrentAppOSDefaultForProtocol(const nsACString& aScheme, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIEXTERNALPROTOCOLSERVICE \
  nsresult ExternalProtocolHandlerExists(const char * aProtocolScheme, bool *_retval); \
  nsresult IsExposedProtocol(const char * aProtocolScheme, bool *_retval); \
  nsresult GetProtocolHandlerInfo(const nsACString& aProtocolScheme, nsIHandlerInfo **_retval); \
  nsresult GetProtocolHandlerInfoFromOS(const nsACString& aProtocolScheme, bool *aFound, nsIHandlerInfo **_retval); \
  nsresult SetProtocolHandlerDefaults(nsIHandlerInfo *aHandlerInfo, bool aOSHandlerExists); \
  nsresult LoadURI(nsIURI *aURI, nsIPrincipal *aTriggeringPrincipal, mozilla::dom::BrowsingContext *aBrowsingContext); \
  nsresult GetApplicationDescription(const nsACString& aScheme, nsAString& _retval); \
  nsresult IsCurrentAppOSDefaultForProtocol(const nsACString& aScheme, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIEXTERNALPROTOCOLSERVICE(_to) \
  NS_IMETHOD ExternalProtocolHandlerExists(const char * aProtocolScheme, bool *_retval) override { return _to ExternalProtocolHandlerExists(aProtocolScheme, _retval); } \
  NS_IMETHOD IsExposedProtocol(const char * aProtocolScheme, bool *_retval) override { return _to IsExposedProtocol(aProtocolScheme, _retval); } \
  NS_IMETHOD GetProtocolHandlerInfo(const nsACString& aProtocolScheme, nsIHandlerInfo **_retval) override { return _to GetProtocolHandlerInfo(aProtocolScheme, _retval); } \
  NS_IMETHOD GetProtocolHandlerInfoFromOS(const nsACString& aProtocolScheme, bool *aFound, nsIHandlerInfo **_retval) override { return _to GetProtocolHandlerInfoFromOS(aProtocolScheme, aFound, _retval); } \
  NS_IMETHOD SetProtocolHandlerDefaults(nsIHandlerInfo *aHandlerInfo, bool aOSHandlerExists) override { return _to SetProtocolHandlerDefaults(aHandlerInfo, aOSHandlerExists); } \
  NS_IMETHOD LoadURI(nsIURI *aURI, nsIPrincipal *aTriggeringPrincipal, mozilla::dom::BrowsingContext *aBrowsingContext) override { return _to LoadURI(aURI, aTriggeringPrincipal, aBrowsingContext); } \
  NS_IMETHOD GetApplicationDescription(const nsACString& aScheme, nsAString& _retval) override { return _to GetApplicationDescription(aScheme, _retval); } \
  NS_IMETHOD IsCurrentAppOSDefaultForProtocol(const nsACString& aScheme, bool *_retval) override { return _to IsCurrentAppOSDefaultForProtocol(aScheme, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIEXTERNALPROTOCOLSERVICE(_to) \
  NS_IMETHOD ExternalProtocolHandlerExists(const char * aProtocolScheme, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ExternalProtocolHandlerExists(aProtocolScheme, _retval); } \
  NS_IMETHOD IsExposedProtocol(const char * aProtocolScheme, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsExposedProtocol(aProtocolScheme, _retval); } \
  NS_IMETHOD GetProtocolHandlerInfo(const nsACString& aProtocolScheme, nsIHandlerInfo **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProtocolHandlerInfo(aProtocolScheme, _retval); } \
  NS_IMETHOD GetProtocolHandlerInfoFromOS(const nsACString& aProtocolScheme, bool *aFound, nsIHandlerInfo **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProtocolHandlerInfoFromOS(aProtocolScheme, aFound, _retval); } \
  NS_IMETHOD SetProtocolHandlerDefaults(nsIHandlerInfo *aHandlerInfo, bool aOSHandlerExists) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetProtocolHandlerDefaults(aHandlerInfo, aOSHandlerExists); } \
  NS_IMETHOD LoadURI(nsIURI *aURI, nsIPrincipal *aTriggeringPrincipal, mozilla::dom::BrowsingContext *aBrowsingContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LoadURI(aURI, aTriggeringPrincipal, aBrowsingContext); } \
  NS_IMETHOD GetApplicationDescription(const nsACString& aScheme, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetApplicationDescription(aScheme, _retval); } \
  NS_IMETHOD IsCurrentAppOSDefaultForProtocol(const nsACString& aScheme, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsCurrentAppOSDefaultForProtocol(aScheme, _retval); } 


#endif /* __gen_nsIExternalProtocolService_h__ */
