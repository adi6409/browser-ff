/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIWebNavigation.idl
 */

#ifndef __gen_nsIWebNavigation_h__
#define __gen_nsIWebNavigation_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIInputStream; /* forward declaration */

class nsISHistory; /* forward declaration */

class nsIURI; /* forward declaration */

class nsIPrincipal; /* forward declaration */

class nsIChildSHistory; /* forward declaration */

namespace mozilla {
namespace dom {
class Document; /* webidl Document */
} // namespace dom
} // namespace mozilla

#include "mozilla/dom/ChildSHistory.h"
namespace mozilla {
namespace dom {
struct LoadURIOptions;
} // namespace dom
} // namespace mozilla

/* starting interface:    nsIWebNavigation */
#define NS_IWEBNAVIGATION_IID_STR "3ade79d4-8cb9-4952-b18d-4f9b63ca0d31"

#define NS_IWEBNAVIGATION_IID \
  {0x3ade79d4, 0x8cb9, 0x4952, \
    { 0xb1, 0x8d, 0x4f, 0x9b, 0x63, 0xca, 0x0d, 0x31 }}

class nsIWebNavigation : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWEBNAVIGATION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWebNavigation;

  /* readonly attribute boolean canGoBack; */
  NS_IMETHOD GetCanGoBack(bool *aCanGoBack) = 0;

  /* readonly attribute boolean canGoForward; */
  NS_IMETHOD GetCanGoForward(bool *aCanGoForward) = 0;

  /* void goBack ([optional] in boolean aRequireUserInteraction); */
  NS_IMETHOD GoBack(bool aRequireUserInteraction) = 0;

  /* void goForward ([optional] in boolean aRequireUserInteraction); */
  NS_IMETHOD GoForward(bool aRequireUserInteraction) = 0;

  /* void gotoIndex (in long index); */
  NS_IMETHOD GotoIndex(int32_t index) = 0;

  enum {
    LOAD_FLAGS_MASK = 65535U,
    LOAD_FLAGS_NONE = 0U,
    LOAD_FLAGS_IS_REFRESH = 16U,
    LOAD_FLAGS_IS_LINK = 32U,
    LOAD_FLAGS_BYPASS_HISTORY = 64U,
    LOAD_FLAGS_REPLACE_HISTORY = 128U,
    LOAD_FLAGS_BYPASS_CACHE = 256U,
    LOAD_FLAGS_BYPASS_PROXY = 512U,
    LOAD_FLAGS_CHARSET_CHANGE = 1024U,
    LOAD_FLAGS_STOP_CONTENT = 2048U,
    LOAD_FLAGS_FROM_EXTERNAL = 4096U,
    LOAD_FLAGS_ALLOW_MIXED_CONTENT = 8192U,
    LOAD_FLAGS_FIRST_LOAD = 16384U,
    LOAD_FLAGS_ALLOW_POPUPS = 32768U,
    LOAD_FLAGS_BYPASS_CLASSIFIER = 65536U,
    LOAD_FLAGS_FORCE_ALLOW_COOKIES = 131072U,
    LOAD_FLAGS_DISALLOW_INHERIT_PRINCIPAL = 262144U,
    LOAD_FLAGS_ERROR_LOAD_CHANGES_RV = 524288U,
    LOAD_FLAGS_ALLOW_THIRD_PARTY_FIXUP = 1048576U,
    LOAD_FLAGS_FIXUP_SCHEME_TYPOS = 2097152U,
    LOAD_FLAGS_FORCE_ALLOW_DATA_URI = 4194304U,
    LOAD_FLAGS_IS_REDIRECT = 8388608U,
    LOAD_FLAGS_DISABLE_TRR = 16777216U,
    LOAD_FLAGS_FORCE_TRR = 33554432U,
    LOAD_FLAGS_BYPASS_LOAD_URI_DELEGATE = 67108864U
  };

  /* [binaryname(LoadURIFromScript),implicit_jscontext] void loadURI (in AString aURI, in jsval aLoadURIOptions); */
  NS_IMETHOD LoadURIFromScript(const nsAString& aURI, JS::HandleValue aLoadURIOptions, JSContext* cx) = 0;

  /* [binaryname(LoadURI),nostdcall] void binaryLoadURI (in AString aURI, in LoadURIOptionsRef aLoadURIOptions); */
  virtual nsresult LoadURI(const nsAString& aURI, const mozilla::dom::LoadURIOptions & aLoadURIOptions) = 0;

  /* void reload (in unsigned long aReloadFlags); */
  NS_IMETHOD Reload(uint32_t aReloadFlags) = 0;

  enum {
    STOP_NETWORK = 1U,
    STOP_CONTENT = 2U,
    STOP_ALL = 3U
  };

  /* void stop (in unsigned long aStopFlags); */
  NS_IMETHOD Stop(uint32_t aStopFlags) = 0;

  /* readonly attribute Document document; */
  NS_IMETHOD GetDocument(mozilla::dom::Document **aDocument) = 0;

  /* readonly attribute nsIURI currentURI; */
  NS_IMETHOD GetCurrentURI(nsIURI **aCurrentURI) = 0;

  /* [binaryname(SessionHistoryXPCOM)] readonly attribute nsISupports sessionHistory; */
  NS_IMETHOD GetSessionHistoryXPCOM(nsISupports **aSessionHistory) = 0;

   /**
   * Get the session history object used by this nsIWebNavigation instance.
   * Use this method instead of the XPCOM method when getting the
   * SessionHistory from C++ code.
   */
  already_AddRefed<mozilla::dom::ChildSHistory>
  GetSessionHistory()
  {
    nsCOMPtr<nsISupports> history;
    GetSessionHistoryXPCOM(getter_AddRefs(history));
    return history.forget()
      .downcast<mozilla::dom::ChildSHistory>();
  }
    /* void resumeRedirectedLoad (in unsigned long long aLoadIdentifier, in long aHistoryIndex); */
  NS_IMETHOD ResumeRedirectedLoad(uint64_t aLoadIdentifier, int32_t aHistoryIndex) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWebNavigation, NS_IWEBNAVIGATION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWEBNAVIGATION \
  NS_IMETHOD GetCanGoBack(bool *aCanGoBack) override; \
  NS_IMETHOD GetCanGoForward(bool *aCanGoForward) override; \
  NS_IMETHOD GoBack(bool aRequireUserInteraction) override; \
  NS_IMETHOD GoForward(bool aRequireUserInteraction) override; \
  NS_IMETHOD GotoIndex(int32_t index) override; \
  NS_IMETHOD LoadURIFromScript(const nsAString& aURI, JS::HandleValue aLoadURIOptions, JSContext* cx) override; \
  virtual nsresult LoadURI(const nsAString& aURI, const mozilla::dom::LoadURIOptions & aLoadURIOptions) override; \
  NS_IMETHOD Reload(uint32_t aReloadFlags) override; \
  NS_IMETHOD Stop(uint32_t aStopFlags) override; \
  NS_IMETHOD GetDocument(mozilla::dom::Document **aDocument) override; \
  NS_IMETHOD GetCurrentURI(nsIURI **aCurrentURI) override; \
  NS_IMETHOD GetSessionHistoryXPCOM(nsISupports **aSessionHistory) override; \
  NS_IMETHOD ResumeRedirectedLoad(uint64_t aLoadIdentifier, int32_t aHistoryIndex) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWEBNAVIGATION \
  nsresult GetCanGoBack(bool *aCanGoBack); \
  nsresult GetCanGoForward(bool *aCanGoForward); \
  nsresult GoBack(bool aRequireUserInteraction); \
  nsresult GoForward(bool aRequireUserInteraction); \
  nsresult GotoIndex(int32_t index); \
  nsresult LoadURIFromScript(const nsAString& aURI, JS::HandleValue aLoadURIOptions, JSContext* cx); \
  nsresult LoadURI(const nsAString& aURI, const mozilla::dom::LoadURIOptions & aLoadURIOptions); \
  nsresult Reload(uint32_t aReloadFlags); \
  nsresult Stop(uint32_t aStopFlags); \
  nsresult GetDocument(mozilla::dom::Document **aDocument); \
  nsresult GetCurrentURI(nsIURI **aCurrentURI); \
  nsresult GetSessionHistoryXPCOM(nsISupports **aSessionHistory); \
  nsresult ResumeRedirectedLoad(uint64_t aLoadIdentifier, int32_t aHistoryIndex); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWEBNAVIGATION(_to) \
  NS_IMETHOD GetCanGoBack(bool *aCanGoBack) override { return _to GetCanGoBack(aCanGoBack); } \
  NS_IMETHOD GetCanGoForward(bool *aCanGoForward) override { return _to GetCanGoForward(aCanGoForward); } \
  NS_IMETHOD GoBack(bool aRequireUserInteraction) override { return _to GoBack(aRequireUserInteraction); } \
  NS_IMETHOD GoForward(bool aRequireUserInteraction) override { return _to GoForward(aRequireUserInteraction); } \
  NS_IMETHOD GotoIndex(int32_t index) override { return _to GotoIndex(index); } \
  NS_IMETHOD LoadURIFromScript(const nsAString& aURI, JS::HandleValue aLoadURIOptions, JSContext* cx) override { return _to LoadURIFromScript(aURI, aLoadURIOptions, cx); } \
  virtual nsresult LoadURI(const nsAString& aURI, const mozilla::dom::LoadURIOptions & aLoadURIOptions) override { return _to LoadURI(aURI, aLoadURIOptions); } \
  NS_IMETHOD Reload(uint32_t aReloadFlags) override { return _to Reload(aReloadFlags); } \
  NS_IMETHOD Stop(uint32_t aStopFlags) override { return _to Stop(aStopFlags); } \
  NS_IMETHOD GetDocument(mozilla::dom::Document **aDocument) override { return _to GetDocument(aDocument); } \
  NS_IMETHOD GetCurrentURI(nsIURI **aCurrentURI) override { return _to GetCurrentURI(aCurrentURI); } \
  NS_IMETHOD GetSessionHistoryXPCOM(nsISupports **aSessionHistory) override { return _to GetSessionHistoryXPCOM(aSessionHistory); } \
  NS_IMETHOD ResumeRedirectedLoad(uint64_t aLoadIdentifier, int32_t aHistoryIndex) override { return _to ResumeRedirectedLoad(aLoadIdentifier, aHistoryIndex); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWEBNAVIGATION(_to) \
  NS_IMETHOD GetCanGoBack(bool *aCanGoBack) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCanGoBack(aCanGoBack); } \
  NS_IMETHOD GetCanGoForward(bool *aCanGoForward) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCanGoForward(aCanGoForward); } \
  NS_IMETHOD GoBack(bool aRequireUserInteraction) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GoBack(aRequireUserInteraction); } \
  NS_IMETHOD GoForward(bool aRequireUserInteraction) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GoForward(aRequireUserInteraction); } \
  NS_IMETHOD GotoIndex(int32_t index) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GotoIndex(index); } \
  NS_IMETHOD LoadURIFromScript(const nsAString& aURI, JS::HandleValue aLoadURIOptions, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LoadURIFromScript(aURI, aLoadURIOptions, cx); } \
  virtual nsresult LoadURI(const nsAString& aURI, const mozilla::dom::LoadURIOptions & aLoadURIOptions) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LoadURI(aURI, aLoadURIOptions); } \
  NS_IMETHOD Reload(uint32_t aReloadFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Reload(aReloadFlags); } \
  NS_IMETHOD Stop(uint32_t aStopFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Stop(aStopFlags); } \
  NS_IMETHOD GetDocument(mozilla::dom::Document **aDocument) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDocument(aDocument); } \
  NS_IMETHOD GetCurrentURI(nsIURI **aCurrentURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCurrentURI(aCurrentURI); } \
  NS_IMETHOD GetSessionHistoryXPCOM(nsISupports **aSessionHistory) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSessionHistoryXPCOM(aSessionHistory); } \
  NS_IMETHOD ResumeRedirectedLoad(uint64_t aLoadIdentifier, int32_t aHistoryIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResumeRedirectedLoad(aLoadIdentifier, aHistoryIndex); } 


#endif /* __gen_nsIWebNavigation_h__ */
