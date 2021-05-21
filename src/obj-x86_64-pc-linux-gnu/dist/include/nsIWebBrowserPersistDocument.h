/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/webbrowserpersist/nsIWebBrowserPersistDocument.idl
 */

#ifndef __gen_nsIWebBrowserPersistDocument_h__
#define __gen_nsIWebBrowserPersistDocument_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIContentPolicy_h__
#include "nsIContentPolicy.h"
#endif

#include "js/GCAnnotations.h"

#include "mozilla/AlreadyAddRefed.h"
#include "mozilla/Assertions.h"
#include "mozilla/DebugOnly.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIInputStream; /* forward declaration */

class nsIOutputStream; /* forward declaration */

class nsIPrincipal; /* forward declaration */

class nsIRemoteTab; /* forward declaration */

class nsIWebBrowserPersistResourceVisitor; /* forward declaration */

class nsIWebBrowserPersistWriteCompletion; /* forward declaration */

class nsIReferrerInfo; /* forward declaration */

class nsISHEntry; /* forward declaration */

class nsICookieJarSettings; /* forward declaration */

namespace mozilla {
namespace dom {
class BrowsingContext; /* webidl BrowsingContext */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIWebBrowserPersistURIMap */
#define NS_IWEBBROWSERPERSISTURIMAP_IID_STR "d52e8b93-2771-45e8-a5b0-6e12b667046b"

#define NS_IWEBBROWSERPERSISTURIMAP_IID \
  {0xd52e8b93, 0x2771, 0x45e8, \
    { 0xa5, 0xb0, 0x6e, 0x12, 0xb6, 0x67, 0x04, 0x6b }}

class NS_NO_VTABLE nsIWebBrowserPersistURIMap : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWEBBROWSERPERSISTURIMAP_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWebBrowserPersistURIMap;

  /* readonly attribute unsigned long numMappedURIs; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNumMappedURIs(uint32_t *aNumMappedURIs) = 0;

  /* void getURIMapping (in unsigned long aIndex, out AUTF8String aMapFrom, out AUTF8String aMapTo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetURIMapping(uint32_t aIndex, nsACString& aMapFrom, nsACString& aMapTo) = 0;

  /* readonly attribute AUTF8String targetBaseURI; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTargetBaseURI(nsACString& aTargetBaseURI) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWebBrowserPersistURIMap, NS_IWEBBROWSERPERSISTURIMAP_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWEBBROWSERPERSISTURIMAP \
  NS_IMETHOD GetNumMappedURIs(uint32_t *aNumMappedURIs) override; \
  NS_IMETHOD GetURIMapping(uint32_t aIndex, nsACString& aMapFrom, nsACString& aMapTo) override; \
  NS_IMETHOD GetTargetBaseURI(nsACString& aTargetBaseURI) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWEBBROWSERPERSISTURIMAP \
  nsresult GetNumMappedURIs(uint32_t *aNumMappedURIs); \
  nsresult GetURIMapping(uint32_t aIndex, nsACString& aMapFrom, nsACString& aMapTo); \
  nsresult GetTargetBaseURI(nsACString& aTargetBaseURI); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWEBBROWSERPERSISTURIMAP(_to) \
  NS_IMETHOD GetNumMappedURIs(uint32_t *aNumMappedURIs) override { return _to GetNumMappedURIs(aNumMappedURIs); } \
  NS_IMETHOD GetURIMapping(uint32_t aIndex, nsACString& aMapFrom, nsACString& aMapTo) override { return _to GetURIMapping(aIndex, aMapFrom, aMapTo); } \
  NS_IMETHOD GetTargetBaseURI(nsACString& aTargetBaseURI) override { return _to GetTargetBaseURI(aTargetBaseURI); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWEBBROWSERPERSISTURIMAP(_to) \
  NS_IMETHOD GetNumMappedURIs(uint32_t *aNumMappedURIs) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNumMappedURIs(aNumMappedURIs); } \
  NS_IMETHOD GetURIMapping(uint32_t aIndex, nsACString& aMapFrom, nsACString& aMapTo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetURIMapping(aIndex, aMapFrom, aMapTo); } \
  NS_IMETHOD GetTargetBaseURI(nsACString& aTargetBaseURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTargetBaseURI(aTargetBaseURI); } 


/* starting interface:    nsIWebBrowserPersistDocument */
#define NS_IWEBBROWSERPERSISTDOCUMENT_IID_STR "74aa4918-5d15-46b6-9ccf-74f9696d721d"

#define NS_IWEBBROWSERPERSISTDOCUMENT_IID \
  {0x74aa4918, 0x5d15, 0x46b6, \
    { 0x9c, 0xcf, 0x74, 0xf9, 0x69, 0x6d, 0x72, 0x1d }}

class NS_NO_VTABLE nsIWebBrowserPersistDocument : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWEBBROWSERPERSISTDOCUMENT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWebBrowserPersistDocument;

