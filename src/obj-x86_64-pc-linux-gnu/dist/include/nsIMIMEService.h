/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/mime/nsIMIMEService.idl
 */

#ifndef __gen_nsIMIMEService_h__
#define __gen_nsIMIMEService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIFile; /* forward declaration */

class nsIMIMEInfo; /* forward declaration */

class nsIURI; /* forward declaration */

#define NS_MIMESERVICE_CID                           \
{ /* 03af31da-3109-11d3-8cd0-0060b0fc14a3 */         \
    0x03af31da,                                      \
    0x3109,                                          \
    0x11d3,                                          \
    {0x8c, 0xd0, 0x00, 0x60, 0xb0, 0xfc, 0x14, 0xa3} \
}

/* starting interface:    nsIMIMEService */
#define NS_IMIMESERVICE_IID_STR "5b3675a1-02db-4f8f-a560-b34736635f47"

#define NS_IMIMESERVICE_IID \
  {0x5b3675a1, 0x02db, 0x4f8f, \
    { 0xa5, 0x60, 0xb3, 0x47, 0x36, 0x63, 0x5f, 0x47 }}

class NS_NO_VTABLE nsIMIMEService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IMIMESERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIMIMEService;

  /* nsIMIMEInfo getFromTypeAndExtension (in ACString aMIMEType, in AUTF8String aFileExt); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFromTypeAndExtension(const nsACString& aMIMEType, const nsACString& aFileExt, nsIMIMEInfo **_retval) = 0;

  /* ACString getTypeFromExtension (in AUTF8String aFileExt); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTypeFromExtension(const nsACString& aFileExt, nsACString& _retval) = 0;

  /* ACString getTypeFromURI (in nsIURI aURI); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTypeFromURI(nsIURI *aURI, nsACString& _retval) = 0;

  /* ACString getTypeFromFile (in nsIFile aFile); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTypeFromFile(nsIFile *aFile, nsACString& _retval) = 0;

  /* AUTF8String getPrimaryExtension (in ACString aMIMEType, in AUTF8String aFileExt); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPrimaryExtension(const nsACString& aMIMEType, const nsACString& aFileExt, nsACString& _retval) = 0;

  /* nsIMIMEInfo getMIMEInfoFromOS (in ACString aType, in ACString aFileExtension, out boolean aFound); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMIMEInfoFromOS(const nsACString& aType, const nsACString& aFileExtension, bool *aFound, nsIMIMEInfo **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIMIMEService, NS_IMIMESERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIMIMESERVICE \
  NS_IMETHOD GetFromTypeAndExtension(const nsACString& aMIMEType, const nsACString& aFileExt, nsIMIMEInfo **_retval) override; \
  NS_IMETHOD GetTypeFromExtension(const nsACString& aFileExt, nsACString& _retval) override; \
  NS_IMETHOD GetTypeFromURI(nsIURI *aURI, nsACString& _retval) override; \
  NS_IMETHOD GetTypeFromFile(nsIFile *aFile, nsACString& _retval) override; \
  NS_IMETHOD GetPrimaryExtension(const nsACString& aMIMEType, const nsACString& aFileExt, nsACString& _retval) override; \
  NS_IMETHOD GetMIMEInfoFromOS(const nsACString& aType, const nsACString& aFileExtension, bool *aFound, nsIMIMEInfo **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIMIMESERVICE \
  nsresult GetFromTypeAndExtension(const nsACString& aMIMEType, const nsACString& aFileExt, nsIMIMEInfo **_retval); \
  nsresult GetTypeFromExtension(const nsACString& aFileExt, nsACString& _retval); \
  nsresult GetTypeFromURI(nsIURI *aURI, nsACString& _retval); \
  nsresult GetTypeFromFile(nsIFile *aFile, nsACString& _retval); \
  nsresult GetPrimaryExtension(const nsACString& aMIMEType, const nsACString& aFileExt, nsACString& _retval); \
  nsresult GetMIMEInfoFromOS(const nsACString& aType, const nsACString& aFileExtension, bool *aFound, nsIMIMEInfo **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIMIMESERVICE(_to) \
  NS_IMETHOD GetFromTypeAndExtension(const nsACString& aMIMEType, const nsACString& aFileExt, nsIMIMEInfo **_retval) override { return _to GetFromTypeAndExtension(aMIMEType, aFileExt, _retval); } \
  NS_IMETHOD GetTypeFromExtension(const nsACString& aFileExt, nsACString& _retval) override { return _to GetTypeFromExtension(aFileExt, _retval); } \
  NS_IMETHOD GetTypeFromURI(nsIURI *aURI, nsACString& _retval) override { return _to GetTypeFromURI(aURI, _retval); } \
  NS_IMETHOD GetTypeFromFile(nsIFile *aFile, nsACString& _retval) override { return _to GetTypeFromFile(aFile, _retval); } \
  NS_IMETHOD GetPrimaryExtension(const nsACString& aMIMEType, const nsACString& aFileExt, nsACString& _retval) override { return _to GetPrimaryExtension(aMIMEType, aFileExt, _retval); } \
  NS_IMETHOD GetMIMEInfoFromOS(const nsACString& aType, const nsACString& aFileExtension, bool *aFound, nsIMIMEInfo **_retval) override { return _to GetMIMEInfoFromOS(aType, aFileExtension, aFound, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIMIMESERVICE(_to) \
  NS_IMETHOD GetFromTypeAndExtension(const nsACString& aMIMEType, const nsACString& aFileExt, nsIMIMEInfo **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFromTypeAndExtension(aMIMEType, aFileExt, _retval); } \
  NS_IMETHOD GetTypeFromExtension(const nsACString& aFileExt, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTypeFromExtension(aFileExt, _retval); } \
  NS_IMETHOD GetTypeFromURI(nsIURI *aURI, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTypeFromURI(aURI, _retval); } \
  NS_IMETHOD GetTypeFromFile(nsIFile *aFile, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTypeFromFile(aFile, _retval); } \
  NS_IMETHOD GetPrimaryExtension(const nsACString& aMIMEType, const nsACString& aFileExt, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrimaryExtension(aMIMEType, aFileExt, _retval); } \
  NS_IMETHOD GetMIMEInfoFromOS(const nsACString& aType, const nsACString& aFileExtension, bool *aFound, nsIMIMEInfo **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMIMEInfoFromOS(aType, aFileExtension, aFound, _retval); } 


#endif /* __gen_nsIMIMEService_h__ */
