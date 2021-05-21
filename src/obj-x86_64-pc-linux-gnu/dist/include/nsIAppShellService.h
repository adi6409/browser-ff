/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpfe/appshell/nsIAppShellService.idl
 */

#ifndef __gen_nsIAppShellService_h__
#define __gen_nsIAppShellService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIAppWindow; /* forward declaration */

class nsIWindowlessBrowser; /* forward declaration */

class nsIURI; /* forward declaration */

class mozIDOMWindowProxy; /* forward declaration */

class nsIAppShell; /* forward declaration */

class nsIRemoteTab; /* forward declaration */

#include "js/TypeDecls.h"

/* starting interface:    nsIAppShellService */
#define NS_IAPPSHELLSERVICE_IID_STR "19266025-354c-4bb9-986b-3483b2b1cdef"

#define NS_IAPPSHELLSERVICE_IID \
  {0x19266025, 0x354c, 0x4bb9, \
    { 0x98, 0x6b, 0x34, 0x83, 0xb2, 0xb1, 0xcd, 0xef }}

class NS_NO_VTABLE nsIAppShellService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IAPPSHELLSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAppShellService;

  enum {
    SIZE_TO_CONTENT = -1
  };

  /* nsIAppWindow createTopLevelWindow (in nsIAppWindow aParent, in nsIURI aUrl, in uint32_t aChromeMask, in long aInitialWidth, in long aInitialHeight); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CreateTopLevelWindow(nsIAppWindow *aParent, nsIURI *aUrl, uint32_t aChromeMask, int32_t aInitialWidth, int32_t aInitialHeight, nsIAppWindow **_retval) = 0;

  /* nsIWindowlessBrowser createWindowlessBrowser ([optional] in bool aIsChrome, [optional] in uint32_t aChromeMask); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CreateWindowlessBrowser(bool aIsChrome, uint32_t aChromeMask, nsIWindowlessBrowser **_retval) = 0;

  /* [noscript] void createHiddenWindow (); */
  NS_IMETHOD CreateHiddenWindow(void) = 0;

  /* void destroyHiddenWindow (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DestroyHiddenWindow(void) = 0;

  /* [noscript] void setScreenId (in uint32_t aScreenId); */
  NS_IMETHOD SetScreenId(uint32_t aScreenId) = 0;

  /* readonly attribute nsIAppWindow hiddenWindow; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHiddenWindow(nsIAppWindow **aHiddenWindow) = 0;

  /* readonly attribute mozIDOMWindowProxy hiddenDOMWindow; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHiddenDOMWindow(mozIDOMWindowProxy **aHiddenDOMWindow) = 0;

  /* readonly attribute boolean applicationProvidedHiddenWindow; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetApplicationProvidedHiddenWindow(bool *aApplicationProvidedHiddenWindow) = 0;

  /* void registerTopLevelWindow (in nsIAppWindow aWindow); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RegisterTopLevelWindow(nsIAppWindow *aWindow) = 0;

  /* void unregisterTopLevelWindow (in nsIAppWindow aWindow); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UnregisterTopLevelWindow(nsIAppWindow *aWindow) = 0;

  /* readonly attribute boolean hasHiddenWindow; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHasHiddenWindow(bool *aHasHiddenWindow) = 0;

  /* bool startEventLoopLagTracking (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD StartEventLoopLagTracking(bool *_retval) = 0;

  /* void stopEventLoopLagTracking (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD StopEventLoopLagTracking(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAppShellService, NS_IAPPSHELLSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIAPPSHELLSERVICE \
  NS_IMETHOD CreateTopLevelWindow(nsIAppWindow *aParent, nsIURI *aUrl, uint32_t aChromeMask, int32_t aInitialWidth, int32_t aInitialHeight, nsIAppWindow **_retval) override; \
  NS_IMETHOD CreateWindowlessBrowser(bool aIsChrome, uint32_t aChromeMask, nsIWindowlessBrowser **_retval) override; \
  NS_IMETHOD CreateHiddenWindow(void) override; \
  NS_IMETHOD DestroyHiddenWindow(void) override; \
  NS_IMETHOD SetScreenId(uint32_t aScreenId) override; \
  NS_IMETHOD GetHiddenWindow(nsIAppWindow **aHiddenWindow) override; \
  NS_IMETHOD GetHiddenDOMWindow(mozIDOMWindowProxy **aHiddenDOMWindow) override; \
  NS_IMETHOD GetApplicationProvidedHiddenWindow(bool *aApplicationProvidedHiddenWindow) override; \
  NS_IMETHOD RegisterTopLevelWindow(nsIAppWindow *aWindow) override; \
  NS_IMETHOD UnregisterTopLevelWindow(nsIAppWindow *aWindow) override; \
  NS_IMETHOD GetHasHiddenWindow(bool *aHasHiddenWindow) override; \
  NS_IMETHOD StartEventLoopLagTracking(bool *_retval) override; \
  NS_IMETHOD StopEventLoopLagTracking(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIAPPSHELLSERVICE \
  nsresult CreateTopLevelWindow(nsIAppWindow *aParent, nsIURI *aUrl, uint32_t aChromeMask, int32_t aInitialWidth, int32_t aInitialHeight, nsIAppWindow **_retval); \
  nsresult CreateWindowlessBrowser(bool aIsChrome, uint32_t aChromeMask, nsIWindowlessBrowser **_retval); \
  nsresult CreateHiddenWindow(void); \
  nsresult DestroyHiddenWindow(void); \
  nsresult SetScreenId(uint32_t aScreenId); \
  nsresult GetHiddenWindow(nsIAppWindow **aHiddenWindow); \
  nsresult GetHiddenDOMWindow(mozIDOMWindowProxy **aHiddenDOMWindow); \
  nsresult GetApplicationProvidedHiddenWindow(bool *aApplicationProvidedHiddenWindow); \
  nsresult RegisterTopLevelWindow(nsIAppWindow *aWindow); \
  nsresult UnregisterTopLevelWindow(nsIAppWindow *aWindow); \
  nsresult GetHasHiddenWindow(bool *aHasHiddenWindow); \
  nsresult StartEventLoopLagTracking(bool *_retval); \
  nsresult StopEventLoopLagTracking(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIAPPSHELLSERVICE(_to) \
  NS_IMETHOD CreateTopLevelWindow(nsIAppWindow *aParent, nsIURI *aUrl, uint32_t aChromeMask, int32_t aInitialWidth, int32_t aInitialHeight, nsIAppWindow **_retval) override { return _to CreateTopLevelWindow(aParent, aUrl, aChromeMask, aInitialWidth, aInitialHeight, _retval); } \
  NS_IMETHOD CreateWindowlessBrowser(bool aIsChrome, uint32_t aChromeMask, nsIWindowlessBrowser **_retval) override { return _to CreateWindowlessBrowser(aIsChrome, aChromeMask, _retval); } \
  NS_IMETHOD CreateHiddenWindow(void) override { return _to CreateHiddenWindow(); } \
  NS_IMETHOD DestroyHiddenWindow(void) override { return _to DestroyHiddenWindow(); } \
  NS_IMETHOD SetScreenId(uint32_t aScreenId) override { return _to SetScreenId(aScreenId); } \
  NS_IMETHOD GetHiddenWindow(nsIAppWindow **aHiddenWindow) override { return _to GetHiddenWindow(aHiddenWindow); } \
  NS_IMETHOD GetHiddenDOMWindow(mozIDOMWindowProxy **aHiddenDOMWindow) override { return _to GetHiddenDOMWindow(aHiddenDOMWindow); } \
  NS_IMETHOD GetApplicationProvidedHiddenWindow(bool *aApplicationProvidedHiddenWindow) override { return _to GetApplicationProvidedHiddenWindow(aApplicationProvidedHiddenWindow); } \
  NS_IMETHOD RegisterTopLevelWindow(nsIAppWindow *aWindow) override { return _to RegisterTopLevelWindow(aWindow); } \
  NS_IMETHOD UnregisterTopLevelWindow(nsIAppWindow *aWindow) override { return _to UnregisterTopLevelWindow(aWindow); } \
  NS_IMETHOD GetHasHiddenWindow(bool *aHasHiddenWindow) override { return _to GetHasHiddenWindow(aHasHiddenWindow); } \
  NS_IMETHOD StartEventLoopLagTracking(bool *_retval) override { return _to StartEventLoopLagTracking(_retval); } \
  NS_IMETHOD StopEventLoopLagTracking(void) override { return _to StopEventLoopLagTracking(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIAPPSHELLSERVICE(_to) \
  NS_IMETHOD CreateTopLevelWindow(nsIAppWindow *aParent, nsIURI *aUrl, uint32_t aChromeMask, int32_t aInitialWidth, int32_t aInitialHeight, nsIAppWindow **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateTopLevelWindow(aParent, aUrl, aChromeMask, aInitialWidth, aInitialHeight, _retval); } \
  NS_IMETHOD CreateWindowlessBrowser(bool aIsChrome, uint32_t aChromeMask, nsIWindowlessBrowser **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateWindowlessBrowser(aIsChrome, aChromeMask, _retval); } \
  NS_IMETHOD CreateHiddenWindow(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateHiddenWindow(); } \
  NS_IMETHOD DestroyHiddenWindow(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DestroyHiddenWindow(); } \
  NS_IMETHOD SetScreenId(uint32_t aScreenId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetScreenId(aScreenId); } \
  NS_IMETHOD GetHiddenWindow(nsIAppWindow **aHiddenWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHiddenWindow(aHiddenWindow); } \
  NS_IMETHOD GetHiddenDOMWindow(mozIDOMWindowProxy **aHiddenDOMWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHiddenDOMWindow(aHiddenDOMWindow); } \
  NS_IMETHOD GetApplicationProvidedHiddenWindow(bool *aApplicationProvidedHiddenWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetApplicationProvidedHiddenWindow(aApplicationProvidedHiddenWindow); } \
  NS_IMETHOD RegisterTopLevelWindow(nsIAppWindow *aWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterTopLevelWindow(aWindow); } \
  NS_IMETHOD UnregisterTopLevelWindow(nsIAppWindow *aWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnregisterTopLevelWindow(aWindow); } \
  NS_IMETHOD GetHasHiddenWindow(bool *aHasHiddenWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHasHiddenWindow(aHasHiddenWindow); } \
  NS_IMETHOD StartEventLoopLagTracking(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StartEventLoopLagTracking(_retval); } \
  NS_IMETHOD StopEventLoopLagTracking(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StopEventLoopLagTracking(); } 


#endif /* __gen_nsIAppShellService_h__ */
