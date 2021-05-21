/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIBrowserChild.idl
 */

#ifndef __gen_nsIBrowserChild_h__
#define __gen_nsIBrowserChild_h__


#ifndef __gen_domstubs_h__
#include "domstubs.h"
#endif

#ifndef __gen_nsIDroppedLinkHandler_h__
#include "nsIDroppedLinkHandler.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIWebBrowserChrome3; /* forward declaration */

namespace mozilla {
namespace dom {
class ContentFrameMessageManager; /* webidl ContentFrameMessageManager */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIBrowserChild */
#define NS_IBROWSERCHILD_IID_STR "1fb79c27-e760-4088-b19c-1ce3673ec24e"

#define NS_IBROWSERCHILD_IID \
  {0x1fb79c27, 0xe760, 0x4088, \
    { 0xb1, 0x9c, 0x1c, 0xe3, 0x67, 0x3e, 0xc2, 0x4e }}

class NS_NO_VTABLE nsIBrowserChild : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IBROWSERCHILD_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIBrowserChild;

  /* readonly attribute ContentFrameMessageManager messageManager; */
  NS_IMETHOD GetMessageManager(mozilla::dom::ContentFrameMessageManager **aMessageManager) = 0;

  /* attribute nsIWebBrowserChrome3 webBrowserChrome; */
  NS_IMETHOD GetWebBrowserChrome(nsIWebBrowserChrome3 **aWebBrowserChrome) = 0;
  NS_IMETHOD SetWebBrowserChrome(nsIWebBrowserChrome3 *aWebBrowserChrome) = 0;

  /* [notxpcom] void sendRequestFocus (in boolean canFocus, in CallerType aCallerType); */
  NS_IMETHOD_(void) SendRequestFocus(bool canFocus, mozilla::dom::CallerType aCallerType) = 0;

  /* [noscript] void remoteSizeShellTo (in int32_t width, in int32_t height, in int32_t shellItemWidth, in int32_t shellItemHeight); */
  NS_IMETHOD RemoteSizeShellTo(int32_t width, int32_t height, int32_t shellItemWidth, int32_t shellItemHeight) = 0;

  /* void remoteDropLinks (in Array<nsIDroppedLinkItem> links); */
  NS_IMETHOD RemoteDropLinks(const nsTArray<RefPtr<nsIDroppedLinkItem>>& links) = 0;

  /* readonly attribute uint64_t tabId; */
  NS_IMETHOD GetTabId(uint64_t *aTabId) = 0;

  /* attribute boolean hasSiblings; */
  NS_IMETHOD GetHasSiblings(bool *aHasSiblings) = 0;
  NS_IMETHOD SetHasSiblings(bool aHasSiblings) = 0;

  /* void beginSendingWebProgressEventsToParent (); */
  NS_IMETHOD BeginSendingWebProgressEventsToParent(void) = 0;

  /* void notifyNavigationFinished (); */
  NS_IMETHOD NotifyNavigationFinished(void) = 0;

