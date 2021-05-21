/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/webbrowserpersist/nsIWebBrowserPersist.idl
 */

#ifndef __gen_nsIWebBrowserPersist_h__
#define __gen_nsIWebBrowserPersist_h__


#ifndef __gen_nsICancelable_h__
#include "nsICancelable.h"
#endif

#ifndef __gen_nsIContentPolicy_h__
#include "nsIContentPolicy.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsIInputStream; /* forward declaration */

class nsIWebProgressListener; /* forward declaration */

class nsIFile; /* forward declaration */

class nsIChannel; /* forward declaration */

class nsILoadContext; /* forward declaration */

class nsIPrincipal; /* forward declaration */

class nsIReferrerInfo; /* forward declaration */

class nsICookieJarSettings; /* forward declaration */


/* starting interface:    nsIWebBrowserPersist */
#define NS_IWEBBROWSERPERSIST_IID_STR "8cd752a4-60b1-42c3-a819-65c7a1138a28"

#define NS_IWEBBROWSERPERSIST_IID \
  {0x8cd752a4, 0x60b1, 0x42c3, \
    { 0xa8, 0x19, 0x65, 0xc7, 0xa1, 0x13, 0x8a, 0x28 }}

class NS_NO_VTABLE nsIWebBrowserPersist : public nsICancelable {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWEBBROWSERPERSIST_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWebBrowserPersist;

  enum {
    PERSIST_FLAGS_NONE = 0U,
    PERSIST_FLAGS_FROM_CACHE = 1U,
    PERSIST_FLAGS_BYPASS_CACHE = 2U,
    PERSIST_FLAGS_IGNORE_REDIRECTED_DATA = 4U,
    PERSIST_FLAGS_IGNORE_IFRAMES = 8U,
    PERSIST_FLAGS_NO_CONVERSION = 16U,
    PERSIST_FLAGS_REPLACE_EXISTING_FILES = 32U,
    PERSIST_FLAGS_NO_BASE_TAG_MODIFICATIONS = 64U,
    PERSIST_FLAGS_FIXUP_ORIGINAL_DOM = 128U,
    PERSIST_FLAGS_FIXUP_LINKS_TO_DESTINATION = 256U,
    PERSIST_FLAGS_DONT_FIXUP_LINKS = 512U,
    PERSIST_FLAGS_SERIALIZE_OUTPUT = 1024U,
    PERSIST_FLAGS_DONT_CHANGE_FILENAMES = 2048U,
    PERSIST_FLAGS_FAIL_ON_BROKEN_LINKS = 4096U,
    PERSIST_FLAGS_CLEANUP_ON_FAILURE = 8192U,
    PERSIST_FLAGS_AUTODETECT_APPLY_CONVERSION = 16384U,
    PERSIST_FLAGS_APPEND_TO_FILE = 32768U
  };

