/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIURIFixup.idl
 */

#ifndef __gen_nsIURIFixup_h__
#define __gen_nsIURIFixup_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsIInputStream; /* forward declaration */

namespace mozilla {
namespace dom {
class BrowsingContext; /* webidl BrowsingContext */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIURIFixupInfo */
#define NS_IURIFIXUPINFO_IID_STR "4819f183-b532-4932-ac09-b309cd853be7"

#define NS_IURIFIXUPINFO_IID \
  {0x4819f183, 0xb532, 0x4932, \
    { 0xac, 0x09, 0xb3, 0x09, 0xcd, 0x85, 0x3b, 0xe7 }}

class NS_NO_VTABLE nsIURIFixupInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURIFIXUPINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIURIFixupInfo;

  /* attribute BrowsingContext consumer; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetConsumer(mozilla::dom::BrowsingContext **aConsumer) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetConsumer(mozilla::dom::BrowsingContext *aConsumer) = 0;

  /* attribute nsIURI preferredURI; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPreferredURI(nsIURI **aPreferredURI) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPreferredURI(nsIURI *aPreferredURI) = 0;

  /* attribute nsIURI fixedURI; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFixedURI(nsIURI **aFixedURI) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetFixedURI(nsIURI *aFixedURI) = 0;

  /* attribute AString keywordProviderName; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetKeywordProviderName(nsAString& aKeywordProviderName) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetKeywordProviderName(const nsAString& aKeywordProviderName) = 0;

  /* attribute AString keywordAsSent; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetKeywordAsSent(nsAString& aKeywordAsSent) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetKeywordAsSent(const nsAString& aKeywordAsSent) = 0;

  /* attribute boolean fixupChangedProtocol; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFixupChangedProtocol(bool *aFixupChangedProtocol) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetFixupChangedProtocol(bool aFixupChangedProtocol) = 0;

  /* attribute boolean fixupCreatedAlternateURI; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFixupCreatedAlternateURI(bool *aFixupCreatedAlternateURI) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetFixupCreatedAlternateURI(bool aFixupCreatedAlternateURI) = 0;

  /* attribute AUTF8String originalInput; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOriginalInput(nsACString& aOriginalInput) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetOriginalInput(const nsACString& aOriginalInput) = 0;

  /* attribute nsIInputStream postData; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPostData(nsIInputStream **aPostData) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPostData(nsIInputStream *aPostData) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIURIFixupInfo, NS_IURIFIXUPINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURIFIXUPINFO \
  NS_IMETHOD GetConsumer(mozilla::dom::BrowsingContext **aConsumer) override; \
  NS_IMETHOD SetConsumer(mozilla::dom::BrowsingContext *aConsumer) override; \
  NS_IMETHOD GetPreferredURI(nsIURI **aPreferredURI) override; \
  NS_IMETHOD SetPreferredURI(nsIURI *aPreferredURI) override; \
  NS_IMETHOD GetFixedURI(nsIURI **aFixedURI) override; \
  NS_IMETHOD SetFixedURI(nsIURI *aFixedURI) override; \
  NS_IMETHOD GetKeywordProviderName(nsAString& aKeywordProviderName) override; \
  NS_IMETHOD SetKeywordProviderName(const nsAString& aKeywordProviderName) override; \
  NS_IMETHOD GetKeywordAsSent(nsAString& aKeywordAsSent) override; \
  NS_IMETHOD SetKeywordAsSent(const nsAString& aKeywordAsSent) override; \
  NS_IMETHOD GetFixupChangedProtocol(bool *aFixupChangedProtocol) override; \
  NS_IMETHOD SetFixupChangedProtocol(bool aFixupChangedProtocol) override; \
  NS_IMETHOD GetFixupCreatedAlternateURI(bool *aFixupCreatedAlternateURI) override; \
  NS_IMETHOD SetFixupCreatedAlternateURI(bool aFixupCreatedAlternateURI) override; \
  NS_IMETHOD GetOriginalInput(nsACString& aOriginalInput) override; \
  NS_IMETHOD SetOriginalInput(const nsACString& aOriginalInput) override; \
  NS_IMETHOD GetPostData(nsIInputStream **aPostData) override; \
  NS_IMETHOD SetPostData(nsIInputStream *aPostData) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURIFIXUPINFO \
  nsresult GetConsumer(mozilla::dom::BrowsingContext **aConsumer); \
  nsresult SetConsumer(mozilla::dom::BrowsingContext *aConsumer); \
  nsresult GetPreferredURI(nsIURI **aPreferredURI); \
  nsresult SetPreferredURI(nsIURI *aPreferredURI); \
  nsresult GetFixedURI(nsIURI **aFixedURI); \
  nsresult SetFixedURI(nsIURI *aFixedURI); \
  nsresult GetKeywordProviderName(nsAString& aKeywordProviderName); \
  nsresult SetKeywordProviderName(const nsAString& aKeywordProviderName); \
  nsresult GetKeywordAsSent(nsAString& aKeywordAsSent); \
  nsresult SetKeywordAsSent(const nsAString& aKeywordAsSent); \
  nsresult GetFixupChangedProtocol(bool *aFixupChangedProtocol); \
  nsresult SetFixupChangedProtocol(bool aFixupChangedProtocol); \
  nsresult GetFixupCreatedAlternateURI(bool *aFixupCreatedAlternateURI); \
  nsresult SetFixupCreatedAlternateURI(bool aFixupCreatedAlternateURI); \
  nsresult GetOriginalInput(nsACString& aOriginalInput); \
  nsresult SetOriginalInput(const nsACString& aOriginalInput); \
  nsresult GetPostData(nsIInputStream **aPostData); \
  nsresult SetPostData(nsIInputStream *aPostData); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURIFIXUPINFO(_to) \
  NS_IMETHOD GetConsumer(mozilla::dom::BrowsingContext **aConsumer) override { return _to GetConsumer(aConsumer); } \
  NS_IMETHOD SetConsumer(mozilla::dom::BrowsingContext *aConsumer) override { return _to SetConsumer(aConsumer); } \
  NS_IMETHOD GetPreferredURI(nsIURI **aPreferredURI) override { return _to GetPreferredURI(aPreferredURI); } \
  NS_IMETHOD SetPreferredURI(nsIURI *aPreferredURI) override { return _to SetPreferredURI(aPreferredURI); } \
  NS_IMETHOD GetFixedURI(nsIURI **aFixedURI) override { return _to GetFixedURI(aFixedURI); } \
  NS_IMETHOD SetFixedURI(nsIURI *aFixedURI) override { return _to SetFixedURI(aFixedURI); } \
  NS_IMETHOD GetKeywordProviderName(nsAString& aKeywordProviderName) override { return _to GetKeywordProviderName(aKeywordProviderName); } \
  NS_IMETHOD SetKeywordProviderName(const nsAString& aKeywordProviderName) override { return _to SetKeywordProviderName(aKeywordProviderName); } \
  NS_IMETHOD GetKeywordAsSent(nsAString& aKeywordAsSent) override { return _to GetKeywordAsSent(aKeywordAsSent); } \
  NS_IMETHOD SetKeywordAsSent(const nsAString& aKeywordAsSent) override { return _to SetKeywordAsSent(aKeywordAsSent); } \
  NS_IMETHOD GetFixupChangedProtocol(bool *aFixupChangedProtocol) override { return _to GetFixupChangedProtocol(aFixupChangedProtocol); } \
  NS_IMETHOD SetFixupChangedProtocol(bool aFixupChangedProtocol) override { return _to SetFixupChangedProtocol(aFixupChangedProtocol); } \
  NS_IMETHOD GetFixupCreatedAlternateURI(bool *aFixupCreatedAlternateURI) override { return _to GetFixupCreatedAlternateURI(aFixupCreatedAlternateURI); } \
  NS_IMETHOD SetFixupCreatedAlternateURI(bool aFixupCreatedAlternateURI) override { return _to SetFixupCreatedAlternateURI(aFixupCreatedAlternateURI); } \
  NS_IMETHOD GetOriginalInput(nsACString& aOriginalInput) override { return _to GetOriginalInput(aOriginalInput); } \
  NS_IMETHOD SetOriginalInput(const nsACString& aOriginalInput) override { return _to SetOriginalInput(aOriginalInput); } \
  NS_IMETHOD GetPostData(nsIInputStream **aPostData) override { return _to GetPostData(aPostData); } \
  NS_IMETHOD SetPostData(nsIInputStream *aPostData) override { return _to SetPostData(aPostData); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURIFIXUPINFO(_to) \
  NS_IMETHOD GetConsumer(mozilla::dom::BrowsingContext **aConsumer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetConsumer(aConsumer); } \
  NS_IMETHOD SetConsumer(mozilla::dom::BrowsingContext *aConsumer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetConsumer(aConsumer); } \
  NS_IMETHOD GetPreferredURI(nsIURI **aPreferredURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPreferredURI(aPreferredURI); } \
  NS_IMETHOD SetPreferredURI(nsIURI *aPreferredURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPreferredURI(aPreferredURI); } \
  NS_IMETHOD GetFixedURI(nsIURI **aFixedURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFixedURI(aFixedURI); } \
  NS_IMETHOD SetFixedURI(nsIURI *aFixedURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFixedURI(aFixedURI); } \
  NS_IMETHOD GetKeywordProviderName(nsAString& aKeywordProviderName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetKeywordProviderName(aKeywordProviderName); } \
  NS_IMETHOD SetKeywordProviderName(const nsAString& aKeywordProviderName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetKeywordProviderName(aKeywordProviderName); } \
  NS_IMETHOD GetKeywordAsSent(nsAString& aKeywordAsSent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetKeywordAsSent(aKeywordAsSent); } \
  NS_IMETHOD SetKeywordAsSent(const nsAString& aKeywordAsSent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetKeywordAsSent(aKeywordAsSent); } \
  NS_IMETHOD GetFixupChangedProtocol(bool *aFixupChangedProtocol) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFixupChangedProtocol(aFixupChangedProtocol); } \
  NS_IMETHOD SetFixupChangedProtocol(bool aFixupChangedProtocol) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFixupChangedProtocol(aFixupChangedProtocol); } \
  NS_IMETHOD GetFixupCreatedAlternateURI(bool *aFixupCreatedAlternateURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFixupCreatedAlternateURI(aFixupCreatedAlternateURI); } \
  NS_IMETHOD SetFixupCreatedAlternateURI(bool aFixupCreatedAlternateURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFixupCreatedAlternateURI(aFixupCreatedAlternateURI); } \
  NS_IMETHOD GetOriginalInput(nsACString& aOriginalInput) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOriginalInput(aOriginalInput); } \
  NS_IMETHOD SetOriginalInput(const nsACString& aOriginalInput) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetOriginalInput(aOriginalInput); } \
  NS_IMETHOD GetPostData(nsIInputStream **aPostData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPostData(aPostData); } \
  NS_IMETHOD SetPostData(nsIInputStream *aPostData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPostData(aPostData); } 


/* starting interface:    nsIURIFixup */
#define NS_IURIFIXUP_IID_STR "1da7e9d4-620b-4949-849a-1cd6077b1b2d"

#define NS_IURIFIXUP_IID \
  {0x1da7e9d4, 0x620b, 0x4949, \
    { 0x84, 0x9a, 0x1c, 0xd6, 0x07, 0x7b, 0x1b, 0x2d }}

class NS_NO_VTABLE nsIURIFixup : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURIFIXUP_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIURIFixup;

