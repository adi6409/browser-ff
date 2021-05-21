/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIBrowserDOMWindow.idl
 */

#ifndef __gen_nsIBrowserDOMWindow_h__
#define __gen_nsIBrowserDOMWindow_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIDOMWindowProxy; /* forward declaration */

class nsIDOMWindow; /* forward declaration */

class nsIURI; /* forward declaration */

class nsIPrincipal; /* forward declaration */

class nsIContentSecurityPolicy; /* forward declaration */

class nsIReferrerInfo; /* forward declaration */

class nsIOpenWindowInfo; /* forward declaration */

namespace mozilla {
namespace dom {
class BrowsingContext; /* webidl BrowsingContext */
} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIOpenURIInFrameParams */
#define NS_IOPENURIINFRAMEPARAMS_IID_STR "e774db14-79ac-4156-a7a3-aa3fd0a22c10"

#define NS_IOPENURIINFRAMEPARAMS_IID \
  {0xe774db14, 0x79ac, 0x4156, \
    { 0xa7, 0xa3, 0xaa, 0x3f, 0xd0, 0xa2, 0x2c, 0x10 }}

class NS_NO_VTABLE nsIOpenURIInFrameParams : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IOPENURIINFRAMEPARAMS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIOpenURIInFrameParams;

  /* readonly attribute nsIOpenWindowInfo openWindowInfo; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOpenWindowInfo(nsIOpenWindowInfo **aOpenWindowInfo) = 0;

  /* attribute nsIReferrerInfo referrerInfo; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetReferrerInfo(nsIReferrerInfo **aReferrerInfo) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetReferrerInfo(nsIReferrerInfo *aReferrerInfo) = 0;

  /* readonly attribute boolean isPrivate; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsPrivate(bool *aIsPrivate) = 0;

  /* attribute nsIPrincipal triggeringPrincipal; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTriggeringPrincipal(nsIPrincipal **aTriggeringPrincipal) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetTriggeringPrincipal(nsIPrincipal *aTriggeringPrincipal) = 0;

  /* attribute nsIContentSecurityPolicy csp; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCsp(nsIContentSecurityPolicy **aCsp) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCsp(nsIContentSecurityPolicy *aCsp) = 0;

  /* readonly attribute Element openerBrowser; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOpenerBrowser(mozilla::dom::Element **aOpenerBrowser) = 0;

  /* [implicit_jscontext] readonly attribute jsval openerOriginAttributes; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOpenerOriginAttributes(JSContext* cx, JS::MutableHandleValue aOpenerOriginAttributes) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIOpenURIInFrameParams, NS_IOPENURIINFRAMEPARAMS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIOPENURIINFRAMEPARAMS \
  NS_IMETHOD GetOpenWindowInfo(nsIOpenWindowInfo **aOpenWindowInfo) override; \
  NS_IMETHOD GetReferrerInfo(nsIReferrerInfo **aReferrerInfo) override; \
  NS_IMETHOD SetReferrerInfo(nsIReferrerInfo *aReferrerInfo) override; \
  NS_IMETHOD GetIsPrivate(bool *aIsPrivate) override; \
  NS_IMETHOD GetTriggeringPrincipal(nsIPrincipal **aTriggeringPrincipal) override; \
  NS_IMETHOD SetTriggeringPrincipal(nsIPrincipal *aTriggeringPrincipal) override; \
  NS_IMETHOD GetCsp(nsIContentSecurityPolicy **aCsp) override; \
  NS_IMETHOD SetCsp(nsIContentSecurityPolicy *aCsp) override; \
  NS_IMETHOD GetOpenerBrowser(mozilla::dom::Element **aOpenerBrowser) override; \
  NS_IMETHOD GetOpenerOriginAttributes(JSContext* cx, JS::MutableHandleValue aOpenerOriginAttributes) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIOPENURIINFRAMEPARAMS \
  nsresult GetOpenWindowInfo(nsIOpenWindowInfo **aOpenWindowInfo); \
  nsresult GetReferrerInfo(nsIReferrerInfo **aReferrerInfo); \
  nsresult SetReferrerInfo(nsIReferrerInfo *aReferrerInfo); \
  nsresult GetIsPrivate(bool *aIsPrivate); \
  nsresult GetTriggeringPrincipal(nsIPrincipal **aTriggeringPrincipal); \
  nsresult SetTriggeringPrincipal(nsIPrincipal *aTriggeringPrincipal); \
  nsresult GetCsp(nsIContentSecurityPolicy **aCsp); \
  nsresult SetCsp(nsIContentSecurityPolicy *aCsp); \
  nsresult GetOpenerBrowser(mozilla::dom::Element **aOpenerBrowser); \
  nsresult GetOpenerOriginAttributes(JSContext* cx, JS::MutableHandleValue aOpenerOriginAttributes); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIOPENURIINFRAMEPARAMS(_to) \
  NS_IMETHOD GetOpenWindowInfo(nsIOpenWindowInfo **aOpenWindowInfo) override { return _to GetOpenWindowInfo(aOpenWindowInfo); } \
  NS_IMETHOD GetReferrerInfo(nsIReferrerInfo **aReferrerInfo) override { return _to GetReferrerInfo(aReferrerInfo); } \
  NS_IMETHOD SetReferrerInfo(nsIReferrerInfo *aReferrerInfo) override { return _to SetReferrerInfo(aReferrerInfo); } \
  NS_IMETHOD GetIsPrivate(bool *aIsPrivate) override { return _to GetIsPrivate(aIsPrivate); } \
  NS_IMETHOD GetTriggeringPrincipal(nsIPrincipal **aTriggeringPrincipal) override { return _to GetTriggeringPrincipal(aTriggeringPrincipal); } \
  NS_IMETHOD SetTriggeringPrincipal(nsIPrincipal *aTriggeringPrincipal) override { return _to SetTriggeringPrincipal(aTriggeringPrincipal); } \
  NS_IMETHOD GetCsp(nsIContentSecurityPolicy **aCsp) override { return _to GetCsp(aCsp); } \
  NS_IMETHOD SetCsp(nsIContentSecurityPolicy *aCsp) override { return _to SetCsp(aCsp); } \
  NS_IMETHOD GetOpenerBrowser(mozilla::dom::Element **aOpenerBrowser) override { return _to GetOpenerBrowser(aOpenerBrowser); } \
  NS_IMETHOD GetOpenerOriginAttributes(JSContext* cx, JS::MutableHandleValue aOpenerOriginAttributes) override { return _to GetOpenerOriginAttributes(cx, aOpenerOriginAttributes); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIOPENURIINFRAMEPARAMS(_to) \
  NS_IMETHOD GetOpenWindowInfo(nsIOpenWindowInfo **aOpenWindowInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOpenWindowInfo(aOpenWindowInfo); } \
  NS_IMETHOD GetReferrerInfo(nsIReferrerInfo **aReferrerInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetReferrerInfo(aReferrerInfo); } \
  NS_IMETHOD SetReferrerInfo(nsIReferrerInfo *aReferrerInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetReferrerInfo(aReferrerInfo); } \
  NS_IMETHOD GetIsPrivate(bool *aIsPrivate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsPrivate(aIsPrivate); } \
  NS_IMETHOD GetTriggeringPrincipal(nsIPrincipal **aTriggeringPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTriggeringPrincipal(aTriggeringPrincipal); } \
  NS_IMETHOD SetTriggeringPrincipal(nsIPrincipal *aTriggeringPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTriggeringPrincipal(aTriggeringPrincipal); } \
  NS_IMETHOD GetCsp(nsIContentSecurityPolicy **aCsp) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCsp(aCsp); } \
  NS_IMETHOD SetCsp(nsIContentSecurityPolicy *aCsp) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCsp(aCsp); } \
  NS_IMETHOD GetOpenerBrowser(mozilla::dom::Element **aOpenerBrowser) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOpenerBrowser(aOpenerBrowser); } \
  NS_IMETHOD GetOpenerOriginAttributes(JSContext* cx, JS::MutableHandleValue aOpenerOriginAttributes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOpenerOriginAttributes(cx, aOpenerOriginAttributes); } 


/* starting interface:    nsIBrowserDOMWindow */
#define NS_IBROWSERDOMWINDOW_IID_STR "2a9bb880-5d73-40f3-8152-c60c8d137a14"

#define NS_IBROWSERDOMWINDOW_IID \
  {0x2a9bb880, 0x5d73, 0x40f3, \
    { 0x81, 0x52, 0xc6, 0x0c, 0x8d, 0x13, 0x7a, 0x14 }}

class NS_NO_VTABLE nsIBrowserDOMWindow : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IBROWSERDOMWINDOW_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIBrowserDOMWindow;

