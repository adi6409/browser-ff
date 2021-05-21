/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/exthandler/nsIHandlerService.idl
 */

#ifndef __gen_nsIHandlerService_h__
#define __gen_nsIHandlerService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIHandlerInfo; /* forward declaration */

class nsISimpleEnumerator; /* forward declaration */


/* starting interface:    nsIHandlerService */
#define NS_IHANDLERSERVICE_IID_STR "53f0ad17-ec62-46a1-adbc-efccc06babcd"

#define NS_IHANDLERSERVICE_IID \
  {0x53f0ad17, 0xec62, 0x46a1, \
    { 0xad, 0xbc, 0xef, 0xcc, 0xc0, 0x6b, 0xab, 0xcd }}

class NS_NO_VTABLE nsIHandlerService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHANDLERSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIHandlerService;

  /* void asyncInit (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncInit(void) = 0;

  /* nsISimpleEnumerator enumerate (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Enumerate(nsISimpleEnumerator **_retval) = 0;

  /* void fillHandlerInfo (in nsIHandlerInfo aHandlerInfo, in ACString aOverrideType); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FillHandlerInfo(nsIHandlerInfo *aHandlerInfo, const nsACString& aOverrideType) = 0;

  /* void store (in nsIHandlerInfo aHandlerInfo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Store(nsIHandlerInfo *aHandlerInfo) = 0;

  /* boolean exists (in nsIHandlerInfo aHandlerInfo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Exists(nsIHandlerInfo *aHandlerInfo, bool *_retval) = 0;

  /* void remove (in nsIHandlerInfo aHandlerInfo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Remove(nsIHandlerInfo *aHandlerInfo) = 0;

  /* ACString getTypeFromExtension (in ACString aFileExtension); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTypeFromExtension(const nsACString& aFileExtension, nsACString& _retval) = 0;

  /* boolean existsForProtocolOS (in ACString aProtocolScheme); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ExistsForProtocolOS(const nsACString& aProtocolScheme, bool *_retval) = 0;

  /* boolean existsForProtocol (in ACString aProtocolScheme); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ExistsForProtocol(const nsACString& aProtocolScheme, bool *_retval) = 0;

  /* void getMIMEInfoFromOS (in nsIHandlerInfo aHandlerInfo, in ACString aMIMEType, in ACString aExtension, out bool aFound); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMIMEInfoFromOS(nsIHandlerInfo *aHandlerInfo, const nsACString& aMIMEType, const nsACString& aExtension, bool *aFound) = 0;

  /* AString getApplicationDescription (in ACString aProtocolScheme); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetApplicationDescription(const nsACString& aProtocolScheme, nsAString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHandlerService, NS_IHANDLERSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHANDLERSERVICE \
  NS_IMETHOD AsyncInit(void) override; \
  NS_IMETHOD Enumerate(nsISimpleEnumerator **_retval) override; \
  NS_IMETHOD FillHandlerInfo(nsIHandlerInfo *aHandlerInfo, const nsACString& aOverrideType) override; \
  NS_IMETHOD Store(nsIHandlerInfo *aHandlerInfo) override; \
  NS_IMETHOD Exists(nsIHandlerInfo *aHandlerInfo, bool *_retval) override; \
  NS_IMETHOD Remove(nsIHandlerInfo *aHandlerInfo) override; \
  NS_IMETHOD GetTypeFromExtension(const nsACString& aFileExtension, nsACString& _retval) override; \
  NS_IMETHOD ExistsForProtocolOS(const nsACString& aProtocolScheme, bool *_retval) override; \
  NS_IMETHOD ExistsForProtocol(const nsACString& aProtocolScheme, bool *_retval) override; \
  NS_IMETHOD GetMIMEInfoFromOS(nsIHandlerInfo *aHandlerInfo, const nsACString& aMIMEType, const nsACString& aExtension, bool *aFound) override; \
  NS_IMETHOD GetApplicationDescription(const nsACString& aProtocolScheme, nsAString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHANDLERSERVICE \
  nsresult AsyncInit(void); \
  nsresult Enumerate(nsISimpleEnumerator **_retval); \
  nsresult FillHandlerInfo(nsIHandlerInfo *aHandlerInfo, const nsACString& aOverrideType); \
  nsresult Store(nsIHandlerInfo *aHandlerInfo); \
  nsresult Exists(nsIHandlerInfo *aHandlerInfo, bool *_retval); \
  nsresult Remove(nsIHandlerInfo *aHandlerInfo); \
  nsresult GetTypeFromExtension(const nsACString& aFileExtension, nsACString& _retval); \
  nsresult ExistsForProtocolOS(const nsACString& aProtocolScheme, bool *_retval); \
  nsresult ExistsForProtocol(const nsACString& aProtocolScheme, bool *_retval); \
  nsresult GetMIMEInfoFromOS(nsIHandlerInfo *aHandlerInfo, const nsACString& aMIMEType, const nsACString& aExtension, bool *aFound); \
  nsresult GetApplicationDescription(const nsACString& aProtocolScheme, nsAString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHANDLERSERVICE(_to) \
  NS_IMETHOD AsyncInit(void) override { return _to AsyncInit(); } \
  NS_IMETHOD Enumerate(nsISimpleEnumerator **_retval) override { return _to Enumerate(_retval); } \
  NS_IMETHOD FillHandlerInfo(nsIHandlerInfo *aHandlerInfo, const nsACString& aOverrideType) override { return _to FillHandlerInfo(aHandlerInfo, aOverrideType); } \
  NS_IMETHOD Store(nsIHandlerInfo *aHandlerInfo) override { return _to Store(aHandlerInfo); } \
  NS_IMETHOD Exists(nsIHandlerInfo *aHandlerInfo, bool *_retval) override { return _to Exists(aHandlerInfo, _retval); } \
  NS_IMETHOD Remove(nsIHandlerInfo *aHandlerInfo) override { return _to Remove(aHandlerInfo); } \
  NS_IMETHOD GetTypeFromExtension(const nsACString& aFileExtension, nsACString& _retval) override { return _to GetTypeFromExtension(aFileExtension, _retval); } \
  NS_IMETHOD ExistsForProtocolOS(const nsACString& aProtocolScheme, bool *_retval) override { return _to ExistsForProtocolOS(aProtocolScheme, _retval); } \
  NS_IMETHOD ExistsForProtocol(const nsACString& aProtocolScheme, bool *_retval) override { return _to ExistsForProtocol(aProtocolScheme, _retval); } \
  NS_IMETHOD GetMIMEInfoFromOS(nsIHandlerInfo *aHandlerInfo, const nsACString& aMIMEType, const nsACString& aExtension, bool *aFound) override { return _to GetMIMEInfoFromOS(aHandlerInfo, aMIMEType, aExtension, aFound); } \
  NS_IMETHOD GetApplicationDescription(const nsACString& aProtocolScheme, nsAString& _retval) override { return _to GetApplicationDescription(aProtocolScheme, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHANDLERSERVICE(_to) \
  NS_IMETHOD AsyncInit(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncInit(); } \
  NS_IMETHOD Enumerate(nsISimpleEnumerator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Enumerate(_retval); } \
  NS_IMETHOD FillHandlerInfo(nsIHandlerInfo *aHandlerInfo, const nsACString& aOverrideType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FillHandlerInfo(aHandlerInfo, aOverrideType); } \
  NS_IMETHOD Store(nsIHandlerInfo *aHandlerInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Store(aHandlerInfo); } \
  NS_IMETHOD Exists(nsIHandlerInfo *aHandlerInfo, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Exists(aHandlerInfo, _retval); } \
  NS_IMETHOD Remove(nsIHandlerInfo *aHandlerInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Remove(aHandlerInfo); } \
  NS_IMETHOD GetTypeFromExtension(const nsACString& aFileExtension, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTypeFromExtension(aFileExtension, _retval); } \
  NS_IMETHOD ExistsForProtocolOS(const nsACString& aProtocolScheme, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ExistsForProtocolOS(aProtocolScheme, _retval); } \
  NS_IMETHOD ExistsForProtocol(const nsACString& aProtocolScheme, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ExistsForProtocol(aProtocolScheme, _retval); } \
  NS_IMETHOD GetMIMEInfoFromOS(nsIHandlerInfo *aHandlerInfo, const nsACString& aMIMEType, const nsACString& aExtension, bool *aFound) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMIMEInfoFromOS(aHandlerInfo, aMIMEType, aExtension, aFound); } \
  NS_IMETHOD GetApplicationDescription(const nsACString& aProtocolScheme, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetApplicationDescription(aProtocolScheme, _retval); } 


#endif /* __gen_nsIHandlerService_h__ */
