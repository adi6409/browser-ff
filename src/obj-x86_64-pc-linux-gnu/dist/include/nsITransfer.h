/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/base/nsITransfer.idl
 */

#ifndef __gen_nsITransfer_h__
#define __gen_nsITransfer_h__


#ifndef __gen_nsIWebProgressListener2_h__
#include "nsIWebProgressListener2.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIArray; /* forward declaration */

class nsIURI; /* forward declaration */

class nsICancelable; /* forward declaration */

class nsIMIMEInfo; /* forward declaration */

class nsIFile; /* forward declaration */

namespace mozilla {
namespace dom {
class BrowsingContext; /* webidl BrowsingContext */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsITransfer */
#define NS_ITRANSFER_IID_STR "37ec75d3-97ad-4da8-afaa-eabe5b4afd73"

#define NS_ITRANSFER_IID \
  {0x37ec75d3, 0x97ad, 0x4da8, \
    { 0xaf, 0xaa, 0xea, 0xbe, 0x5b, 0x4a, 0xfd, 0x73 }}

class NS_NO_VTABLE nsITransfer : public nsIWebProgressListener2 {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITRANSFER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITransfer;

  enum {
    DOWNLOAD_ACCEPTABLE = 0U,
    DOWNLOAD_FORBIDDEN = 1U,
    DOWNLOAD_POTENTIALLY_UNSAFE = 2U
  };

