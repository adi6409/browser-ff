/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/workers/nsIWorkerDebugger.idl
 */

#ifndef __gen_nsIWorkerDebugger_h__
#define __gen_nsIWorkerDebugger_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIDOMWindow; /* forward declaration */

class nsIPrincipal; /* forward declaration */


/* starting interface:    nsIWorkerDebuggerListener */
#define NS_IWORKERDEBUGGERLISTENER_IID_STR "9cf3b48e-361d-486a-8917-55cf8d00bb41"

#define NS_IWORKERDEBUGGERLISTENER_IID \
  {0x9cf3b48e, 0x361d, 0x486a, \
    { 0x89, 0x17, 0x55, 0xcf, 0x8d, 0x00, 0xbb, 0x41 }}

class NS_NO_VTABLE nsIWorkerDebuggerListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWORKERDEBUGGERLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWorkerDebuggerListener;

  /* void onClose (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnClose(void) = 0;

  /* void onError (in AString filename, in unsigned long lineno, in AString message); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnError(const nsAString& filename, uint32_t lineno, const nsAString& message) = 0;

  /* void onMessage (in AString message); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnMessage(const nsAString& message) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWorkerDebuggerListener, NS_IWORKERDEBUGGERLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWORKERDEBUGGERLISTENER \
  NS_IMETHOD OnClose(void) override; \
  NS_IMETHOD OnError(const nsAString& filename, uint32_t lineno, const nsAString& message) override; \
  NS_IMETHOD OnMessage(const nsAString& message) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWORKERDEBUGGERLISTENER \
  nsresult OnClose(void); \
  nsresult OnError(const nsAString& filename, uint32_t lineno, const nsAString& message); \
  nsresult OnMessage(const nsAString& message); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWORKERDEBUGGERLISTENER(_to) \
  NS_IMETHOD OnClose(void) override { return _to OnClose(); } \
  NS_IMETHOD OnError(const nsAString& filename, uint32_t lineno, const nsAString& message) override { return _to OnError(filename, lineno, message); } \
  NS_IMETHOD OnMessage(const nsAString& message) override { return _to OnMessage(message); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWORKERDEBUGGERLISTENER(_to) \
  NS_IMETHOD OnClose(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnClose(); } \
  NS_IMETHOD OnError(const nsAString& filename, uint32_t lineno, const nsAString& message) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnError(filename, lineno, message); } \
  NS_IMETHOD OnMessage(const nsAString& message) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnMessage(message); } 


/* starting interface:    nsIWorkerDebugger */
#define NS_IWORKERDEBUGGER_IID_STR "22f93aa3-8a05-46be-87e0-fa93bf8a8eff"

#define NS_IWORKERDEBUGGER_IID \
  {0x22f93aa3, 0x8a05, 0x46be, \
    { 0x87, 0xe0, 0xfa, 0x93, 0xbf, 0x8a, 0x8e, 0xff }}