  /* readonly attribute boolean isClosed; */
  NS_IMETHOD GetIsClosed(bool *aIsClosed) = 0;

  /* readonly attribute boolean isPrivate; */
  NS_IMETHOD GetIsPrivate(bool *aIsPrivate) = 0;

  /* readonly attribute AUTF8String documentURI; */
  NS_IMETHOD GetDocumentURI(nsACString& aDocumentURI) = 0;

  /* readonly attribute AUTF8String baseURI; */
  NS_IMETHOD GetBaseURI(nsACString& aBaseURI) = 0;

  /* readonly attribute ACString contentType; */
  NS_IMETHOD GetContentType(nsACString& aContentType) = 0;

  /* readonly attribute ACString characterSet; */
  NS_IMETHOD GetCharacterSet(nsACString& aCharacterSet) = 0;

  /* readonly attribute AString title; */
  NS_IMETHOD GetTitle(nsAString& aTitle) = 0;

  /* readonly attribute nsIReferrerInfo referrerInfo; */
  NS_IMETHOD GetReferrerInfo(nsIReferrerInfo **aReferrerInfo) = 0;

  /* readonly attribute nsICookieJarSettings cookieJarSettings; */
  NS_IMETHOD GetCookieJarSettings(nsICookieJarSettings **aCookieJarSettings) = 0;

  /* readonly attribute AString contentDisposition; */
  NS_IMETHOD GetContentDisposition(nsAString& aContentDisposition) = 0;

  /* readonly attribute nsIInputStream postData; */
  NS_IMETHOD GetPostData(nsIInputStream **aPostData) = 0;

  /* readonly attribute nsIPrincipal principal; */
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) = 0;

