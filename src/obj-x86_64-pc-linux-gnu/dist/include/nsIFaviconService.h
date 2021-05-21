/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/places/nsIFaviconService.idl
 */

#ifndef __gen_nsIFaviconService_h__
#define __gen_nsIFaviconService_h__


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

class nsIPrincipal; /* forward declaration */

class mozIPlacesPendingOperation; /* forward declaration */

class nsIFaviconDataCallback; /* forward declaration */


/* starting interface:    nsIFaviconService */
#define NS_IFAVICONSERVICE_IID_STR "e81e0b0c-b9f1-4c2e-8f3c-b809933cf73c"

#define NS_IFAVICONSERVICE_IID \
  {0xe81e0b0c, 0xb9f1, 0x4c2e, \
    { 0x8f, 0x3c, 0xb8, 0x09, 0x93, 0x3c, 0xf7, 0x3c }}

class NS_NO_VTABLE nsIFaviconService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFAVICONSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFaviconService;

  enum {
    FAVICON_LOAD_PRIVATE = 1U,
    FAVICON_LOAD_NON_PRIVATE = 2U,
    MAX_FAVICON_BUFFER_SIZE = 65536U
  };

  /* nsIURI getFaviconLinkForIcon (in nsIURI aFaviconURI); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFaviconLinkForIcon(nsIURI *aFaviconURI, nsIURI **_retval) = 0;

  /* void expireAllFavicons (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ExpireAllFavicons(void) = 0;

  /* void setDefaultIconURIPreferredSize (in unsigned short aDefaultSize); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDefaultIconURIPreferredSize(uint16_t aDefaultSize) = 0;

  /* unsigned short preferredSizeFromURI (in nsIURI aURI); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PreferredSizeFromURI(nsIURI *aURI, uint16_t *_retval) = 0;

  /* readonly attribute nsIURI defaultFavicon; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDefaultFavicon(nsIURI **aDefaultFavicon) = 0;

  /* readonly attribute AUTF8String defaultFaviconMimeType; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDefaultFaviconMimeType(nsACString& aDefaultFaviconMimeType) = 0;

  /* mozIPlacesPendingOperation setAndFetchFaviconForPage (in nsIURI aPageURI, in nsIURI aFaviconURI, in boolean aForceReload, in unsigned long aFaviconLoadType, [optional] in nsIFaviconDataCallback aCallback, [optional] in nsIPrincipal aLoadingPrincipal, [optional] in unsigned long long aRequestContextID); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetAndFetchFaviconForPage(nsIURI *aPageURI, nsIURI *aFaviconURI, bool aForceReload, uint32_t aFaviconLoadType, nsIFaviconDataCallback *aCallback, nsIPrincipal *aLoadingPrincipal, uint64_t aRequestContextID, mozIPlacesPendingOperation **_retval) = 0;

  /* void replaceFaviconData (in nsIURI aFaviconURI, in Array<octet> aData, in AUTF8String aMimeType, [optional] in PRTime aExpiration); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ReplaceFaviconData(nsIURI *aFaviconURI, const nsTArray<uint8_t >& aData, const nsACString& aMimeType, PRTime aExpiration) = 0;

  /* void replaceFaviconDataFromDataURL (in nsIURI aFaviconURI, in AString aDataURL, [optional] in PRTime aExpiration, [optional] in nsIPrincipal aLoadingPrincipal); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ReplaceFaviconDataFromDataURL(nsIURI *aFaviconURI, const nsAString& aDataURL, PRTime aExpiration, nsIPrincipal *aLoadingPrincipal) = 0;

  /* void getFaviconURLForPage (in nsIURI aPageURI, in nsIFaviconDataCallback aCallback, [optional] in unsigned short aPreferredWidth); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFaviconURLForPage(nsIURI *aPageURI, nsIFaviconDataCallback *aCallback, uint16_t aPreferredWidth) = 0;

  /* void getFaviconDataForPage (in nsIURI aPageURI, in nsIFaviconDataCallback aCallback, [optional] in unsigned short aPreferredWidth); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFaviconDataForPage(nsIURI *aPageURI, nsIFaviconDataCallback *aCallback, uint16_t aPreferredWidth) = 0;

  /* void copyFavicons (in nsIURI aFromPageURI, in nsIURI aToPageURI, in unsigned long aFaviconLoadType, [optional] in nsIFaviconDataCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CopyFavicons(nsIURI *aFromPageURI, nsIURI *aToPageURI, uint32_t aFaviconLoadType, nsIFaviconDataCallback *aCallback) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFaviconService, NS_IFAVICONSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFAVICONSERVICE \
  NS_IMETHOD GetFaviconLinkForIcon(nsIURI *aFaviconURI, nsIURI **_retval) override; \
  NS_IMETHOD ExpireAllFavicons(void) override; \
  NS_IMETHOD SetDefaultIconURIPreferredSize(uint16_t aDefaultSize) override; \
  NS_IMETHOD PreferredSizeFromURI(nsIURI *aURI, uint16_t *_retval) override; \
  NS_IMETHOD GetDefaultFavicon(nsIURI **aDefaultFavicon) override; \
  NS_IMETHOD GetDefaultFaviconMimeType(nsACString& aDefaultFaviconMimeType) override; \
  NS_IMETHOD SetAndFetchFaviconForPage(nsIURI *aPageURI, nsIURI *aFaviconURI, bool aForceReload, uint32_t aFaviconLoadType, nsIFaviconDataCallback *aCallback, nsIPrincipal *aLoadingPrincipal, uint64_t aRequestContextID, mozIPlacesPendingOperation **_retval) override; \
  NS_IMETHOD ReplaceFaviconData(nsIURI *aFaviconURI, const nsTArray<uint8_t >& aData, const nsACString& aMimeType, PRTime aExpiration) override; \
  NS_IMETHOD ReplaceFaviconDataFromDataURL(nsIURI *aFaviconURI, const nsAString& aDataURL, PRTime aExpiration, nsIPrincipal *aLoadingPrincipal) override; \
  NS_IMETHOD GetFaviconURLForPage(nsIURI *aPageURI, nsIFaviconDataCallback *aCallback, uint16_t aPreferredWidth) override; \
  NS_IMETHOD GetFaviconDataForPage(nsIURI *aPageURI, nsIFaviconDataCallback *aCallback, uint16_t aPreferredWidth) override; \
  NS_IMETHOD CopyFavicons(nsIURI *aFromPageURI, nsIURI *aToPageURI, uint32_t aFaviconLoadType, nsIFaviconDataCallback *aCallback) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFAVICONSERVICE \
  nsresult GetFaviconLinkForIcon(nsIURI *aFaviconURI, nsIURI **_retval); \
  nsresult ExpireAllFavicons(void); \
  nsresult SetDefaultIconURIPreferredSize(uint16_t aDefaultSize); \
  nsresult PreferredSizeFromURI(nsIURI *aURI, uint16_t *_retval); \
  nsresult GetDefaultFavicon(nsIURI **aDefaultFavicon); \
  nsresult GetDefaultFaviconMimeType(nsACString& aDefaultFaviconMimeType); \
  nsresult SetAndFetchFaviconForPage(nsIURI *aPageURI, nsIURI *aFaviconURI, bool aForceReload, uint32_t aFaviconLoadType, nsIFaviconDataCallback *aCallback, nsIPrincipal *aLoadingPrincipal, uint64_t aRequestContextID, mozIPlacesPendingOperation **_retval); \
  nsresult ReplaceFaviconData(nsIURI *aFaviconURI, const nsTArray<uint8_t >& aData, const nsACString& aMimeType, PRTime aExpiration); \
  nsresult ReplaceFaviconDataFromDataURL(nsIURI *aFaviconURI, const nsAString& aDataURL, PRTime aExpiration, nsIPrincipal *aLoadingPrincipal); \
  nsresult GetFaviconURLForPage(nsIURI *aPageURI, nsIFaviconDataCallback *aCallback, uint16_t aPreferredWidth); \
  nsresult GetFaviconDataForPage(nsIURI *aPageURI, nsIFaviconDataCallback *aCallback, uint16_t aPreferredWidth); \
  nsresult CopyFavicons(nsIURI *aFromPageURI, nsIURI *aToPageURI, uint32_t aFaviconLoadType, nsIFaviconDataCallback *aCallback); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFAVICONSERVICE(_to) \
  NS_IMETHOD GetFaviconLinkForIcon(nsIURI *aFaviconURI, nsIURI **_retval) override { return _to GetFaviconLinkForIcon(aFaviconURI, _retval); } \
  NS_IMETHOD ExpireAllFavicons(void) override { return _to ExpireAllFavicons(); } \
  NS_IMETHOD SetDefaultIconURIPreferredSize(uint16_t aDefaultSize) override { return _to SetDefaultIconURIPreferredSize(aDefaultSize); } \
  NS_IMETHOD PreferredSizeFromURI(nsIURI *aURI, uint16_t *_retval) override { return _to PreferredSizeFromURI(aURI, _retval); } \
  NS_IMETHOD GetDefaultFavicon(nsIURI **aDefaultFavicon) override { return _to GetDefaultFavicon(aDefaultFavicon); } \
  NS_IMETHOD GetDefaultFaviconMimeType(nsACString& aDefaultFaviconMimeType) override { return _to GetDefaultFaviconMimeType(aDefaultFaviconMimeType); } \
  NS_IMETHOD SetAndFetchFaviconForPage(nsIURI *aPageURI, nsIURI *aFaviconURI, bool aForceReload, uint32_t aFaviconLoadType, nsIFaviconDataCallback *aCallback, nsIPrincipal *aLoadingPrincipal, uint64_t aRequestContextID, mozIPlacesPendingOperation **_retval) override { return _to SetAndFetchFaviconForPage(aPageURI, aFaviconURI, aForceReload, aFaviconLoadType, aCallback, aLoadingPrincipal, aRequestContextID, _retval); } \
  NS_IMETHOD ReplaceFaviconData(nsIURI *aFaviconURI, const nsTArray<uint8_t >& aData, const nsACString& aMimeType, PRTime aExpiration) override { return _to ReplaceFaviconData(aFaviconURI, aData, aMimeType, aExpiration); } \
  NS_IMETHOD ReplaceFaviconDataFromDataURL(nsIURI *aFaviconURI, const nsAString& aDataURL, PRTime aExpiration, nsIPrincipal *aLoadingPrincipal) override { return _to ReplaceFaviconDataFromDataURL(aFaviconURI, aDataURL, aExpiration, aLoadingPrincipal); } \
  NS_IMETHOD GetFaviconURLForPage(nsIURI *aPageURI, nsIFaviconDataCallback *aCallback, uint16_t aPreferredWidth) override { return _to GetFaviconURLForPage(aPageURI, aCallback, aPreferredWidth); } \
  NS_IMETHOD GetFaviconDataForPage(nsIURI *aPageURI, nsIFaviconDataCallback *aCallback, uint16_t aPreferredWidth) override { return _to GetFaviconDataForPage(aPageURI, aCallback, aPreferredWidth); } \
  NS_IMETHOD CopyFavicons(nsIURI *aFromPageURI, nsIURI *aToPageURI, uint32_t aFaviconLoadType, nsIFaviconDataCallback *aCallback) override { return _to CopyFavicons(aFromPageURI, aToPageURI, aFaviconLoadType, aCallback); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFAVICONSERVICE(_to) \
  NS_IMETHOD GetFaviconLinkForIcon(nsIURI *aFaviconURI, nsIURI **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFaviconLinkForIcon(aFaviconURI, _retval); } \
  NS_IMETHOD ExpireAllFavicons(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ExpireAllFavicons(); } \
  NS_IMETHOD SetDefaultIconURIPreferredSize(uint16_t aDefaultSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDefaultIconURIPreferredSize(aDefaultSize); } \
  NS_IMETHOD PreferredSizeFromURI(nsIURI *aURI, uint16_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PreferredSizeFromURI(aURI, _retval); } \
  NS_IMETHOD GetDefaultFavicon(nsIURI **aDefaultFavicon) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDefaultFavicon(aDefaultFavicon); } \
  NS_IMETHOD GetDefaultFaviconMimeType(nsACString& aDefaultFaviconMimeType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDefaultFaviconMimeType(aDefaultFaviconMimeType); } \
  NS_IMETHOD SetAndFetchFaviconForPage(nsIURI *aPageURI, nsIURI *aFaviconURI, bool aForceReload, uint32_t aFaviconLoadType, nsIFaviconDataCallback *aCallback, nsIPrincipal *aLoadingPrincipal, uint64_t aRequestContextID, mozIPlacesPendingOperation **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAndFetchFaviconForPage(aPageURI, aFaviconURI, aForceReload, aFaviconLoadType, aCallback, aLoadingPrincipal, aRequestContextID, _retval); } \
  NS_IMETHOD ReplaceFaviconData(nsIURI *aFaviconURI, const nsTArray<uint8_t >& aData, const nsACString& aMimeType, PRTime aExpiration) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReplaceFaviconData(aFaviconURI, aData, aMimeType, aExpiration); } \
  NS_IMETHOD ReplaceFaviconDataFromDataURL(nsIURI *aFaviconURI, const nsAString& aDataURL, PRTime aExpiration, nsIPrincipal *aLoadingPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReplaceFaviconDataFromDataURL(aFaviconURI, aDataURL, aExpiration, aLoadingPrincipal); } \
  NS_IMETHOD GetFaviconURLForPage(nsIURI *aPageURI, nsIFaviconDataCallback *aCallback, uint16_t aPreferredWidth) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFaviconURLForPage(aPageURI, aCallback, aPreferredWidth); } \
  NS_IMETHOD GetFaviconDataForPage(nsIURI *aPageURI, nsIFaviconDataCallback *aCallback, uint16_t aPreferredWidth) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFaviconDataForPage(aPageURI, aCallback, aPreferredWidth); } \
  NS_IMETHOD CopyFavicons(nsIURI *aFromPageURI, nsIURI *aToPageURI, uint32_t aFaviconLoadType, nsIFaviconDataCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CopyFavicons(aFromPageURI, aToPageURI, aFaviconLoadType, aCallback); } 


/* starting interface:    nsIFaviconDataCallback */
#define NS_IFAVICONDATACALLBACK_IID_STR "c85e5c82-b70f-4621-9528-beb2aa47fb44"

