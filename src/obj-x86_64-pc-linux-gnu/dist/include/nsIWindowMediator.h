/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpfe/appshell/nsIWindowMediator.idl
 */

#ifndef __gen_nsIWindowMediator_h__
#define __gen_nsIWindowMediator_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsISimpleEnumerator_h__
#include "nsISimpleEnumerator.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#define NS_WINDOWMEDIATOR_CID \
{ 0x79a2b7cc, 0xf05b, 0x4605, \
  { 0xbf, 0xa0, 0xfa, 0xc5, 0x4f, 0x27, 0xee, 0xc8 } }
#define NS_WINDOWMEDIATOR_CONTRACTID \
  "@mozilla.org/appshell/window-mediator;1"
class mozIDOMWindow; /* forward declaration */

class mozIDOMWindowProxy; /* forward declaration */

class nsIAppWindow; /* forward declaration */

class nsIWidget; /* forward declaration */

class nsIWindowMediatorListener; /* forward declaration */


/* starting interface:    nsIWindowMediator */
#define NS_IWINDOWMEDIATOR_IID_STR "df0da056-357d-427f-bafd-e6cbf19c9381"

#define NS_IWINDOWMEDIATOR_IID \
  {0xdf0da056, 0x357d, 0x427f, \
    { 0xba, 0xfd, 0xe6, 0xcb, 0xf1, 0x9c, 0x93, 0x81 }}