  enum {
    FIXUP_FLAG_NONE = 0U,
    FIXUP_FLAG_ALLOW_KEYWORD_LOOKUP = 1U,
    FIXUP_FLAGS_MAKE_ALTERNATE_URI = 2U,
    FIXUP_FLAG_PRIVATE_CONTEXT = 4U,
    FIXUP_FLAG_FIX_SCHEME_TYPOS = 8U
  };

  /* nsIURIFixupInfo getFixupURIInfo (in AUTF8String aURIText, [optional] in unsigned long aFixupFlags); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFixupURIInfo(const nsACString& aURIText, uint32_t aFixupFlags, nsIURIFixupInfo **_retval) = 0;

  /* unsigned long webNavigationFlagsToFixupFlags (in AUTF8String aURIText, in unsigned long aDocShellFlags); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD WebNavigationFlagsToFixupFlags(const nsACString& aURIText, uint32_t aDocShellFlags, uint32_t *_retval) = 0;

  /* nsIURIFixupInfo keywordToURI (in AUTF8String aKeyword, [optional] in boolean aIsPrivateContext); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD KeywordToURI(const nsACString& aKeyword, bool aIsPrivateContext, nsIURIFixupInfo **_retval) = 0;

  /* bool isDomainKnown (in AUTF8String aDomain); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsDomainKnown(const nsACString& aDomain, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIURIFixup, NS_IURIFIXUP_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURIFIXUP \
  NS_IMETHOD GetFixupURIInfo(const nsACString& aURIText, uint32_t aFixupFlags, nsIURIFixupInfo **_retval) override; \
  NS_IMETHOD WebNavigationFlagsToFixupFlags(const nsACString& aURIText, uint32_t aDocShellFlags, uint32_t *_retval) override; \
  NS_IMETHOD KeywordToURI(const nsACString& aKeyword, bool aIsPrivateContext, nsIURIFixupInfo **_retval) override; \
  NS_IMETHOD IsDomainKnown(const nsACString& aDomain, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURIFIXUP \
  nsresult GetFixupURIInfo(const nsACString& aURIText, uint32_t aFixupFlags, nsIURIFixupInfo **_retval); \
  nsresult WebNavigationFlagsToFixupFlags(const nsACString& aURIText, uint32_t aDocShellFlags, uint32_t *_retval); \
  nsresult KeywordToURI(const nsACString& aKeyword, bool aIsPrivateContext, nsIURIFixupInfo **_retval); \
  nsresult IsDomainKnown(const nsACString& aDomain, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURIFIXUP(_to) \
  NS_IMETHOD GetFixupURIInfo(const nsACString& aURIText, uint32_t aFixupFlags, nsIURIFixupInfo **_retval) override { return _to GetFixupURIInfo(aURIText, aFixupFlags, _retval); } \
  NS_IMETHOD WebNavigationFlagsToFixupFlags(const nsACString& aURIText, uint32_t aDocShellFlags, uint32_t *_retval) override { return _to WebNavigationFlagsToFixupFlags(aURIText, aDocShellFlags, _retval); } \
  NS_IMETHOD KeywordToURI(const nsACString& aKeyword, bool aIsPrivateContext, nsIURIFixupInfo **_retval) override { return _to KeywordToURI(aKeyword, aIsPrivateContext, _retval); } \
  NS_IMETHOD IsDomainKnown(const nsACString& aDomain, bool *_retval) override { return _to IsDomainKnown(aDomain, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURIFIXUP(_to) \
  NS_IMETHOD GetFixupURIInfo(const nsACString& aURIText, uint32_t aFixupFlags, nsIURIFixupInfo **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFixupURIInfo(aURIText, aFixupFlags, _retval); } \
  NS_IMETHOD WebNavigationFlagsToFixupFlags(const nsACString& aURIText, uint32_t aDocShellFlags, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WebNavigationFlagsToFixupFlags(aURIText, aDocShellFlags, _retval); } \
  NS_IMETHOD KeywordToURI(const nsACString& aKeyword, bool aIsPrivateContext, nsIURIFixupInfo **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->KeywordToURI(aKeyword, aIsPrivateContext, _retval); } \
  NS_IMETHOD IsDomainKnown(const nsACString& aDomain, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsDomainKnown(aDomain, _retval); } 


#endif /* __gen_nsIURIFixup_h__ */
