/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpfe/appshell/nsIXULBrowserWindow.idl
 */

#ifndef __gen_nsIXULBrowserWindow_h__
#define __gen_nsIXULBrowserWindow_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIURI_h__
#include "nsIURI.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIBrowser; /* forward declaration */

class nsIRequest; /* forward declaration */

class nsIInputStream; /* forward declaration */

class nsIDocShell; /* forward declaration */

class nsIRemoteTab; /* forward declaration */

class nsIPrincipal; /* forward declaration */

class mozIDOMWindowProxy; /* forward declaration */

class nsIContentSecurityPolicy; /* forward declaration */

class nsIReferrerInfo; /* forward declaration */

namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla

class nsINode; /* webidl Node */


/* starting interface:    nsIXULBrowserWindow */
#define NS_IXULBROWSERWINDOW_IID_STR "a8675fa9-c8b4-4350-9803-c38f344a9e38"

#define NS_IXULBROWSERWINDOW_IID \
  {0xa8675fa9, 0xc8b4, 0x4350, \
    { 0x98, 0x03, 0xc3, 0x8f, 0x34, 0x4a, 0x9e, 0x38 }}

class NS_NO_VTABLE nsIXULBrowserWindow : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IXULBROWSERWINDOW_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIXULBrowserWindow;

  /* void setOverLink (in AString link); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetOverLink(const nsAString& link) = 0;

  /* AString onBeforeLinkTraversal (in AString originalTarget, in nsIURI linkURI, in Node linkNode, in boolean isAppTab); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnBeforeLinkTraversal(const nsAString& originalTarget, nsIURI *linkURI, nsINode *linkNode, bool isAppTab, nsAString& _retval) = 0;

  /* void showTooltip (in long x, in long y, in AString tooltip, in AString direction, in Element browser); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ShowTooltip(int32_t x, int32_t y, const nsAString& tooltip, const nsAString& direction, mozilla::dom::Element *browser) = 0;

  /* void hideTooltip (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HideTooltip(void) = 0;

  /* uint32_t getTabCount (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTabCount(uint32_t *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIXULBrowserWindow, NS_IXULBROWSERWINDOW_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIXULBROWSERWINDOW \
  NS_IMETHOD SetOverLink(const nsAString& link) override; \
  NS_IMETHOD OnBeforeLinkTraversal(const nsAString& originalTarget, nsIURI *linkURI, nsINode *linkNode, bool isAppTab, nsAString& _retval) override; \
  NS_IMETHOD ShowTooltip(int32_t x, int32_t y, const nsAString& tooltip, const nsAString& direction, mozilla::dom::Element *browser) override; \
  NS_IMETHOD HideTooltip(void) override; \
  NS_IMETHOD GetTabCount(uint32_t *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIXULBROWSERWINDOW \
  nsresult SetOverLink(const nsAString& link); \
  nsresult OnBeforeLinkTraversal(const nsAString& originalTarget, nsIURI *linkURI, nsINode *linkNode, bool isAppTab, nsAString& _retval); \
  nsresult ShowTooltip(int32_t x, int32_t y, const nsAString& tooltip, const nsAString& direction, mozilla::dom::Element *browser); \
  nsresult HideTooltip(void); \
  nsresult GetTabCount(uint32_t *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIXULBROWSERWINDOW(_to) \
  NS_IMETHOD SetOverLink(const nsAString& link) override { return _to SetOverLink(link); } \
  NS_IMETHOD OnBeforeLinkTraversal(const nsAString& originalTarget, nsIURI *linkURI, nsINode *linkNode, bool isAppTab, nsAString& _retval) override { return _to OnBeforeLinkTraversal(originalTarget, linkURI, linkNode, isAppTab, _retval); } \
  NS_IMETHOD ShowTooltip(int32_t x, int32_t y, const nsAString& tooltip, const nsAString& direction, mozilla::dom::Element *browser) override { return _to ShowTooltip(x, y, tooltip, direction, browser); } \
  NS_IMETHOD HideTooltip(void) override { return _to HideTooltip(); } \
  NS_IMETHOD GetTabCount(uint32_t *_retval) override { return _to GetTabCount(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIXULBROWSERWINDOW(_to) \
  NS_IMETHOD SetOverLink(const nsAString& link) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetOverLink(link); } \
  NS_IMETHOD OnBeforeLinkTraversal(const nsAString& originalTarget, nsIURI *linkURI, nsINode *linkNode, bool isAppTab, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnBeforeLinkTraversal(originalTarget, linkURI, linkNode, isAppTab, _retval); } \
  NS_IMETHOD ShowTooltip(int32_t x, int32_t y, const nsAString& tooltip, const nsAString& direction, mozilla::dom::Element *browser) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ShowTooltip(x, y, tooltip, direction, browser); } \
  NS_IMETHOD HideTooltip(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HideTooltip(); } \
  NS_IMETHOD GetTabCount(uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTabCount(_retval); } 


#endif /* __gen_nsIXULBrowserWindow_h__ */