  /* attribute unsigned long persistFlags; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPersistFlags(uint32_t *aPersistFlags) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPersistFlags(uint32_t aPersistFlags) = 0;

  enum {
    PERSIST_STATE_READY = 1U,
    PERSIST_STATE_SAVING = 2U,
    PERSIST_STATE_FINISHED = 3U
  };

  /* readonly attribute unsigned long currentState; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCurrentState(uint32_t *aCurrentState) = 0;

  /* readonly attribute nsresult result; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetResult(nsresult *aResult) = 0;

  /* attribute nsIWebProgressListener progressListener; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetProgressListener(nsIWebProgressListener **aProgressListener) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetProgressListener(nsIWebProgressListener *aProgressListener) = 0;

  /* void saveURI (in nsIURI aURI, in nsIPrincipal aTriggeringPrincipal, in unsigned long aCacheKey, in nsIReferrerInfo aReferrerInfo, in nsICookieJarSettings aCookieJarSettings, in nsIInputStream aPostData, in string aExtraHeaders, in nsISupports aFile, in nsContentPolicyType aContentPolicyType, in nsILoadContext aPrivacyContext); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SaveURI(nsIURI *aURI, nsIPrincipal *aTriggeringPrincipal, uint32_t aCacheKey, nsIReferrerInfo *aReferrerInfo, nsICookieJarSettings *aCookieJarSettings, nsIInputStream *aPostData, const char * aExtraHeaders, nsISupports *aFile, nsContentPolicyType aContentPolicyType, nsILoadContext *aPrivacyContext) = 0;

  /* void savePrivacyAwareURI (in nsIURI aURI, in nsIPrincipal aTriggeringPrincipal, in unsigned long aCacheKey, in nsIReferrerInfo aReferrerInfo, in nsICookieJarSettings aCookieJarSettings, in nsIInputStream aPostData, in string aExtraHeaders, in nsISupports aFile, in nsContentPolicyType aContentPolicyType, in boolean aIsPrivate); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SavePrivacyAwareURI(nsIURI *aURI, nsIPrincipal *aTriggeringPrincipal, uint32_t aCacheKey, nsIReferrerInfo *aReferrerInfo, nsICookieJarSettings *aCookieJarSettings, nsIInputStream *aPostData, const char * aExtraHeaders, nsISupports *aFile, nsContentPolicyType aContentPolicyType, bool aIsPrivate) = 0;

  /* void saveChannel (in nsIChannel aChannel, in nsISupports aFile); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SaveChannel(nsIChannel *aChannel, nsISupports *aFile) = 0;

  enum {
    ENCODE_FLAGS_SELECTION_ONLY = 1U,
    ENCODE_FLAGS_FORMATTED = 2U,
    ENCODE_FLAGS_RAW = 4U,
    ENCODE_FLAGS_BODY_ONLY = 8U,
    ENCODE_FLAGS_PREFORMATTED = 16U,
    ENCODE_FLAGS_WRAP = 32U,
    ENCODE_FLAGS_FORMAT_FLOWED = 64U,
    ENCODE_FLAGS_ABSOLUTE_LINKS = 128U,
    ENCODE_FLAGS_CR_LINEBREAKS = 512U,
    ENCODE_FLAGS_LF_LINEBREAKS = 1024U,
    ENCODE_FLAGS_NOSCRIPT_CONTENT = 2048U,
    ENCODE_FLAGS_NOFRAMES_CONTENT = 4096U,
    ENCODE_FLAGS_ENCODE_BASIC_ENTITIES = 8192U
  };

  /* void saveDocument (in nsISupports aDocument, in nsISupports aFile, in nsISupports aDataPath, in string aOutputContentType, in unsigned long aEncodingFlags, in unsigned long aWrapColumn); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SaveDocument(nsISupports *aDocument, nsISupports *aFile, nsISupports *aDataPath, const char * aOutputContentType, uint32_t aEncodingFlags, uint32_t aWrapColumn) = 0;

  /* void cancelSave (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CancelSave(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWebBrowserPersist, NS_IWEBBROWSERPERSIST_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWEBBROWSERPERSIST \
  NS_IMETHOD GetPersistFlags(uint32_t *aPersistFlags) override; \
  NS_IMETHOD SetPersistFlags(uint32_t aPersistFlags) override; \
  NS_IMETHOD GetCurrentState(uint32_t *aCurrentState) override; \
  NS_IMETHOD GetResult(nsresult *aResult) override; \
  NS_IMETHOD GetProgressListener(nsIWebProgressListener **aProgressListener) override; \
  NS_IMETHOD SetProgressListener(nsIWebProgressListener *aProgressListener) override; \
  NS_IMETHOD SaveURI(nsIURI *aURI, nsIPrincipal *aTriggeringPrincipal, uint32_t aCacheKey, nsIReferrerInfo *aReferrerInfo, nsICookieJarSettings *aCookieJarSettings, nsIInputStream *aPostData, const char * aExtraHeaders, nsISupports *aFile, nsContentPolicyType aContentPolicyType, nsILoadContext *aPrivacyContext) override; \
  NS_IMETHOD SavePrivacyAwareURI(nsIURI *aURI, nsIPrincipal *aTriggeringPrincipal, uint32_t aCacheKey, nsIReferrerInfo *aReferrerInfo, nsICookieJarSettings *aCookieJarSettings, nsIInputStream *aPostData, const char * aExtraHeaders, nsISupports *aFile, nsContentPolicyType aContentPolicyType, bool aIsPrivate) override; \
  NS_IMETHOD SaveChannel(nsIChannel *aChannel, nsISupports *aFile) override; \
  NS_IMETHOD SaveDocument(nsISupports *aDocument, nsISupports *aFile, nsISupports *aDataPath, const char * aOutputContentType, uint32_t aEncodingFlags, uint32_t aWrapColumn) override; \
  NS_IMETHOD CancelSave(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWEBBROWSERPERSIST \
  nsresult GetPersistFlags(uint32_t *aPersistFlags); \
  nsresult SetPersistFlags(uint32_t aPersistFlags); \
  nsresult GetCurrentState(uint32_t *aCurrentState); \
  nsresult GetResult(nsresult *aResult); \
  nsresult GetProgressListener(nsIWebProgressListener **aProgressListener); \
  nsresult SetProgressListener(nsIWebProgressListener *aProgressListener); \
  nsresult SaveURI(nsIURI *aURI, nsIPrincipal *aTriggeringPrincipal, uint32_t aCacheKey, nsIReferrerInfo *aReferrerInfo, nsICookieJarSettings *aCookieJarSettings, nsIInputStream *aPostData, const char * aExtraHeaders, nsISupports *aFile, nsContentPolicyType aContentPolicyType, nsILoadContext *aPrivacyContext); \
  nsresult SavePrivacyAwareURI(nsIURI *aURI, nsIPrincipal *aTriggeringPrincipal, uint32_t aCacheKey, nsIReferrerInfo *aReferrerInfo, nsICookieJarSettings *aCookieJarSettings, nsIInputStream *aPostData, const char * aExtraHeaders, nsISupports *aFile, nsContentPolicyType aContentPolicyType, bool aIsPrivate); \
  nsresult SaveChannel(nsIChannel *aChannel, nsISupports *aFile); \
  nsresult SaveDocument(nsISupports *aDocument, nsISupports *aFile, nsISupports *aDataPath, const char * aOutputContentType, uint32_t aEncodingFlags, uint32_t aWrapColumn); \
  nsresult CancelSave(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWEBBROWSERPERSIST(_to) \
  NS_IMETHOD GetPersistFlags(uint32_t *aPersistFlags) override { return _to GetPersistFlags(aPersistFlags); } \
  NS_IMETHOD SetPersistFlags(uint32_t aPersistFlags) override { return _to SetPersistFlags(aPersistFlags); } \
  NS_IMETHOD GetCurrentState(uint32_t *aCurrentState) override { return _to GetCurrentState(aCurrentState); } \
  NS_IMETHOD GetResult(nsresult *aResult) override { return _to GetResult(aResult); } \
  NS_IMETHOD GetProgressListener(nsIWebProgressListener **aProgressListener) override { return _to GetProgressListener(aProgressListener); } \
  NS_IMETHOD SetProgressListener(nsIWebProgressListener *aProgressListener) override { return _to SetProgressListener(aProgressListener); } \
  NS_IMETHOD SaveURI(nsIURI *aURI, nsIPrincipal *aTriggeringPrincipal, uint32_t aCacheKey, nsIReferrerInfo *aReferrerInfo, nsICookieJarSettings *aCookieJarSettings, nsIInputStream *aPostData, const char * aExtraHeaders, nsISupports *aFile, nsContentPolicyType aContentPolicyType, nsILoadContext *aPrivacyContext) override { return _to SaveURI(aURI, aTriggeringPrincipal, aCacheKey, aReferrerInfo, aCookieJarSettings, aPostData, aExtraHeaders, aFile, aContentPolicyType, aPrivacyContext); } \
  NS_IMETHOD SavePrivacyAwareURI(nsIURI *aURI, nsIPrincipal *aTriggeringPrincipal, uint32_t aCacheKey, nsIReferrerInfo *aReferrerInfo, nsICookieJarSettings *aCookieJarSettings, nsIInputStream *aPostData, const char * aExtraHeaders, nsISupports *aFile, nsContentPolicyType aContentPolicyType, bool aIsPrivate) override { return _to SavePrivacyAwareURI(aURI, aTriggeringPrincipal, aCacheKey, aReferrerInfo, aCookieJarSettings, aPostData, aExtraHeaders, aFile, aContentPolicyType, aIsPrivate); } \
  NS_IMETHOD SaveChannel(nsIChannel *aChannel, nsISupports *aFile) override { return _to SaveChannel(aChannel, aFile); } \
  NS_IMETHOD SaveDocument(nsISupports *aDocument, nsISupports *aFile, nsISupports *aDataPath, const char * aOutputContentType, uint32_t aEncodingFlags, uint32_t aWrapColumn) override { return _to SaveDocument(aDocument, aFile, aDataPath, aOutputContentType, aEncodingFlags, aWrapColumn); } \
  NS_IMETHOD CancelSave(void) override { return _to CancelSave(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWEBBROWSERPERSIST(_to) \
  NS_IMETHOD GetPersistFlags(uint32_t *aPersistFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPersistFlags(aPersistFlags); } \
  NS_IMETHOD SetPersistFlags(uint32_t aPersistFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPersistFlags(aPersistFlags); } \
  NS_IMETHOD GetCurrentState(uint32_t *aCurrentState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCurrentState(aCurrentState); } \
  NS_IMETHOD GetResult(nsresult *aResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResult(aResult); } \
  NS_IMETHOD GetProgressListener(nsIWebProgressListener **aProgressListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProgressListener(aProgressListener); } \
  NS_IMETHOD SetProgressListener(nsIWebProgressListener *aProgressListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetProgressListener(aProgressListener); } \
  NS_IMETHOD SaveURI(nsIURI *aURI, nsIPrincipal *aTriggeringPrincipal, uint32_t aCacheKey, nsIReferrerInfo *aReferrerInfo, nsICookieJarSettings *aCookieJarSettings, nsIInputStream *aPostData, const char * aExtraHeaders, nsISupports *aFile, nsContentPolicyType aContentPolicyType, nsILoadContext *aPrivacyContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SaveURI(aURI, aTriggeringPrincipal, aCacheKey, aReferrerInfo, aCookieJarSettings, aPostData, aExtraHeaders, aFile, aContentPolicyType, aPrivacyContext); } \
  NS_IMETHOD SavePrivacyAwareURI(nsIURI *aURI, nsIPrincipal *aTriggeringPrincipal, uint32_t aCacheKey, nsIReferrerInfo *aReferrerInfo, nsICookieJarSettings *aCookieJarSettings, nsIInputStream *aPostData, const char * aExtraHeaders, nsISupports *aFile, nsContentPolicyType aContentPolicyType, bool aIsPrivate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SavePrivacyAwareURI(aURI, aTriggeringPrincipal, aCacheKey, aReferrerInfo, aCookieJarSettings, aPostData, aExtraHeaders, aFile, aContentPolicyType, aIsPrivate); } \
  NS_IMETHOD SaveChannel(nsIChannel *aChannel, nsISupports *aFile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SaveChannel(aChannel, aFile); } \
  NS_IMETHOD SaveDocument(nsISupports *aDocument, nsISupports *aFile, nsISupports *aDataPath, const char * aOutputContentType, uint32_t aEncodingFlags, uint32_t aWrapColumn) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SaveDocument(aDocument, aFile, aDataPath, aOutputContentType, aEncodingFlags, aWrapColumn); } \
  NS_IMETHOD CancelSave(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CancelSave(); } 

// {7E677795-C582-4cd1-9E8D-8271B3474D2A}
#define NS_WEBBROWSERPERSIST_CID \
  { 0x7e677795, 0xc582, 0x4cd1, { 0x9e, 0x8d, 0x82, 0x71, 0xb3, 0x47, 0x4d, 0x2a } }
#define NS_WEBBROWSERPERSIST_CONTRACTID \
  "@mozilla.org/embedding/browser/nsWebBrowserPersist;1"

#endif /* __gen_nsIWebBrowserPersist_h__ */
