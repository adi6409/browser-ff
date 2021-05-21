/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/remote/components/nsIRemoteAgent.idl
 */

#ifndef __gen_nsIRemoteAgent_h__
#define __gen_nsIRemoteAgent_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIRemoteAgent */
#define NS_IREMOTEAGENT_IID_STR "8f685a9d-8181-46d6-a71d-869289099c6d"

#define NS_IREMOTEAGENT_IID \
  {0x8f685a9d, 0x8181, 0x46d6, \
    { 0xa7, 0x1d, 0x86, 0x92, 0x89, 0x09, 0x9c, 0x6d }}

class NS_NO_VTABLE nsIRemoteAgent : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IREMOTEAGENT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIRemoteAgent;

  /* readonly attribute AString debuggerAddress; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDebuggerAddress(nsAString& aDebuggerAddress) = 0;

  /* readonly attribute boolean listening; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetListening(bool *aListening) = 0;

  /* void listen (in AString aURL); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Listen(const nsAString& aURL) = 0;

  /* void close (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Close(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIRemoteAgent, NS_IREMOTEAGENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIREMOTEAGENT \
  NS_IMETHOD GetDebuggerAddress(nsAString& aDebuggerAddress) override; \
  NS_IMETHOD GetListening(bool *aListening) override; \
  NS_IMETHOD Listen(const nsAString& aURL) override; \
  NS_IMETHOD Close(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIREMOTEAGENT \
  nsresult GetDebuggerAddress(nsAString& aDebuggerAddress); \
  nsresult GetListening(bool *aListening); \
  nsresult Listen(const nsAString& aURL); \
  nsresult Close(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIREMOTEAGENT(_to) \
  NS_IMETHOD GetDebuggerAddress(nsAString& aDebuggerAddress) override { return _to GetDebuggerAddress(aDebuggerAddress); } \
  NS_IMETHOD GetListening(bool *aListening) override { return _to GetListening(aListening); } \
  NS_IMETHOD Listen(const nsAString& aURL) override { return _to Listen(aURL); } \
  NS_IMETHOD Close(void) override { return _to Close(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIREMOTEAGENT(_to) \
  NS_IMETHOD GetDebuggerAddress(nsAString& aDebuggerAddress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDebuggerAddress(aDebuggerAddress); } \
  NS_IMETHOD GetListening(bool *aListening) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetListening(aListening); } \
  NS_IMETHOD Listen(const nsAString& aURL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Listen(aURL); } \
  NS_IMETHOD Close(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Close(); } 

#define NS_REMOTEAGENT_CONTRACTID "@mozilla.org/remote/agent;1"
#define NS_REMOTEAGENT_CID \
  { 0x8f685a9d, 0x8181, 0x46d6, \
    { 0xa7, 0x1d, x86, x92, x89, x09, x9c, x6d } }

#endif /* __gen_nsIRemoteAgent_h__ */
