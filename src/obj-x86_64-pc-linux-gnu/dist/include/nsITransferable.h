/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsITransferable.idl
 */

#ifndef __gen_nsITransferable_h__
#define __gen_nsITransferable_h__


#ifndef __gen_nsIArray_h__
#include "nsIArray.h"
#endif

#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIFormatConverter_h__
#include "nsIFormatConverter.h"
#endif

#ifndef __gen_nsIContentPolicy_h__
#include "nsIContentPolicy.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsICookieJarSettings; /* forward declaration */

class nsIPrincipal; /* forward declaration */


// these probably shouldn't live here, but in some central repository shared
// by the entire app.
#define kTextMime                   "text/plain"
#define kRTFMime                    "text/rtf"
#define kUnicodeMime                "text/unicode"
#define kMozTextInternal          "text/x-moz-text-internal"  // text data which isn't suppoed to be parsed by other apps.
#define kHTMLMime                   "text/html"
#define kAOLMailMime                "AOLMAIL"
#define kPNGImageMime               "image/png"
#define kJPEGImageMime              "image/jpeg"
#define kJPGImageMime               "image/jpg"
#define kGIFImageMime               "image/gif"
#define kFileMime                   "application/x-moz-file"
#define kURLMime                    "text/x-moz-url"        // data contains url\ntitle
#define kURLDataMime                "text/x-moz-url-data"   // data contains url only
#define kURLDescriptionMime         "text/x-moz-url-desc"   // data contains description
#define kURLPrivateMime             "text/x-moz-url-priv"   // same as kURLDataMime but for private uses
#define kNativeImageMime            "application/x-moz-nativeimage"
#define kNativeHTMLMime             "application/x-moz-nativehtml"
// These are used to indicate the context for a fragment of HTML source, such
// that some parent structure and style can be preserved. kHTMLContext
// contains the serialized ancestor elements, whereas kHTMLInfo are numbers
// identifying where in the context the fragment was from.
#define kHTMLContext   "text/_moz_htmlcontext"
#define kHTMLInfo      "text/_moz_htmlinfo"
// Holds the MIME type from the image request. This is used to ensure the
// local application handler for the request's MIME type accepts images with
// the given filename extension (from kFilePromiseDestFilename). When the
// image is dragged out, we replace the extension with a compatible extension.
#define kImageRequestMime           "text/_moz_requestmime"
// the source URL for a file promise
#define kFilePromiseURLMime         "application/x-moz-file-promise-url"
// the destination filename for a file promise
#define kFilePromiseDestFilename    "application/x-moz-file-promise-dest-filename"
// a dataless flavor used to interact with the OS during file drags
#define kFilePromiseMime            "application/x-moz-file-promise"
// a synthetic flavor, put into the transferable once we know the destination directory of a file drag
#define kFilePromiseDirectoryMime   "application/x-moz-file-promise-dir"
#define kCustomTypesMime "application/x-moz-custom-clipdata"
class nsITransferable; /* forward declaration */

class nsILoadContext; /* forward declaration */


/* starting interface:    nsIFlavorDataProvider */
#define NS_IFLAVORDATAPROVIDER_IID_STR "7e225e5f-711c-11d7-9fae-000393636592"

#define NS_IFLAVORDATAPROVIDER_IID \
  {0x7e225e5f, 0x711c, 0x11d7, \
    { 0x9f, 0xae, 0x00, 0x03, 0x93, 0x63, 0x65, 0x92 }}

class NS_NO_VTABLE nsIFlavorDataProvider : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFLAVORDATAPROVIDER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFlavorDataProvider;

  /* void getFlavorData (in nsITransferable aTransferable, in string aFlavor, out nsISupports aData); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFlavorData(nsITransferable *aTransferable, const char * aFlavor, nsISupports **aData) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFlavorDataProvider, NS_IFLAVORDATAPROVIDER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFLAVORDATAPROVIDER \
  NS_IMETHOD GetFlavorData(nsITransferable *aTransferable, const char * aFlavor, nsISupports **aData) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFLAVORDATAPROVIDER \
  nsresult GetFlavorData(nsITransferable *aTransferable, const char * aFlavor, nsISupports **aData); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFLAVORDATAPROVIDER(_to) \
  NS_IMETHOD GetFlavorData(nsITransferable *aTransferable, const char * aFlavor, nsISupports **aData) override { return _to GetFlavorData(aTransferable, aFlavor, aData); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFLAVORDATAPROVIDER(_to) \
  NS_IMETHOD GetFlavorData(nsITransferable *aTransferable, const char * aFlavor, nsISupports **aData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFlavorData(aTransferable, aFlavor, aData); } 


/* starting interface:    nsITransferable */
#define NS_ITRANSFERABLE_IID_STR "97e0c418-1c1e-4106-bad1-9fcb11dff2fe"

#define NS_ITRANSFERABLE_IID \
  {0x97e0c418, 0x1c1e, 0x4106, \
    { 0xba, 0xd1, 0x9f, 0xcb, 0x11, 0xdf, 0xf2, 0xfe }}