  enum {
    OPEN_DEFAULTWINDOW = 0,
    OPEN_CURRENTWINDOW = 1,
    OPEN_NEWWINDOW = 2,
    OPEN_NEWTAB = 3,
    OPEN_PRINT_BROWSER = 4,
    OPEN_NEW = 0,
    OPEN_EXTERNAL = 1,
    OPEN_NO_OPENER = 4,
    OPEN_NO_REFERRER = 8
  };

  /* BrowsingContext createContentWindow (in nsIURI aURI, in nsIOpenWindowInfo aOpenWindowInfo, in short aWhere, in long aFlags, in nsIPrincipal aTriggeringPrincipal, [optional] in nsIContentSecurityPolicy aCsp); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CreateContentWindow(nsIURI *aURI, nsIOpenWindowInfo *aOpenWindowInfo, int16_t aWhere, int32_t aFlags, nsIPrincipal *aTriggeringPrincipal, nsIContentSecurityPolicy *aCsp, mozilla::dom::BrowsingContext **_retval) = 0;

  /* Element createContentWindowInFrame (in nsIURI aURI, in nsIOpenURIInFrameParams params, in short aWhere, in long aFlags, in AString aName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CreateContentWindowInFrame(nsIURI *aURI, nsIOpenURIInFrameParams *params, int16_t aWhere, int32_t aFlags, const nsAString& aName, mozilla::dom::Element **_retval) = 0;

  /* BrowsingContext openURI (in nsIURI aURI, in nsIOpenWindowInfo aOpenWindowInfo, in short aWhere, in long aFlags, in nsIPrincipal aTriggeringPrincipal, [optional] in nsIContentSecurityPolicy aCsp); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OpenURI(nsIURI *aURI, nsIOpenWindowInfo *aOpenWindowInfo, int16_t aWhere, int32_t aFlags, nsIPrincipal *aTriggeringPrincipal, nsIContentSecurityPolicy *aCsp, mozilla::dom::BrowsingContext **_retval) = 0;

  /* Element openURIInFrame (in nsIURI aURI, in nsIOpenURIInFrameParams params, in short aWhere, in long aFlags, in AString aName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OpenURIInFrame(nsIURI *aURI, nsIOpenURIInFrameParams *params, int16_t aWhere, int32_t aFlags, const nsAString& aName, mozilla::dom::Element **_retval) = 0;

  /* boolean canClose (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CanClose(bool *_retval) = 0;

  /* readonly attribute unsigned long tabCount; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTabCount(uint32_t *aTabCount) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIBrowserDOMWindow, NS_IBROWSERDOMWINDOW_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIBROWSERDOMWINDOW \
  NS_IMETHOD CreateContentWindow(nsIURI *aURI, nsIOpenWindowInfo *aOpenWindowInfo, int16_t aWhere, int32_t aFlags, nsIPrincipal *aTriggeringPrincipal, nsIContentSecurityPolicy *aCsp, mozilla::dom::BrowsingContext **_retval) override; \
  NS_IMETHOD CreateContentWindowInFrame(nsIURI *aURI, nsIOpenURIInFrameParams *params, int16_t aWhere, int32_t aFlags, const nsAString& aName, mozilla::dom::Element **_retval) override; \
  NS_IMETHOD OpenURI(nsIURI *aURI, nsIOpenWindowInfo *aOpenWindowInfo, int16_t aWhere, int32_t aFlags, nsIPrincipal *aTriggeringPrincipal, nsIContentSecurityPolicy *aCsp, mozilla::dom::BrowsingContext **_retval) override; \
  NS_IMETHOD OpenURIInFrame(nsIURI *aURI, nsIOpenURIInFrameParams *params, int16_t aWhere, int32_t aFlags, const nsAString& aName, mozilla::dom::Element **_retval) override; \
  NS_IMETHOD CanClose(bool *_retval) override; \
  NS_IMETHOD GetTabCount(uint32_t *aTabCount) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIBROWSERDOMWINDOW \
  nsresult CreateContentWindow(nsIURI *aURI, nsIOpenWindowInfo *aOpenWindowInfo, int16_t aWhere, int32_t aFlags, nsIPrincipal *aTriggeringPrincipal, nsIContentSecurityPolicy *aCsp, mozilla::dom::BrowsingContext **_retval); \
  nsresult CreateContentWindowInFrame(nsIURI *aURI, nsIOpenURIInFrameParams *params, int16_t aWhere, int32_t aFlags, const nsAString& aName, mozilla::dom::Element **_retval); \
  nsresult OpenURI(nsIURI *aURI, nsIOpenWindowInfo *aOpenWindowInfo, int16_t aWhere, int32_t aFlags, nsIPrincipal *aTriggeringPrincipal, nsIContentSecurityPolicy *aCsp, mozilla::dom::BrowsingContext **_retval); \
  nsresult OpenURIInFrame(nsIURI *aURI, nsIOpenURIInFrameParams *params, int16_t aWhere, int32_t aFlags, const nsAString& aName, mozilla::dom::Element **_retval); \
  nsresult CanClose(bool *_retval); \
  nsresult GetTabCount(uint32_t *aTabCount); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIBROWSERDOMWINDOW(_to) \
  NS_IMETHOD CreateContentWindow(nsIURI *aURI, nsIOpenWindowInfo *aOpenWindowInfo, int16_t aWhere, int32_t aFlags, nsIPrincipal *aTriggeringPrincipal, nsIContentSecurityPolicy *aCsp, mozilla::dom::BrowsingContext **_retval) override { return _to CreateContentWindow(aURI, aOpenWindowInfo, aWhere, aFlags, aTriggeringPrincipal, aCsp, _retval); } \
  NS_IMETHOD CreateContentWindowInFrame(nsIURI *aURI, nsIOpenURIInFrameParams *params, int16_t aWhere, int32_t aFlags, const nsAString& aName, mozilla::dom::Element **_retval) override { return _to CreateContentWindowInFrame(aURI, params, aWhere, aFlags, aName, _retval); } \
  NS_IMETHOD OpenURI(nsIURI *aURI, nsIOpenWindowInfo *aOpenWindowInfo, int16_t aWhere, int32_t aFlags, nsIPrincipal *aTriggeringPrincipal, nsIContentSecurityPolicy *aCsp, mozilla::dom::BrowsingContext **_retval) override { return _to OpenURI(aURI, aOpenWindowInfo, aWhere, aFlags, aTriggeringPrincipal, aCsp, _retval); } \
  NS_IMETHOD OpenURIInFrame(nsIURI *aURI, nsIOpenURIInFrameParams *params, int16_t aWhere, int32_t aFlags, const nsAString& aName, mozilla::dom::Element **_retval) override { return _to OpenURIInFrame(aURI, params, aWhere, aFlags, aName, _retval); } \
  NS_IMETHOD CanClose(bool *_retval) override { return _to CanClose(_retval); } \
  NS_IMETHOD GetTabCount(uint32_t *aTabCount) override { return _to GetTabCount(aTabCount); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIBROWSERDOMWINDOW(_to) \
  NS_IMETHOD CreateContentWindow(nsIURI *aURI, nsIOpenWindowInfo *aOpenWindowInfo, int16_t aWhere, int32_t aFlags, nsIPrincipal *aTriggeringPrincipal, nsIContentSecurityPolicy *aCsp, mozilla::dom::BrowsingContext **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateContentWindow(aURI, aOpenWindowInfo, aWhere, aFlags, aTriggeringPrincipal, aCsp, _retval); } \
  NS_IMETHOD CreateContentWindowInFrame(nsIURI *aURI, nsIOpenURIInFrameParams *params, int16_t aWhere, int32_t aFlags, const nsAString& aName, mozilla::dom::Element **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateContentWindowInFrame(aURI, params, aWhere, aFlags, aName, _retval); } \
  NS_IMETHOD OpenURI(nsIURI *aURI, nsIOpenWindowInfo *aOpenWindowInfo, int16_t aWhere, int32_t aFlags, nsIPrincipal *aTriggeringPrincipal, nsIContentSecurityPolicy *aCsp, mozilla::dom::BrowsingContext **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenURI(aURI, aOpenWindowInfo, aWhere, aFlags, aTriggeringPrincipal, aCsp, _retval); } \
  NS_IMETHOD OpenURIInFrame(nsIURI *aURI, nsIOpenURIInFrameParams *params, int16_t aWhere, int32_t aFlags, const nsAString& aName, mozilla::dom::Element **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenURIInFrame(aURI, params, aWhere, aFlags, aName, _retval); } \
  NS_IMETHOD CanClose(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CanClose(_retval); } \
  NS_IMETHOD GetTabCount(uint32_t *aTabCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTabCount(aTabCount); } 


#endif /* __gen_nsIBrowserDOMWindow_h__ */