  /* void init (in nsIURI aSource, in nsIURI aTarget, in AString aDisplayName, in nsIMIMEInfo aMIMEInfo, in PRTime startTime, in nsIFile aTempFile, in nsICancelable aCancelable, in boolean aIsPrivate, in long aDownloadClassification); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(nsIURI *aSource, nsIURI *aTarget, const nsAString& aDisplayName, nsIMIMEInfo *aMIMEInfo, PRTime startTime, nsIFile *aTempFile, nsICancelable *aCancelable, bool aIsPrivate, int32_t aDownloadClassification) = 0;

  /* void initWithBrowsingContext (in nsIURI aSource, in nsIURI aTarget, in AString aDisplayName, in nsIMIMEInfo aMIMEInfo, in PRTime startTime, in nsIFile aTempFile, in nsICancelable aCancelable, in boolean aIsPrivate, in long aDownloadClassification, in BrowsingContext aBrowsingContext, in boolean aHandleInternally); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD InitWithBrowsingContext(nsIURI *aSource, nsIURI *aTarget, const nsAString& aDisplayName, nsIMIMEInfo *aMIMEInfo, PRTime startTime, nsIFile *aTempFile, nsICancelable *aCancelable, bool aIsPrivate, int32_t aDownloadClassification, mozilla::dom::BrowsingContext *aBrowsingContext, bool aHandleInternally) = 0;

  /* void setSha256Hash (in ACString aHash); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSha256Hash(const nsACString& aHash) = 0;

  /* void setSignatureInfo (in Array<Array<Array<uint8_t>>> aSignatureInfo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSignatureInfo(const nsTArray<nsTArray<nsTArray<uint8_t >>>& aSignatureInfo) = 0;

  /* void setRedirects (in nsIArray aRedirects); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetRedirects(nsIArray *aRedirects) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITransfer, NS_ITRANSFER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITRANSFER \
  NS_IMETHOD Init(nsIURI *aSource, nsIURI *aTarget, const nsAString& aDisplayName, nsIMIMEInfo *aMIMEInfo, PRTime startTime, nsIFile *aTempFile, nsICancelable *aCancelable, bool aIsPrivate, int32_t aDownloadClassification) override; \
  NS_IMETHOD InitWithBrowsingContext(nsIURI *aSource, nsIURI *aTarget, const nsAString& aDisplayName, nsIMIMEInfo *aMIMEInfo, PRTime startTime, nsIFile *aTempFile, nsICancelable *aCancelable, bool aIsPrivate, int32_t aDownloadClassification, mozilla::dom::BrowsingContext *aBrowsingContext, bool aHandleInternally) override; \
  NS_IMETHOD SetSha256Hash(const nsACString& aHash) override; \
  NS_IMETHOD SetSignatureInfo(const nsTArray<nsTArray<nsTArray<uint8_t >>>& aSignatureInfo) override; \
  NS_IMETHOD SetRedirects(nsIArray *aRedirects) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITRANSFER \
  nsresult Init(nsIURI *aSource, nsIURI *aTarget, const nsAString& aDisplayName, nsIMIMEInfo *aMIMEInfo, PRTime startTime, nsIFile *aTempFile, nsICancelable *aCancelable, bool aIsPrivate, int32_t aDownloadClassification); \
  nsresult InitWithBrowsingContext(nsIURI *aSource, nsIURI *aTarget, const nsAString& aDisplayName, nsIMIMEInfo *aMIMEInfo, PRTime startTime, nsIFile *aTempFile, nsICancelable *aCancelable, bool aIsPrivate, int32_t aDownloadClassification, mozilla::dom::BrowsingContext *aBrowsingContext, bool aHandleInternally); \
  nsresult SetSha256Hash(const nsACString& aHash); \
  nsresult SetSignatureInfo(const nsTArray<nsTArray<nsTArray<uint8_t >>>& aSignatureInfo); \
  nsresult SetRedirects(nsIArray *aRedirects); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITRANSFER(_to) \
  NS_IMETHOD Init(nsIURI *aSource, nsIURI *aTarget, const nsAString& aDisplayName, nsIMIMEInfo *aMIMEInfo, PRTime startTime, nsIFile *aTempFile, nsICancelable *aCancelable, bool aIsPrivate, int32_t aDownloadClassification) override { return _to Init(aSource, aTarget, aDisplayName, aMIMEInfo, startTime, aTempFile, aCancelable, aIsPrivate, aDownloadClassification); } \
  NS_IMETHOD InitWithBrowsingContext(nsIURI *aSource, nsIURI *aTarget, const nsAString& aDisplayName, nsIMIMEInfo *aMIMEInfo, PRTime startTime, nsIFile *aTempFile, nsICancelable *aCancelable, bool aIsPrivate, int32_t aDownloadClassification, mozilla::dom::BrowsingContext *aBrowsingContext, bool aHandleInternally) override { return _to InitWithBrowsingContext(aSource, aTarget, aDisplayName, aMIMEInfo, startTime, aTempFile, aCancelable, aIsPrivate, aDownloadClassification, aBrowsingContext, aHandleInternally); } \
  NS_IMETHOD SetSha256Hash(const nsACString& aHash) override { return _to SetSha256Hash(aHash); } \
  NS_IMETHOD SetSignatureInfo(const nsTArray<nsTArray<nsTArray<uint8_t >>>& aSignatureInfo) override { return _to SetSignatureInfo(aSignatureInfo); } \
  NS_IMETHOD SetRedirects(nsIArray *aRedirects) override { return _to SetRedirects(aRedirects); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITRANSFER(_to) \
  NS_IMETHOD Init(nsIURI *aSource, nsIURI *aTarget, const nsAString& aDisplayName, nsIMIMEInfo *aMIMEInfo, PRTime startTime, nsIFile *aTempFile, nsICancelable *aCancelable, bool aIsPrivate, int32_t aDownloadClassification) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aSource, aTarget, aDisplayName, aMIMEInfo, startTime, aTempFile, aCancelable, aIsPrivate, aDownloadClassification); } \
  NS_IMETHOD InitWithBrowsingContext(nsIURI *aSource, nsIURI *aTarget, const nsAString& aDisplayName, nsIMIMEInfo *aMIMEInfo, PRTime startTime, nsIFile *aTempFile, nsICancelable *aCancelable, bool aIsPrivate, int32_t aDownloadClassification, mozilla::dom::BrowsingContext *aBrowsingContext, bool aHandleInternally) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitWithBrowsingContext(aSource, aTarget, aDisplayName, aMIMEInfo, startTime, aTempFile, aCancelable, aIsPrivate, aDownloadClassification, aBrowsingContext, aHandleInternally); } \
  NS_IMETHOD SetSha256Hash(const nsACString& aHash) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSha256Hash(aHash); } \
  NS_IMETHOD SetSignatureInfo(const nsTArray<nsTArray<nsTArray<uint8_t >>>& aSignatureInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSignatureInfo(aSignatureInfo); } \
  NS_IMETHOD SetRedirects(nsIArray *aRedirects) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRedirects(aRedirects); } 

/**
 * A component with this contract ID will be created each time a download is
 * started, and nsITransfer::Init will be called on it and an observer will be set.
 *
 * Notifications of the download progress will happen via
 * nsIWebProgressListener/nsIWebProgressListener2.
 *
 * INTERFACES THAT MUST BE IMPLEMENTED:
 *   nsITransfer
 *   nsIWebProgressListener
 *   nsIWebProgressListener2
 *
 * XXX move this to nsEmbedCID.h once the interfaces (and the contract ID) are
 * frozen.
 */
#define NS_TRANSFER_CONTRACTID "@mozilla.org/transfer;1"

#endif /* __gen_nsITransfer_h__ */