class NS_NO_VTABLE nsITransferable : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITRANSFERABLE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITransferable;

  /* void init (in nsILoadContext aContext); */
  NS_IMETHOD Init(nsILoadContext *aContext) = 0;

  /* Array<ACString> flavorsTransferableCanExport (); */
  NS_IMETHOD FlavorsTransferableCanExport(nsTArray<nsCString >& _retval) = 0;

  /* [must_use] void getTransferData (in string aFlavor, out nsISupports aData); */
  [[nodiscard]] NS_IMETHOD GetTransferData(const char * aFlavor, nsISupports **aData) = 0;

  /* void getAnyTransferData (out ACString aFlavor, out nsISupports aData); */
  NS_IMETHOD GetAnyTransferData(nsACString& aFlavor, nsISupports **aData) = 0;

  /* Array<ACString> flavorsTransferableCanImport (); */
  NS_IMETHOD FlavorsTransferableCanImport(nsTArray<nsCString >& _retval) = 0;

  /* void setTransferData (in string aFlavor, in nsISupports aData); */
  NS_IMETHOD SetTransferData(const char * aFlavor, nsISupports *aData) = 0;

  /* void addDataFlavor (in string aDataFlavor); */
  NS_IMETHOD AddDataFlavor(const char * aDataFlavor) = 0;

  /* void removeDataFlavor (in string aDataFlavor); */
  NS_IMETHOD RemoveDataFlavor(const char * aDataFlavor) = 0;

  /* attribute nsIFormatConverter converter; */
  NS_IMETHOD GetConverter(nsIFormatConverter **aConverter) = 0;
  NS_IMETHOD SetConverter(nsIFormatConverter *aConverter) = 0;

  /* [nostdcall,notxpcom] attribute boolean isPrivateData; */
  virtual bool GetIsPrivateData() = 0;
  virtual void SetIsPrivateData(bool aIsPrivateData) = 0;

  /* [nostdcall,notxpcom] attribute nsIPrincipal requestingPrincipal; */
  virtual nsIPrincipal * GetRequestingPrincipal() = 0;
  virtual void SetRequestingPrincipal(nsIPrincipal *aRequestingPrincipal) = 0;

  /* [nostdcall,notxpcom] attribute nsContentPolicyType contentPolicyType; */
  virtual nsContentPolicyType GetContentPolicyType() = 0;
  virtual void SetContentPolicyType(nsContentPolicyType aContentPolicyType) = 0;

  /* [nostdcall,notxpcom] attribute nsICookieJarSettings cookieJarSettings; */
  virtual nsICookieJarSettings * GetCookieJarSettings() = 0;
  virtual void SetCookieJarSettings(nsICookieJarSettings *aCookieJarSettings) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITransferable, NS_ITRANSFERABLE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITRANSFERABLE \
  NS_IMETHOD Init(nsILoadContext *aContext) override; \
  NS_IMETHOD FlavorsTransferableCanExport(nsTArray<nsCString >& _retval) override; \
  [[nodiscard]] NS_IMETHOD GetTransferData(const char * aFlavor, nsISupports **aData) override; \
  NS_IMETHOD GetAnyTransferData(nsACString& aFlavor, nsISupports **aData) override; \
  NS_IMETHOD FlavorsTransferableCanImport(nsTArray<nsCString >& _retval) override; \
  NS_IMETHOD SetTransferData(const char * aFlavor, nsISupports *aData) override; \
  NS_IMETHOD AddDataFlavor(const char * aDataFlavor) override; \
  NS_IMETHOD RemoveDataFlavor(const char * aDataFlavor) override; \
  NS_IMETHOD GetConverter(nsIFormatConverter **aConverter) override; \
  NS_IMETHOD SetConverter(nsIFormatConverter *aConverter) override; \
  virtual bool GetIsPrivateData() override; \
  virtual void SetIsPrivateData(bool aIsPrivateData) override; \
  virtual nsIPrincipal * GetRequestingPrincipal() override; \
  virtual void SetRequestingPrincipal(nsIPrincipal *aRequestingPrincipal) override; \
  virtual nsContentPolicyType GetContentPolicyType() override; \
  virtual void SetContentPolicyType(nsContentPolicyType aContentPolicyType) override; \
  virtual nsICookieJarSettings * GetCookieJarSettings() override; \
  virtual void SetCookieJarSettings(nsICookieJarSettings *aCookieJarSettings) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITRANSFERABLE \
  nsresult Init(nsILoadContext *aContext); \
  nsresult FlavorsTransferableCanExport(nsTArray<nsCString >& _retval); \
  [[nodiscard]] nsresult GetTransferData(const char * aFlavor, nsISupports **aData); \
  nsresult GetAnyTransferData(nsACString& aFlavor, nsISupports **aData); \
  nsresult FlavorsTransferableCanImport(nsTArray<nsCString >& _retval); \
  nsresult SetTransferData(const char * aFlavor, nsISupports *aData); \
  nsresult AddDataFlavor(const char * aDataFlavor); \
  nsresult RemoveDataFlavor(const char * aDataFlavor); \
  nsresult GetConverter(nsIFormatConverter **aConverter); \
  nsresult SetConverter(nsIFormatConverter *aConverter); \
  bool GetIsPrivateData(); \
  void SetIsPrivateData(bool aIsPrivateData); \
  nsIPrincipal * GetRequestingPrincipal(); \
  void SetRequestingPrincipal(nsIPrincipal *aRequestingPrincipal); \
  nsContentPolicyType GetContentPolicyType(); \
  void SetContentPolicyType(nsContentPolicyType aContentPolicyType); \
  nsICookieJarSettings * GetCookieJarSettings(); \
  void SetCookieJarSettings(nsICookieJarSettings *aCookieJarSettings); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITRANSFERABLE(_to) \
  NS_IMETHOD Init(nsILoadContext *aContext) override { return _to Init(aContext); } \
  NS_IMETHOD FlavorsTransferableCanExport(nsTArray<nsCString >& _retval) override { return _to FlavorsTransferableCanExport(_retval); } \
  [[nodiscard]] NS_IMETHOD GetTransferData(const char * aFlavor, nsISupports **aData) override { return _to GetTransferData(aFlavor, aData); } \
  NS_IMETHOD GetAnyTransferData(nsACString& aFlavor, nsISupports **aData) override { return _to GetAnyTransferData(aFlavor, aData); } \
  NS_IMETHOD FlavorsTransferableCanImport(nsTArray<nsCString >& _retval) override { return _to FlavorsTransferableCanImport(_retval); } \
  NS_IMETHOD SetTransferData(const char * aFlavor, nsISupports *aData) override { return _to SetTransferData(aFlavor, aData); } \
  NS_IMETHOD AddDataFlavor(const char * aDataFlavor) override { return _to AddDataFlavor(aDataFlavor); } \
  NS_IMETHOD RemoveDataFlavor(const char * aDataFlavor) override { return _to RemoveDataFlavor(aDataFlavor); } \
  NS_IMETHOD GetConverter(nsIFormatConverter **aConverter) override { return _to GetConverter(aConverter); } \
  NS_IMETHOD SetConverter(nsIFormatConverter *aConverter) override { return _to SetConverter(aConverter); } \
  virtual bool GetIsPrivateData() override { return _to GetIsPrivateData(); } \
  virtual void SetIsPrivateData(bool aIsPrivateData) override { return _to SetIsPrivateData(aIsPrivateData); } \
  virtual nsIPrincipal * GetRequestingPrincipal() override { return _to GetRequestingPrincipal(); } \
  virtual void SetRequestingPrincipal(nsIPrincipal *aRequestingPrincipal) override { return _to SetRequestingPrincipal(aRequestingPrincipal); } \
  virtual nsContentPolicyType GetContentPolicyType() override { return _to GetContentPolicyType(); } \
  virtual void SetContentPolicyType(nsContentPolicyType aContentPolicyType) override { return _to SetContentPolicyType(aContentPolicyType); } \
  virtual nsICookieJarSettings * GetCookieJarSettings() override { return _to GetCookieJarSettings(); } \
  virtual void SetCookieJarSettings(nsICookieJarSettings *aCookieJarSettings) override { return _to SetCookieJarSettings(aCookieJarSettings); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITRANSFERABLE(_to) \
  NS_IMETHOD Init(nsILoadContext *aContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aContext); } \
  NS_IMETHOD FlavorsTransferableCanExport(nsTArray<nsCString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FlavorsTransferableCanExport(_retval); } \
  [[nodiscard]] NS_IMETHOD GetTransferData(const char * aFlavor, nsISupports **aData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTransferData(aFlavor, aData); } \
  NS_IMETHOD GetAnyTransferData(nsACString& aFlavor, nsISupports **aData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAnyTransferData(aFlavor, aData); } \
  NS_IMETHOD FlavorsTransferableCanImport(nsTArray<nsCString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FlavorsTransferableCanImport(_retval); } \
  NS_IMETHOD SetTransferData(const char * aFlavor, nsISupports *aData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTransferData(aFlavor, aData); } \
  NS_IMETHOD AddDataFlavor(const char * aDataFlavor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddDataFlavor(aDataFlavor); } \
  NS_IMETHOD RemoveDataFlavor(const char * aDataFlavor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveDataFlavor(aDataFlavor); } \
  NS_IMETHOD GetConverter(nsIFormatConverter **aConverter) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetConverter(aConverter); } \
  NS_IMETHOD SetConverter(nsIFormatConverter *aConverter) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetConverter(aConverter); } \
  virtual bool GetIsPrivateData() override; \
  virtual void SetIsPrivateData(bool aIsPrivateData) override; \
  virtual nsIPrincipal * GetRequestingPrincipal() override; \
  virtual void SetRequestingPrincipal(nsIPrincipal *aRequestingPrincipal) override; \
  virtual nsContentPolicyType GetContentPolicyType() override; \
  virtual void SetContentPolicyType(nsContentPolicyType aContentPolicyType) override; \
  virtual nsICookieJarSettings * GetCookieJarSettings() override; \
  virtual void SetCookieJarSettings(nsICookieJarSettings *aCookieJarSettings) override; 


#endif /* __gen_nsITransferable_h__ */