class NS_NO_VTABLE nsIWorkerDebugger : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWORKERDEBUGGER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWorkerDebugger;

  enum {
    TYPE_DEDICATED = 0U,
    TYPE_SHARED = 1U,
    TYPE_SERVICE = 2U
  };

  /* readonly attribute bool isClosed; */
  NS_IMETHOD GetIsClosed(bool *aIsClosed) = 0;

  /* readonly attribute bool isChrome; */
  NS_IMETHOD GetIsChrome(bool *aIsChrome) = 0;

  /* readonly attribute bool isInitialized; */
  NS_IMETHOD GetIsInitialized(bool *aIsInitialized) = 0;

  /* readonly attribute nsIWorkerDebugger parent; */
  NS_IMETHOD GetParent(nsIWorkerDebugger **aParent) = 0;

  /* readonly attribute unsigned long type; */
  NS_IMETHOD GetType(uint32_t *aType) = 0;

  /* readonly attribute AString url; */
  NS_IMETHOD GetUrl(nsAString& aUrl) = 0;

  /* readonly attribute mozIDOMWindow window; */
  NS_IMETHOD GetWindow(mozIDOMWindow **aWindow) = 0;

  /* readonly attribute Array<uint64_t> windowIDs; */
  NS_IMETHOD GetWindowIDs(nsTArray<uint64_t >& aWindowIDs) = 0;

  /* readonly attribute nsIPrincipal principal; */
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) = 0;

  /* readonly attribute unsigned long serviceWorkerID; */
  NS_IMETHOD GetServiceWorkerID(uint32_t *aServiceWorkerID) = 0;

  /* readonly attribute AString id; */
  NS_IMETHOD GetId(nsAString& aId) = 0;

  /* void initialize (in AString url); */
  NS_IMETHOD Initialize(const nsAString& url) = 0;

  /* [binaryname(PostMessageMoz)] void postMessage (in AString message); */
  NS_IMETHOD PostMessageMoz(const nsAString& message) = 0;

  /* void addListener (in nsIWorkerDebuggerListener listener); */
  NS_IMETHOD AddListener(nsIWorkerDebuggerListener *listener) = 0;

  /* void removeListener (in nsIWorkerDebuggerListener listener); */
  NS_IMETHOD RemoveListener(nsIWorkerDebuggerListener *listener) = 0;

  /* void setDebuggerReady (in boolean ready); */
  NS_IMETHOD SetDebuggerReady(bool ready) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWorkerDebugger, NS_IWORKERDEBUGGER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWORKERDEBUGGER \
  NS_IMETHOD GetIsClosed(bool *aIsClosed) override; \
  NS_IMETHOD GetIsChrome(bool *aIsChrome) override; \
  NS_IMETHOD GetIsInitialized(bool *aIsInitialized) override; \
  NS_IMETHOD GetParent(nsIWorkerDebugger **aParent) override; \
  NS_IMETHOD GetType(uint32_t *aType) override; \
  NS_IMETHOD GetUrl(nsAString& aUrl) override; \
  NS_IMETHOD GetWindow(mozIDOMWindow **aWindow) override; \
  NS_IMETHOD GetWindowIDs(nsTArray<uint64_t >& aWindowIDs) override; \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override; \
  NS_IMETHOD GetServiceWorkerID(uint32_t *aServiceWorkerID) override; \
  NS_IMETHOD GetId(nsAString& aId) override; \
  NS_IMETHOD Initialize(const nsAString& url) override; \
  NS_IMETHOD PostMessageMoz(const nsAString& message) override; \
  NS_IMETHOD AddListener(nsIWorkerDebuggerListener *listener) override; \
  NS_IMETHOD RemoveListener(nsIWorkerDebuggerListener *listener) override; \
  NS_IMETHOD SetDebuggerReady(bool ready) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWORKERDEBUGGER \
  nsresult GetIsClosed(bool *aIsClosed); \
  nsresult GetIsChrome(bool *aIsChrome); \
  nsresult GetIsInitialized(bool *aIsInitialized); \
  nsresult GetParent(nsIWorkerDebugger **aParent); \
  nsresult GetType(uint32_t *aType); \
  nsresult GetUrl(nsAString& aUrl); \
  nsresult GetWindow(mozIDOMWindow **aWindow); \
  nsresult GetWindowIDs(nsTArray<uint64_t >& aWindowIDs); \
  nsresult GetPrincipal(nsIPrincipal **aPrincipal); \
  nsresult GetServiceWorkerID(uint32_t *aServiceWorkerID); \
  nsresult GetId(nsAString& aId); \
  nsresult Initialize(const nsAString& url); \
  nsresult PostMessageMoz(const nsAString& message); \
  nsresult AddListener(nsIWorkerDebuggerListener *listener); \
  nsresult RemoveListener(nsIWorkerDebuggerListener *listener); \
  nsresult SetDebuggerReady(bool ready); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWORKERDEBUGGER(_to) \
  NS_IMETHOD GetIsClosed(bool *aIsClosed) override { return _to GetIsClosed(aIsClosed); } \
  NS_IMETHOD GetIsChrome(bool *aIsChrome) override { return _to GetIsChrome(aIsChrome); } \
  NS_IMETHOD GetIsInitialized(bool *aIsInitialized) override { return _to GetIsInitialized(aIsInitialized); } \
  NS_IMETHOD GetParent(nsIWorkerDebugger **aParent) override { return _to GetParent(aParent); } \
  NS_IMETHOD GetType(uint32_t *aType) override { return _to GetType(aType); } \
  NS_IMETHOD GetUrl(nsAString& aUrl) override { return _to GetUrl(aUrl); } \
  NS_IMETHOD GetWindow(mozIDOMWindow **aWindow) override { return _to GetWindow(aWindow); } \
  NS_IMETHOD GetWindowIDs(nsTArray<uint64_t >& aWindowIDs) override { return _to GetWindowIDs(aWindowIDs); } \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override { return _to GetPrincipal(aPrincipal); } \
  NS_IMETHOD GetServiceWorkerID(uint32_t *aServiceWorkerID) override { return _to GetServiceWorkerID(aServiceWorkerID); } \
  NS_IMETHOD GetId(nsAString& aId) override { return _to GetId(aId); } \
  NS_IMETHOD Initialize(const nsAString& url) override { return _to Initialize(url); } \
  NS_IMETHOD PostMessageMoz(const nsAString& message) override { return _to PostMessageMoz(message); } \
  NS_IMETHOD AddListener(nsIWorkerDebuggerListener *listener) override { return _to AddListener(listener); } \
  NS_IMETHOD RemoveListener(nsIWorkerDebuggerListener *listener) override { return _to RemoveListener(listener); } \
  NS_IMETHOD SetDebuggerReady(bool ready) override { return _to SetDebuggerReady(ready); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWORKERDEBUGGER(_to) \
  NS_IMETHOD GetIsClosed(bool *aIsClosed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsClosed(aIsClosed); } \
  NS_IMETHOD GetIsChrome(bool *aIsChrome) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsChrome(aIsChrome); } \
  NS_IMETHOD GetIsInitialized(bool *aIsInitialized) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsInitialized(aIsInitialized); } \
  NS_IMETHOD GetParent(nsIWorkerDebugger **aParent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetParent(aParent); } \
  NS_IMETHOD GetType(uint32_t *aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetType(aType); } \
  NS_IMETHOD GetUrl(nsAString& aUrl) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUrl(aUrl); } \
  NS_IMETHOD GetWindow(mozIDOMWindow **aWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWindow(aWindow); } \
  NS_IMETHOD GetWindowIDs(nsTArray<uint64_t >& aWindowIDs) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWindowIDs(aWindowIDs); } \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrincipal(aPrincipal); } \
  NS_IMETHOD GetServiceWorkerID(uint32_t *aServiceWorkerID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetServiceWorkerID(aServiceWorkerID); } \
  NS_IMETHOD GetId(nsAString& aId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetId(aId); } \
  NS_IMETHOD Initialize(const nsAString& url) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Initialize(url); } \
  NS_IMETHOD PostMessageMoz(const nsAString& message) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PostMessageMoz(message); } \
  NS_IMETHOD AddListener(nsIWorkerDebuggerListener *listener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddListener(listener); } \
  NS_IMETHOD RemoveListener(nsIWorkerDebuggerListener *listener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveListener(listener); } \
  NS_IMETHOD SetDebuggerReady(bool ready) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDebuggerReady(ready); } 


#endif /* __gen_nsIWorkerDebugger_h__ */
