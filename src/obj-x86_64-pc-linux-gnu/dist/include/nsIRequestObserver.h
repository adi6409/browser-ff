/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIRequestObserver.idl
 */

#ifndef __gen_nsIRequestObserver_h__
#define __gen_nsIRequestObserver_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIRequest; /* forward declaration */


/* starting interface:    nsIRequestObserver */
#define NS_IREQUESTOBSERVER_IID_STR "fd91e2e0-1481-11d3-9333-00104ba0fd40"

#define NS_IREQUESTOBSERVER_IID \
  {0xfd91e2e0, 0x1481, 0x11d3, \
    { 0x93, 0x33, 0x00, 0x10, 0x4b, 0xa0, 0xfd, 0x40 }}

class NS_NO_VTABLE nsIRequestObserver : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IREQUESTOBSERVER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIRequestObserver;

  /* void onStartRequest (in nsIRequest aRequest); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnStartRequest(nsIRequest *aRequest) = 0;

  /* void onStopRequest (in nsIRequest aRequest, in nsresult aStatusCode); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnStopRequest(nsIRequest *aRequest, nsresult aStatusCode) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIRequestObserver, NS_IREQUESTOBSERVER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIREQUESTOBSERVER \
  NS_IMETHOD OnStartRequest(nsIRequest *aRequest) override; \
  NS_IMETHOD OnStopRequest(nsIRequest *aRequest, nsresult aStatusCode) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIREQUESTOBSERVER \
  nsresult OnStartRequest(nsIRequest *aRequest); \
  nsresult OnStopRequest(nsIRequest *aRequest, nsresult aStatusCode); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIREQUESTOBSERVER(_to) \
  NS_IMETHOD OnStartRequest(nsIRequest *aRequest) override { return _to OnStartRequest(aRequest); } \
  NS_IMETHOD OnStopRequest(nsIRequest *aRequest, nsresult aStatusCode) override { return _to OnStopRequest(aRequest, aStatusCode); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIREQUESTOBSERVER(_to) \
  NS_IMETHOD OnStartRequest(nsIRequest *aRequest) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnStartRequest(aRequest); } \
  NS_IMETHOD OnStopRequest(nsIRequest *aRequest, nsresult aStatusCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnStopRequest(aRequest, aStatusCode); } 


#endif /* __gen_nsIRequestObserver_h__ */