#define NS_IFAVICONDATACALLBACK_IID \
  {0xc85e5c82, 0xb70f, 0x4621, \
    { 0x95, 0x28, 0xbe, 0xb2, 0xaa, 0x47, 0xfb, 0x44 }}

class NS_NO_VTABLE nsIFaviconDataCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFAVICONDATACALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFaviconDataCallback;

  /* void onComplete (in nsIURI aFaviconURI, in unsigned long aDataLen, [array, size_is (aDataLen), const] in octet aData, in AUTF8String aMimeType, in unsigned short aWidth); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnComplete(nsIURI *aFaviconURI, uint32_t aDataLen, const uint8_t *aData, const nsACString& aMimeType, uint16_t aWidth) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFaviconDataCallback, NS_IFAVICONDATACALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFAVICONDATACALLBACK \
  NS_IMETHOD OnComplete(nsIURI *aFaviconURI, uint32_t aDataLen, const uint8_t *aData, const nsACString& aMimeType, uint16_t aWidth) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFAVICONDATACALLBACK \
  nsresult OnComplete(nsIURI *aFaviconURI, uint32_t aDataLen, const uint8_t *aData, const nsACString& aMimeType, uint16_t aWidth); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFAVICONDATACALLBACK(_to) \
  NS_IMETHOD OnComplete(nsIURI *aFaviconURI, uint32_t aDataLen, const uint8_t *aData, const nsACString& aMimeType, uint16_t aWidth) override { return _to OnComplete(aFaviconURI, aDataLen, aData, aMimeType, aWidth); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFAVICONDATACALLBACK(_to) \
  NS_IMETHOD OnComplete(nsIURI *aFaviconURI, uint32_t aDataLen, const uint8_t *aData, const nsACString& aMimeType, uint16_t aWidth) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnComplete(aFaviconURI, aDataLen, aData, aMimeType, aWidth); } 


/**
 * Notification sent when all favicons are expired.
 */
#define NS_PLACES_FAVICONS_EXPIRED_TOPIC_ID "places-favicons-expired"
#define FAVICON_DEFAULT_URL "chrome://mozapps/skin/places/defaultFavicon.svg"
#define FAVICON_DEFAULT_MIMETYPE "image/svg+xml"
#define FAVICON_ERRORPAGE_URL "chrome://global/skin/icons/warning.svg"

#endif /* __gen_nsIFaviconService_h__ */
