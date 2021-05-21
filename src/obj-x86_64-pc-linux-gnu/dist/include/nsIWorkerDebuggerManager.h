/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/workers/nsIWorkerDebuggerManager.idl
 */

#ifndef __gen_nsIWorkerDebuggerManager_h__
#define __gen_nsIWorkerDebuggerManager_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsISimpleEnumerator; /* forward declaration */

class nsIWorkerDebugger; /* forward declaration */


/* starting interface:    nsIWorkerDebuggerManagerListener */
#define NS_IWORKERDEBUGGERMANAGERLISTENER_IID_STR "d2aa74ee-6b98-4d5d-8173-4e23422daf1e"

#define NS_IWORKERDEBUGGERMANAGERLISTENER_IID \
  {0xd2aa74ee, 0x6b98, 0x4d5d, \
    { 0x81, 0x73, 0x4e, 0x23, 0x42, 0x2d, 0xaf, 0x1e }}

class NS_NO_VTABLE nsIWorkerDebuggerManagerListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWORKERDEBUGGERMANAGERLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWorkerDebuggerManagerListener;

  /* void onRegister (in nsIWorkerDebugger debugger); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnRegister(nsIWorkerDebugger *debugger) = 0;

  /* void onUnregister (in nsIWorkerDebugger debugger); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnUnregister(nsIWorkerDebugger *debugger) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWorkerDebuggerManagerListener, NS_IWORKERDEBUGGERMANAGERLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWORKERDEBUGGERMANAGERLISTENER \
  NS_IMETHOD OnRegister(nsIWorkerDebugger *debugger) override; \
  NS_IMETHOD OnUnregister(nsIWorkerDebugger *debugger) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWORKERDEBUGGERMANAGERLISTENER \
  nsresult OnRegister(nsIWorkerDebugger *debugger); \
  nsresult OnUnregister(nsIWorkerDebugger *debugger); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWORKERDEBUGGERMANAGERLISTENER(_to) \
  NS_IMETHOD OnRegister(nsIWorkerDebugger *debugger) override { return _to OnRegister(debugger); } \
  NS_IMETHOD OnUnregister(nsIWorkerDebugger *debugger) override { return _to OnUnregister(debugger); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWORKERDEBUGGERMANAGERLISTENER(_to) \
  NS_IMETHOD OnRegister(nsIWorkerDebugger *debugger) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnRegister(debugger); } \
  NS_IMETHOD OnUnregister(nsIWorkerDebugger *debugger) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnUnregister(debugger); } 


/* starting interface:    nsIWorkerDebuggerManager */
#define NS_IWORKERDEBUGGERMANAGER_IID_STR "056d7918-dc86-452a-b4e6-86da3405f015"

#define NS_IWORKERDEBUGGERMANAGER_IID \
  {0x056d7918, 0xdc86, 0x452a, \
    { 0xb4, 0xe6, 0x86, 0xda, 0x34, 0x05, 0xf0, 0x15 }}

class NS_NO_VTABLE nsIWorkerDebuggerManager : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWORKERDEBUGGERMANAGER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWorkerDebuggerManager;

  /* nsISimpleEnumerator getWorkerDebuggerEnumerator (); */
  NS_IMETHOD GetWorkerDebuggerEnumerator(nsISimpleEnumerator **_retval) = 0;

  /* void addListener (in nsIWorkerDebuggerManagerListener listener); */
  NS_IMETHOD AddListener(nsIWorkerDebuggerManagerListener *listener) = 0;

  /* void removeListener (in nsIWorkerDebuggerManagerListener listener); */
  NS_IMETHOD RemoveListener(nsIWorkerDebuggerManagerListener *listener) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWorkerDebuggerManager, NS_IWORKERDEBUGGERMANAGER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWORKERDEBUGGERMANAGER \
  NS_IMETHOD GetWorkerDebuggerEnumerator(nsISimpleEnumerator **_retval) override; \
  NS_IMETHOD AddListener(nsIWorkerDebuggerManagerListener *listener) override; \
  NS_IMETHOD RemoveListener(nsIWorkerDebuggerManagerListener *listener) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWORKERDEBUGGERMANAGER \
  nsresult GetWorkerDebuggerEnumerator(nsISimpleEnumerator **_retval); \
  nsresult AddListener(nsIWorkerDebuggerManagerListener *listener); \
  nsresult RemoveListener(nsIWorkerDebuggerManagerListener *listener); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWORKERDEBUGGERMANAGER(_to) \
  NS_IMETHOD GetWorkerDebuggerEnumerator(nsISimpleEnumerator **_retval) override { return _to GetWorkerDebuggerEnumerator(_retval); } \
  NS_IMETHOD AddListener(nsIWorkerDebuggerManagerListener *listener) override { return _to AddListener(listener); } \
  NS_IMETHOD RemoveListener(nsIWorkerDebuggerManagerListener *listener) override { return _to RemoveListener(listener); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWORKERDEBUGGERMANAGER(_to) \
  NS_IMETHOD GetWorkerDebuggerEnumerator(nsISimpleEnumerator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWorkerDebuggerEnumerator(_retval); } \
  NS_IMETHOD AddListener(nsIWorkerDebuggerManagerListener *listener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddListener(listener); } \
  NS_IMETHOD RemoveListener(nsIWorkerDebuggerManagerListener *listener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveListener(listener); } 


#endif /* __gen_nsIWorkerDebuggerManager_h__ */