class NS_NO_VTABLE nsIWindowMediator : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWINDOWMEDIATOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWindowMediator;

  /* nsISimpleEnumerator getEnumerator (in wstring aWindowType); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEnumerator(const char16_t * aWindowType, nsISimpleEnumerator **_retval) = 0;

  /* nsISimpleEnumerator getAppWindowEnumerator (in wstring aWindowType); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAppWindowEnumerator(const char16_t * aWindowType, nsISimpleEnumerator **_retval) = 0;

  /* nsISimpleEnumerator getZOrderAppWindowEnumerator (in wstring aWindowType, in boolean aFrontToBack); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetZOrderAppWindowEnumerator(const char16_t * aWindowType, bool aFrontToBack, nsISimpleEnumerator **_retval) = 0;

  /* mozIDOMWindowProxy getMostRecentWindow (in wstring aWindowType); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMostRecentWindow(const char16_t * aWindowType, mozIDOMWindowProxy **_retval) = 0;

  /* mozIDOMWindowProxy getMostRecentBrowserWindow (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMostRecentBrowserWindow(mozIDOMWindowProxy **_retval) = 0;

  /* mozIDOMWindowProxy getMostRecentNonPBWindow (in wstring aWindowType); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMostRecentNonPBWindow(const char16_t * aWindowType, mozIDOMWindowProxy **_retval) = 0;

  /* mozIDOMWindowProxy getOuterWindowWithId (in unsigned long long aOuterWindowID); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOuterWindowWithId(uint64_t aOuterWindowID, mozIDOMWindowProxy **_retval) = 0;

  /* mozIDOMWindow getCurrentInnerWindowWithId (in unsigned long long aInnerWindowID); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCurrentInnerWindowWithId(uint64_t aInnerWindowID, mozIDOMWindow **_retval) = 0;

  /* [noscript] void registerWindow (in nsIAppWindow aWindow); */
  NS_IMETHOD RegisterWindow(nsIAppWindow *aWindow) = 0;

  /* [noscript] void unregisterWindow (in nsIAppWindow aWindow); */
  NS_IMETHOD UnregisterWindow(nsIAppWindow *aWindow) = 0;

  /* [noscript] void updateWindowTimeStamp (in nsIAppWindow aWindow); */
  NS_IMETHOD UpdateWindowTimeStamp(nsIAppWindow *aWindow) = 0;

  enum {
    zLevelTop = 1U,
    zLevelBottom = 2U,
    zLevelBelow = 3U
  };

  /* [noscript] boolean calculateZPosition (in nsIAppWindow inWindow, in unsigned long inPosition, in nsIWidget inBelow, out unsigned long outPosition, out nsIWidget outBelow); */
  NS_IMETHOD CalculateZPosition(nsIAppWindow *inWindow, uint32_t inPosition, nsIWidget *inBelow, uint32_t *outPosition, nsIWidget **outBelow, bool *_retval) = 0;

  /* [noscript] void setZPosition (in nsIAppWindow inWindow, in unsigned long inPosition, in nsIAppWindow inBelow); */
  NS_IMETHOD SetZPosition(nsIAppWindow *inWindow, uint32_t inPosition, nsIAppWindow *inBelow) = 0;

  /* [noscript] uint32_t getZLevel (in nsIAppWindow aWindow); */
  NS_IMETHOD GetZLevel(nsIAppWindow *aWindow, uint32_t *_retval) = 0;

  /* [noscript] void setZLevel (in nsIAppWindow aWindow, in uint32_t aZLevel); */
  NS_IMETHOD SetZLevel(nsIAppWindow *aWindow, uint32_t aZLevel) = 0;

  /* void addListener (in nsIWindowMediatorListener aListener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddListener(nsIWindowMediatorListener *aListener) = 0;

  /* void removeListener (in nsIWindowMediatorListener aListener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveListener(nsIWindowMediatorListener *aListener) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWindowMediator, NS_IWINDOWMEDIATOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWINDOWMEDIATOR \
  NS_IMETHOD GetEnumerator(const char16_t * aWindowType, nsISimpleEnumerator **_retval) override; \
  NS_IMETHOD GetAppWindowEnumerator(const char16_t * aWindowType, nsISimpleEnumerator **_retval) override; \
  NS_IMETHOD GetZOrderAppWindowEnumerator(const char16_t * aWindowType, bool aFrontToBack, nsISimpleEnumerator **_retval) override; \
  NS_IMETHOD GetMostRecentWindow(const char16_t * aWindowType, mozIDOMWindowProxy **_retval) override; \
  NS_IMETHOD GetMostRecentBrowserWindow(mozIDOMWindowProxy **_retval) override; \
  NS_IMETHOD GetMostRecentNonPBWindow(const char16_t * aWindowType, mozIDOMWindowProxy **_retval) override; \
  NS_IMETHOD GetOuterWindowWithId(uint64_t aOuterWindowID, mozIDOMWindowProxy **_retval) override; \
  NS_IMETHOD GetCurrentInnerWindowWithId(uint64_t aInnerWindowID, mozIDOMWindow **_retval) override; \
  NS_IMETHOD RegisterWindow(nsIAppWindow *aWindow) override; \
  NS_IMETHOD UnregisterWindow(nsIAppWindow *aWindow) override; \
  NS_IMETHOD UpdateWindowTimeStamp(nsIAppWindow *aWindow) override; \
  NS_IMETHOD CalculateZPosition(nsIAppWindow *inWindow, uint32_t inPosition, nsIWidget *inBelow, uint32_t *outPosition, nsIWidget **outBelow, bool *_retval) override; \
  NS_IMETHOD SetZPosition(nsIAppWindow *inWindow, uint32_t inPosition, nsIAppWindow *inBelow) override; \
  NS_IMETHOD GetZLevel(nsIAppWindow *aWindow, uint32_t *_retval) override; \
  NS_IMETHOD SetZLevel(nsIAppWindow *aWindow, uint32_t aZLevel) override; \
  NS_IMETHOD AddListener(nsIWindowMediatorListener *aListener) override; \
  NS_IMETHOD RemoveListener(nsIWindowMediatorListener *aListener) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWINDOWMEDIATOR \
  nsresult GetEnumerator(const char16_t * aWindowType, nsISimpleEnumerator **_retval); \
  nsresult GetAppWindowEnumerator(const char16_t * aWindowType, nsISimpleEnumerator **_retval); \
  nsresult GetZOrderAppWindowEnumerator(const char16_t * aWindowType, bool aFrontToBack, nsISimpleEnumerator **_retval); \
  nsresult GetMostRecentWindow(const char16_t * aWindowType, mozIDOMWindowProxy **_retval); \
  nsresult GetMostRecentBrowserWindow(mozIDOMWindowProxy **_retval); \
  nsresult GetMostRecentNonPBWindow(const char16_t * aWindowType, mozIDOMWindowProxy **_retval); \
  nsresult GetOuterWindowWithId(uint64_t aOuterWindowID, mozIDOMWindowProxy **_retval); \
  nsresult GetCurrentInnerWindowWithId(uint64_t aInnerWindowID, mozIDOMWindow **_retval); \
  nsresult RegisterWindow(nsIAppWindow *aWindow); \
  nsresult UnregisterWindow(nsIAppWindow *aWindow); \
  nsresult UpdateWindowTimeStamp(nsIAppWindow *aWindow); \
  nsresult CalculateZPosition(nsIAppWindow *inWindow, uint32_t inPosition, nsIWidget *inBelow, uint32_t *outPosition, nsIWidget **outBelow, bool *_retval); \
  nsresult SetZPosition(nsIAppWindow *inWindow, uint32_t inPosition, nsIAppWindow *inBelow); \
  nsresult GetZLevel(nsIAppWindow *aWindow, uint32_t *_retval); \
  nsresult SetZLevel(nsIAppWindow *aWindow, uint32_t aZLevel); \
  nsresult AddListener(nsIWindowMediatorListener *aListener); \
  nsresult RemoveListener(nsIWindowMediatorListener *aListener); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWINDOWMEDIATOR(_to) \
  NS_IMETHOD GetEnumerator(const char16_t * aWindowType, nsISimpleEnumerator **_retval) override { return _to GetEnumerator(aWindowType, _retval); } \
  NS_IMETHOD GetAppWindowEnumerator(const char16_t * aWindowType, nsISimpleEnumerator **_retval) override { return _to GetAppWindowEnumerator(aWindowType, _retval); } \
  NS_IMETHOD GetZOrderAppWindowEnumerator(const char16_t * aWindowType, bool aFrontToBack, nsISimpleEnumerator **_retval) override { return _to GetZOrderAppWindowEnumerator(aWindowType, aFrontToBack, _retval); } \
  NS_IMETHOD GetMostRecentWindow(const char16_t * aWindowType, mozIDOMWindowProxy **_retval) override { return _to GetMostRecentWindow(aWindowType, _retval); } \
  NS_IMETHOD GetMostRecentBrowserWindow(mozIDOMWindowProxy **_retval) override { return _to GetMostRecentBrowserWindow(_retval); } \
  NS_IMETHOD GetMostRecentNonPBWindow(const char16_t * aWindowType, mozIDOMWindowProxy **_retval) override { return _to GetMostRecentNonPBWindow(aWindowType, _retval); } \
  NS_IMETHOD GetOuterWindowWithId(uint64_t aOuterWindowID, mozIDOMWindowProxy **_retval) override { return _to GetOuterWindowWithId(aOuterWindowID, _retval); } \
  NS_IMETHOD GetCurrentInnerWindowWithId(uint64_t aInnerWindowID, mozIDOMWindow **_retval) override { return _to GetCurrentInnerWindowWithId(aInnerWindowID, _retval); } \
  NS_IMETHOD RegisterWindow(nsIAppWindow *aWindow) override { return _to RegisterWindow(aWindow); } \
  NS_IMETHOD UnregisterWindow(nsIAppWindow *aWindow) override { return _to UnregisterWindow(aWindow); } \
  NS_IMETHOD UpdateWindowTimeStamp(nsIAppWindow *aWindow) override { return _to UpdateWindowTimeStamp(aWindow); } \
  NS_IMETHOD CalculateZPosition(nsIAppWindow *inWindow, uint32_t inPosition, nsIWidget *inBelow, uint32_t *outPosition, nsIWidget **outBelow, bool *_retval) override { return _to CalculateZPosition(inWindow, inPosition, inBelow, outPosition, outBelow, _retval); } \
  NS_IMETHOD SetZPosition(nsIAppWindow *inWindow, uint32_t inPosition, nsIAppWindow *inBelow) override { return _to SetZPosition(inWindow, inPosition, inBelow); } \
  NS_IMETHOD GetZLevel(nsIAppWindow *aWindow, uint32_t *_retval) override { return _to GetZLevel(aWindow, _retval); } \
  NS_IMETHOD SetZLevel(nsIAppWindow *aWindow, uint32_t aZLevel) override { return _to SetZLevel(aWindow, aZLevel); } \
  NS_IMETHOD AddListener(nsIWindowMediatorListener *aListener) override { return _to AddListener(aListener); } \
  NS_IMETHOD RemoveListener(nsIWindowMediatorListener *aListener) override { return _to RemoveListener(aListener); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWINDOWMEDIATOR(_to) \
  NS_IMETHOD GetEnumerator(const char16_t * aWindowType, nsISimpleEnumerator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEnumerator(aWindowType, _retval); } \
  NS_IMETHOD GetAppWindowEnumerator(const char16_t * aWindowType, nsISimpleEnumerator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAppWindowEnumerator(aWindowType, _retval); } \
  NS_IMETHOD GetZOrderAppWindowEnumerator(const char16_t * aWindowType, bool aFrontToBack, nsISimpleEnumerator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetZOrderAppWindowEnumerator(aWindowType, aFrontToBack, _retval); } \
  NS_IMETHOD GetMostRecentWindow(const char16_t * aWindowType, mozIDOMWindowProxy **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMostRecentWindow(aWindowType, _retval); } \
  NS_IMETHOD GetMostRecentBrowserWindow(mozIDOMWindowProxy **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMostRecentBrowserWindow(_retval); } \
  NS_IMETHOD GetMostRecentNonPBWindow(const char16_t * aWindowType, mozIDOMWindowProxy **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMostRecentNonPBWindow(aWindowType, _retval); } \
  NS_IMETHOD GetOuterWindowWithId(uint64_t aOuterWindowID, mozIDOMWindowProxy **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOuterWindowWithId(aOuterWindowID, _retval); } \
  NS_IMETHOD GetCurrentInnerWindowWithId(uint64_t aInnerWindowID, mozIDOMWindow **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCurrentInnerWindowWithId(aInnerWindowID, _retval); } \
  NS_IMETHOD RegisterWindow(nsIAppWindow *aWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterWindow(aWindow); } \
  NS_IMETHOD UnregisterWindow(nsIAppWindow *aWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnregisterWindow(aWindow); } \
  NS_IMETHOD UpdateWindowTimeStamp(nsIAppWindow *aWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UpdateWindowTimeStamp(aWindow); } \
  NS_IMETHOD CalculateZPosition(nsIAppWindow *inWindow, uint32_t inPosition, nsIWidget *inBelow, uint32_t *outPosition, nsIWidget **outBelow, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CalculateZPosition(inWindow, inPosition, inBelow, outPosition, outBelow, _retval); } \
  NS_IMETHOD SetZPosition(nsIAppWindow *inWindow, uint32_t inPosition, nsIAppWindow *inBelow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetZPosition(inWindow, inPosition, inBelow); } \
  NS_IMETHOD GetZLevel(nsIAppWindow *aWindow, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetZLevel(aWindow, _retval); } \
  NS_IMETHOD SetZLevel(nsIAppWindow *aWindow, uint32_t aZLevel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetZLevel(aWindow, aZLevel); } \
  NS_IMETHOD AddListener(nsIWindowMediatorListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddListener(aListener); } \
  NS_IMETHOD RemoveListener(nsIWindowMediatorListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveListener(aListener); } 


#endif /* __gen_nsIWindowMediator_h__ */