  /* readonly attribute uint64_t chromeOuterWindowID; */
  NS_IMETHOD GetChromeOuterWindowID(uint64_t *aChromeOuterWindowID) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIBrowserChild, NS_IBROWSERCHILD_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIBROWSERCHILD \
  NS_IMETHOD GetMessageManager(mozilla::dom::ContentFrameMessageManager **aMessageManager) override; \
  NS_IMETHOD GetWebBrowserChrome(nsIWebBrowserChrome3 **aWebBrowserChrome) override; \
  NS_IMETHOD SetWebBrowserChrome(nsIWebBrowserChrome3 *aWebBrowserChrome) override; \
  NS_IMETHOD_(void) SendRequestFocus(bool canFocus, mozilla::dom::CallerType aCallerType) override; \
  NS_IMETHOD RemoteSizeShellTo(int32_t width, int32_t height, int32_t shellItemWidth, int32_t shellItemHeight) override; \
  NS_IMETHOD RemoteDropLinks(const nsTArray<RefPtr<nsIDroppedLinkItem>>& links) override; \
  NS_IMETHOD GetTabId(uint64_t *aTabId) override; \
  NS_IMETHOD GetHasSiblings(bool *aHasSiblings) override; \
  NS_IMETHOD SetHasSiblings(bool aHasSiblings) override; \
  NS_IMETHOD BeginSendingWebProgressEventsToParent(void) override; \
  NS_IMETHOD NotifyNavigationFinished(void) override; \
  NS_IMETHOD GetChromeOuterWindowID(uint64_t *aChromeOuterWindowID) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIBROWSERCHILD \
  nsresult GetMessageManager(mozilla::dom::ContentFrameMessageManager **aMessageManager); \
  nsresult GetWebBrowserChrome(nsIWebBrowserChrome3 **aWebBrowserChrome); \
  nsresult SetWebBrowserChrome(nsIWebBrowserChrome3 *aWebBrowserChrome); \
  nsresult_(void) SendRequestFocus(bool canFocus, mozilla::dom::CallerType aCallerType); \
  nsresult RemoteSizeShellTo(int32_t width, int32_t height, int32_t shellItemWidth, int32_t shellItemHeight); \
  nsresult RemoteDropLinks(const nsTArray<RefPtr<nsIDroppedLinkItem>>& links); \
  nsresult GetTabId(uint64_t *aTabId); \
  nsresult GetHasSiblings(bool *aHasSiblings); \
  nsresult SetHasSiblings(bool aHasSiblings); \
  nsresult BeginSendingWebProgressEventsToParent(void); \
  nsresult NotifyNavigationFinished(void); \
  nsresult GetChromeOuterWindowID(uint64_t *aChromeOuterWindowID); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIBROWSERCHILD(_to) \
  NS_IMETHOD GetMessageManager(mozilla::dom::ContentFrameMessageManager **aMessageManager) override { return _to GetMessageManager(aMessageManager); } \
  NS_IMETHOD GetWebBrowserChrome(nsIWebBrowserChrome3 **aWebBrowserChrome) override { return _to GetWebBrowserChrome(aWebBrowserChrome); } \
  NS_IMETHOD SetWebBrowserChrome(nsIWebBrowserChrome3 *aWebBrowserChrome) override { return _to SetWebBrowserChrome(aWebBrowserChrome); } \
  NS_IMETHOD_(void) SendRequestFocus(bool canFocus, mozilla::dom::CallerType aCallerType) override { return _to SendRequestFocus(canFocus, aCallerType); } \
  NS_IMETHOD RemoteSizeShellTo(int32_t width, int32_t height, int32_t shellItemWidth, int32_t shellItemHeight) override { return _to RemoteSizeShellTo(width, height, shellItemWidth, shellItemHeight); } \
  NS_IMETHOD RemoteDropLinks(const nsTArray<RefPtr<nsIDroppedLinkItem>>& links) override { return _to RemoteDropLinks(links); } \
  NS_IMETHOD GetTabId(uint64_t *aTabId) override { return _to GetTabId(aTabId); } \
  NS_IMETHOD GetHasSiblings(bool *aHasSiblings) override { return _to GetHasSiblings(aHasSiblings); } \
  NS_IMETHOD SetHasSiblings(bool aHasSiblings) override { return _to SetHasSiblings(aHasSiblings); } \
  NS_IMETHOD BeginSendingWebProgressEventsToParent(void) override { return _to BeginSendingWebProgressEventsToParent(); } \
  NS_IMETHOD NotifyNavigationFinished(void) override { return _to NotifyNavigationFinished(); } \
  NS_IMETHOD GetChromeOuterWindowID(uint64_t *aChromeOuterWindowID) override { return _to GetChromeOuterWindowID(aChromeOuterWindowID); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIBROWSERCHILD(_to) \
  NS_IMETHOD GetMessageManager(mozilla::dom::ContentFrameMessageManager **aMessageManager) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMessageManager(aMessageManager); } \
  NS_IMETHOD GetWebBrowserChrome(nsIWebBrowserChrome3 **aWebBrowserChrome) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWebBrowserChrome(aWebBrowserChrome); } \
  NS_IMETHOD SetWebBrowserChrome(nsIWebBrowserChrome3 *aWebBrowserChrome) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetWebBrowserChrome(aWebBrowserChrome); } \
  NS_IMETHOD_(void) SendRequestFocus(bool canFocus, mozilla::dom::CallerType aCallerType) override; \
  NS_IMETHOD RemoteSizeShellTo(int32_t width, int32_t height, int32_t shellItemWidth, int32_t shellItemHeight) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoteSizeShellTo(width, height, shellItemWidth, shellItemHeight); } \
  NS_IMETHOD RemoteDropLinks(const nsTArray<RefPtr<nsIDroppedLinkItem>>& links) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoteDropLinks(links); } \
  NS_IMETHOD GetTabId(uint64_t *aTabId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTabId(aTabId); } \
  NS_IMETHOD GetHasSiblings(bool *aHasSiblings) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHasSiblings(aHasSiblings); } \
  NS_IMETHOD SetHasSiblings(bool aHasSiblings) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetHasSiblings(aHasSiblings); } \
  NS_IMETHOD BeginSendingWebProgressEventsToParent(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->BeginSendingWebProgressEventsToParent(); } \
  NS_IMETHOD NotifyNavigationFinished(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyNavigationFinished(); } \
  NS_IMETHOD GetChromeOuterWindowID(uint64_t *aChromeOuterWindowID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChromeOuterWindowID(aChromeOuterWindowID); } 


#endif /* __gen_nsIBrowserChild_h__ */