  /* [infallible] readonly attribute unsigned long cacheKey; */
  NS_IMETHOD GetCacheKey(uint32_t *aCacheKey) = 0;
  inline uint32_t  GetCacheKey()
  {
    uint32_t result;
    mozilla::DebugOnly<nsresult> rv = GetCacheKey(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* attribute unsigned long persistFlags; */
  NS_IMETHOD GetPersistFlags(uint32_t *aPersistFlags) = 0;
  NS_IMETHOD SetPersistFlags(uint32_t aPersistFlags) = 0;

  /* void readResources (in nsIWebBrowserPersistResourceVisitor aVisitor); */
  NS_IMETHOD ReadResources(nsIWebBrowserPersistResourceVisitor *aVisitor) = 0;

  /* void writeContent (in nsIOutputStream aStream, in nsIWebBrowserPersistURIMap aURIMap, in ACString aRequestedContentType, in unsigned long aEncoderFlags, in unsigned long aWrapColumn, in nsIWebBrowserPersistWriteCompletion aCompletion); */
  NS_IMETHOD WriteContent(nsIOutputStream *aStream, nsIWebBrowserPersistURIMap *aURIMap, const nsACString& aRequestedContentType, uint32_t aEncoderFlags, uint32_t aWrapColumn, nsIWebBrowserPersistWriteCompletion *aCompletion) = 0;

  /* [nostdcall,notxpcom] SHEntryRef GetHistory (); */
  virtual already_AddRefed<nsISHEntry> GetHistory(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWebBrowserPersistDocument, NS_IWEBBROWSERPERSISTDOCUMENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWEBBROWSERPERSISTDOCUMENT \
  NS_IMETHOD GetIsClosed(bool *aIsClosed) override; \
  NS_IMETHOD GetIsPrivate(bool *aIsPrivate) override; \
  NS_IMETHOD GetDocumentURI(nsACString& aDocumentURI) override; \
  NS_IMETHOD GetBaseURI(nsACString& aBaseURI) override; \
  NS_IMETHOD GetContentType(nsACString& aContentType) override; \
  NS_IMETHOD GetCharacterSet(nsACString& aCharacterSet) override; \
  NS_IMETHOD GetTitle(nsAString& aTitle) override; \
  NS_IMETHOD GetReferrerInfo(nsIReferrerInfo **aReferrerInfo) override; \
  NS_IMETHOD GetCookieJarSettings(nsICookieJarSettings **aCookieJarSettings) override; \
  NS_IMETHOD GetContentDisposition(nsAString& aContentDisposition) override; \
  NS_IMETHOD GetPostData(nsIInputStream **aPostData) override; \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override; \
  using nsIWebBrowserPersistDocument::GetCacheKey; \
  NS_IMETHOD GetCacheKey(uint32_t *aCacheKey) override; \
  NS_IMETHOD GetPersistFlags(uint32_t *aPersistFlags) override; \
  NS_IMETHOD SetPersistFlags(uint32_t aPersistFlags) override; \
  NS_IMETHOD ReadResources(nsIWebBrowserPersistResourceVisitor *aVisitor) override; \
  NS_IMETHOD WriteContent(nsIOutputStream *aStream, nsIWebBrowserPersistURIMap *aURIMap, const nsACString& aRequestedContentType, uint32_t aEncoderFlags, uint32_t aWrapColumn, nsIWebBrowserPersistWriteCompletion *aCompletion) override; \
  virtual already_AddRefed<nsISHEntry> GetHistory(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWEBBROWSERPERSISTDOCUMENT \
  nsresult GetIsClosed(bool *aIsClosed); \
  nsresult GetIsPrivate(bool *aIsPrivate); \
  nsresult GetDocumentURI(nsACString& aDocumentURI); \
  nsresult GetBaseURI(nsACString& aBaseURI); \
  nsresult GetContentType(nsACString& aContentType); \
  nsresult GetCharacterSet(nsACString& aCharacterSet); \
  nsresult GetTitle(nsAString& aTitle); \
  nsresult GetReferrerInfo(nsIReferrerInfo **aReferrerInfo); \
  nsresult GetCookieJarSettings(nsICookieJarSettings **aCookieJarSettings); \
  nsresult GetContentDisposition(nsAString& aContentDisposition); \
  nsresult GetPostData(nsIInputStream **aPostData); \
  nsresult GetPrincipal(nsIPrincipal **aPrincipal); \
  using nsIWebBrowserPersistDocument::GetCacheKey; \
  nsresult GetCacheKey(uint32_t *aCacheKey); \
  nsresult GetPersistFlags(uint32_t *aPersistFlags); \
  nsresult SetPersistFlags(uint32_t aPersistFlags); \
  nsresult ReadResources(nsIWebBrowserPersistResourceVisitor *aVisitor); \
  nsresult WriteContent(nsIOutputStream *aStream, nsIWebBrowserPersistURIMap *aURIMap, const nsACString& aRequestedContentType, uint32_t aEncoderFlags, uint32_t aWrapColumn, nsIWebBrowserPersistWriteCompletion *aCompletion); \
  already_AddRefed<nsISHEntry> GetHistory(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWEBBROWSERPERSISTDOCUMENT(_to) \
  NS_IMETHOD GetIsClosed(bool *aIsClosed) override { return _to GetIsClosed(aIsClosed); } \
  NS_IMETHOD GetIsPrivate(bool *aIsPrivate) override { return _to GetIsPrivate(aIsPrivate); } \
  NS_IMETHOD GetDocumentURI(nsACString& aDocumentURI) override { return _to GetDocumentURI(aDocumentURI); } \
  NS_IMETHOD GetBaseURI(nsACString& aBaseURI) override { return _to GetBaseURI(aBaseURI); } \
  NS_IMETHOD GetContentType(nsACString& aContentType) override { return _to GetContentType(aContentType); } \
  NS_IMETHOD GetCharacterSet(nsACString& aCharacterSet) override { return _to GetCharacterSet(aCharacterSet); } \
  NS_IMETHOD GetTitle(nsAString& aTitle) override { return _to GetTitle(aTitle); } \
  NS_IMETHOD GetReferrerInfo(nsIReferrerInfo **aReferrerInfo) override { return _to GetReferrerInfo(aReferrerInfo); } \
  NS_IMETHOD GetCookieJarSettings(nsICookieJarSettings **aCookieJarSettings) override { return _to GetCookieJarSettings(aCookieJarSettings); } \
  NS_IMETHOD GetContentDisposition(nsAString& aContentDisposition) override { return _to GetContentDisposition(aContentDisposition); } \
  NS_IMETHOD GetPostData(nsIInputStream **aPostData) override { return _to GetPostData(aPostData); } \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override { return _to GetPrincipal(aPrincipal); } \
  using nsIWebBrowserPersistDocument::GetCacheKey; \
  NS_IMETHOD GetCacheKey(uint32_t *aCacheKey) override { return _to GetCacheKey(aCacheKey); } \
  NS_IMETHOD GetPersistFlags(uint32_t *aPersistFlags) override { return _to GetPersistFlags(aPersistFlags); } \
  NS_IMETHOD SetPersistFlags(uint32_t aPersistFlags) override { return _to SetPersistFlags(aPersistFlags); } \
  NS_IMETHOD ReadResources(nsIWebBrowserPersistResourceVisitor *aVisitor) override { return _to ReadResources(aVisitor); } \
  NS_IMETHOD WriteContent(nsIOutputStream *aStream, nsIWebBrowserPersistURIMap *aURIMap, const nsACString& aRequestedContentType, uint32_t aEncoderFlags, uint32_t aWrapColumn, nsIWebBrowserPersistWriteCompletion *aCompletion) override { return _to WriteContent(aStream, aURIMap, aRequestedContentType, aEncoderFlags, aWrapColumn, aCompletion); } \
  virtual already_AddRefed<nsISHEntry> GetHistory(void) override { return _to GetHistory(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWEBBROWSERPERSISTDOCUMENT(_to) \
  NS_IMETHOD GetIsClosed(bool *aIsClosed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsClosed(aIsClosed); } \
  NS_IMETHOD GetIsPrivate(bool *aIsPrivate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsPrivate(aIsPrivate); } \
  NS_IMETHOD GetDocumentURI(nsACString& aDocumentURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDocumentURI(aDocumentURI); } \
  NS_IMETHOD GetBaseURI(nsACString& aBaseURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBaseURI(aBaseURI); } \
  NS_IMETHOD GetContentType(nsACString& aContentType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentType(aContentType); } \
  NS_IMETHOD GetCharacterSet(nsACString& aCharacterSet) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCharacterSet(aCharacterSet); } \
  NS_IMETHOD GetTitle(nsAString& aTitle) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTitle(aTitle); } \
  NS_IMETHOD GetReferrerInfo(nsIReferrerInfo **aReferrerInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetReferrerInfo(aReferrerInfo); } \
  NS_IMETHOD GetCookieJarSettings(nsICookieJarSettings **aCookieJarSettings) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCookieJarSettings(aCookieJarSettings); } \
  NS_IMETHOD GetContentDisposition(nsAString& aContentDisposition) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentDisposition(aContentDisposition); } \
  NS_IMETHOD GetPostData(nsIInputStream **aPostData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPostData(aPostData); } \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrincipal(aPrincipal); } \
  NS_IMETHOD GetCacheKey(uint32_t *aCacheKey) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCacheKey(aCacheKey); } \
  NS_IMETHOD GetPersistFlags(uint32_t *aPersistFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPersistFlags(aPersistFlags); } \
  NS_IMETHOD SetPersistFlags(uint32_t aPersistFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPersistFlags(aPersistFlags); } \
  NS_IMETHOD ReadResources(nsIWebBrowserPersistResourceVisitor *aVisitor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReadResources(aVisitor); } \
  NS_IMETHOD WriteContent(nsIOutputStream *aStream, nsIWebBrowserPersistURIMap *aURIMap, const nsACString& aRequestedContentType, uint32_t aEncoderFlags, uint32_t aWrapColumn, nsIWebBrowserPersistWriteCompletion *aCompletion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WriteContent(aStream, aURIMap, aRequestedContentType, aEncoderFlags, aWrapColumn, aCompletion); } \
  virtual already_AddRefed<nsISHEntry> GetHistory(void) override; 


/* starting interface:    nsIWebBrowserPersistResourceVisitor */
#define NS_IWEBBROWSERPERSISTRESOURCEVISITOR_IID_STR "8ce37706-b7d3-481a-be68-54f174fc0d0a"

#define NS_IWEBBROWSERPERSISTRESOURCEVISITOR_IID \
  {0x8ce37706, 0xb7d3, 0x481a, \
    { 0xbe, 0x68, 0x54, 0xf1, 0x74, 0xfc, 0x0d, 0x0a }}

class NS_NO_VTABLE nsIWebBrowserPersistResourceVisitor : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWEBBROWSERPERSISTRESOURCEVISITOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWebBrowserPersistResourceVisitor;

  /* void visitResource (in nsIWebBrowserPersistDocument aDocument, in AUTF8String aURI, in nsContentPolicyType aContentPolicyType); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD VisitResource(nsIWebBrowserPersistDocument *aDocument, const nsACString& aURI, nsContentPolicyType aContentPolicyType) = 0;

  /* void visitDocument (in nsIWebBrowserPersistDocument aDocument, in nsIWebBrowserPersistDocument aSubDocument); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD VisitDocument(nsIWebBrowserPersistDocument *aDocument, nsIWebBrowserPersistDocument *aSubDocument) = 0;

  /* void visitBrowsingContext (in nsIWebBrowserPersistDocument aDocument, in BrowsingContext aContext); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD VisitBrowsingContext(nsIWebBrowserPersistDocument *aDocument, mozilla::dom::BrowsingContext *aContext) = 0;

  /* void endVisit (in nsIWebBrowserPersistDocument aDocument, in nsresult aStatus); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EndVisit(nsIWebBrowserPersistDocument *aDocument, nsresult aStatus) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWebBrowserPersistResourceVisitor, NS_IWEBBROWSERPERSISTRESOURCEVISITOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWEBBROWSERPERSISTRESOURCEVISITOR \
  NS_IMETHOD VisitResource(nsIWebBrowserPersistDocument *aDocument, const nsACString& aURI, nsContentPolicyType aContentPolicyType) override; \
  NS_IMETHOD VisitDocument(nsIWebBrowserPersistDocument *aDocument, nsIWebBrowserPersistDocument *aSubDocument) override; \
  NS_IMETHOD VisitBrowsingContext(nsIWebBrowserPersistDocument *aDocument, mozilla::dom::BrowsingContext *aContext) override; \
  NS_IMETHOD EndVisit(nsIWebBrowserPersistDocument *aDocument, nsresult aStatus) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWEBBROWSERPERSISTRESOURCEVISITOR \
  nsresult VisitResource(nsIWebBrowserPersistDocument *aDocument, const nsACString& aURI, nsContentPolicyType aContentPolicyType); \
  nsresult VisitDocument(nsIWebBrowserPersistDocument *aDocument, nsIWebBrowserPersistDocument *aSubDocument); \
  nsresult VisitBrowsingContext(nsIWebBrowserPersistDocument *aDocument, mozilla::dom::BrowsingContext *aContext); \
  nsresult EndVisit(nsIWebBrowserPersistDocument *aDocument, nsresult aStatus); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWEBBROWSERPERSISTRESOURCEVISITOR(_to) \
  NS_IMETHOD VisitResource(nsIWebBrowserPersistDocument *aDocument, const nsACString& aURI, nsContentPolicyType aContentPolicyType) override { return _to VisitResource(aDocument, aURI, aContentPolicyType); } \
  NS_IMETHOD VisitDocument(nsIWebBrowserPersistDocument *aDocument, nsIWebBrowserPersistDocument *aSubDocument) override { return _to VisitDocument(aDocument, aSubDocument); } \
  NS_IMETHOD VisitBrowsingContext(nsIWebBrowserPersistDocument *aDocument, mozilla::dom::BrowsingContext *aContext) override { return _to VisitBrowsingContext(aDocument, aContext); } \
  NS_IMETHOD EndVisit(nsIWebBrowserPersistDocument *aDocument, nsresult aStatus) override { return _to EndVisit(aDocument, aStatus); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWEBBROWSERPERSISTRESOURCEVISITOR(_to) \
  NS_IMETHOD VisitResource(nsIWebBrowserPersistDocument *aDocument, const nsACString& aURI, nsContentPolicyType aContentPolicyType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->VisitResource(aDocument, aURI, aContentPolicyType); } \
  NS_IMETHOD VisitDocument(nsIWebBrowserPersistDocument *aDocument, nsIWebBrowserPersistDocument *aSubDocument) override { return !_to ? NS_ERROR_NULL_POINTER : _to->VisitDocument(aDocument, aSubDocument); } \
  NS_IMETHOD VisitBrowsingContext(nsIWebBrowserPersistDocument *aDocument, mozilla::dom::BrowsingContext *aContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->VisitBrowsingContext(aDocument, aContext); } \
  NS_IMETHOD EndVisit(nsIWebBrowserPersistDocument *aDocument, nsresult aStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EndVisit(aDocument, aStatus); } 


/* starting interface:    nsIWebBrowserPersistWriteCompletion */
#define NS_IWEBBROWSERPERSISTWRITECOMPLETION_IID_STR "a07e6892-38ae-4207-8340-7fa6ec446ed6"

#define NS_IWEBBROWSERPERSISTWRITECOMPLETION_IID \
  {0xa07e6892, 0x38ae, 0x4207, \
    { 0x83, 0x40, 0x7f, 0xa6, 0xec, 0x44, 0x6e, 0xd6 }}

class NS_NO_VTABLE nsIWebBrowserPersistWriteCompletion : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWEBBROWSERPERSISTWRITECOMPLETION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWebBrowserPersistWriteCompletion;

  /* void onFinish (in nsIWebBrowserPersistDocument aDocument, in nsIOutputStream aStream, in ACString aContentType, in nsresult aStatus); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnFinish(nsIWebBrowserPersistDocument *aDocument, nsIOutputStream *aStream, const nsACString& aContentType, nsresult aStatus) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWebBrowserPersistWriteCompletion, NS_IWEBBROWSERPERSISTWRITECOMPLETION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWEBBROWSERPERSISTWRITECOMPLETION \
  NS_IMETHOD OnFinish(nsIWebBrowserPersistDocument *aDocument, nsIOutputStream *aStream, const nsACString& aContentType, nsresult aStatus) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWEBBROWSERPERSISTWRITECOMPLETION \
  nsresult OnFinish(nsIWebBrowserPersistDocument *aDocument, nsIOutputStream *aStream, const nsACString& aContentType, nsresult aStatus); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWEBBROWSERPERSISTWRITECOMPLETION(_to) \
  NS_IMETHOD OnFinish(nsIWebBrowserPersistDocument *aDocument, nsIOutputStream *aStream, const nsACString& aContentType, nsresult aStatus) override { return _to OnFinish(aDocument, aStream, aContentType, aStatus); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWEBBROWSERPERSISTWRITECOMPLETION(_to) \
  NS_IMETHOD OnFinish(nsIWebBrowserPersistDocument *aDocument, nsIOutputStream *aStream, const nsACString& aContentType, nsresult aStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnFinish(aDocument, aStream, aContentType, aStatus); } 


/* starting interface:    nsIWebBrowserPersistDocumentReceiver */
#define NS_IWEBBROWSERPERSISTDOCUMENTRECEIVER_IID_STR "321e3174-594f-4036-b7be-791b821bd376"

#define NS_IWEBBROWSERPERSISTDOCUMENTRECEIVER_IID \
  {0x321e3174, 0x594f, 0x4036, \
    { 0xb7, 0xbe, 0x79, 0x1b, 0x82, 0x1b, 0xd3, 0x76 }}

class NS_NO_VTABLE nsIWebBrowserPersistDocumentReceiver : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWEBBROWSERPERSISTDOCUMENTRECEIVER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWebBrowserPersistDocumentReceiver;

  /* void onDocumentReady (in nsIWebBrowserPersistDocument aDocument); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnDocumentReady(nsIWebBrowserPersistDocument *aDocument) = 0;

  /* void onError (in nsresult aFailure); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnError(nsresult aFailure) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWebBrowserPersistDocumentReceiver, NS_IWEBBROWSERPERSISTDOCUMENTRECEIVER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWEBBROWSERPERSISTDOCUMENTRECEIVER \
  NS_IMETHOD OnDocumentReady(nsIWebBrowserPersistDocument *aDocument) override; \
  NS_IMETHOD OnError(nsresult aFailure) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWEBBROWSERPERSISTDOCUMENTRECEIVER \
  nsresult OnDocumentReady(nsIWebBrowserPersistDocument *aDocument); \
  nsresult OnError(nsresult aFailure); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWEBBROWSERPERSISTDOCUMENTRECEIVER(_to) \
  NS_IMETHOD OnDocumentReady(nsIWebBrowserPersistDocument *aDocument) override { return _to OnDocumentReady(aDocument); } \
  NS_IMETHOD OnError(nsresult aFailure) override { return _to OnError(aFailure); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWEBBROWSERPERSISTDOCUMENTRECEIVER(_to) \
  NS_IMETHOD OnDocumentReady(nsIWebBrowserPersistDocument *aDocument) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnDocumentReady(aDocument); } \
  NS_IMETHOD OnError(nsresult aFailure) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnError(aFailure); } 


#endif /* __gen_nsIWebBrowserPersistDocument_h__ */
