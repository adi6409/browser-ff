/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsINetworkPredictorVerifier.idl
 */

#ifndef __gen_nsINetworkPredictorVerifier_h__
#define __gen_nsINetworkPredictorVerifier_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */


/* starting interface:    nsINetworkPredictorVerifier */
#define NS_INETWORKPREDICTORVERIFIER_IID_STR "2e43bb32-dabf-4494-9f90-2b3195b1c73d"

#define NS_INETWORKPREDICTORVERIFIER_IID \
  {0x2e43bb32, 0xdabf, 0x4494, \
    { 0x9f, 0x90, 0x2b, 0x31, 0x95, 0xb1, 0xc7, 0x3d }}

class NS_NO_VTABLE nsINetworkPredictorVerifier : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INETWORKPREDICTORVERIFIER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINetworkPredictorVerifier;

  /* void onPredictPrefetch (in nsIURI uri, in uint32_t status); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnPredictPrefetch(nsIURI *uri, uint32_t status) = 0;

  /* void onPredictPreconnect (in nsIURI uri); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnPredictPreconnect(nsIURI *uri) = 0;

  /* void onPredictDNS (in nsIURI uri); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnPredictDNS(nsIURI *uri) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINetworkPredictorVerifier, NS_INETWORKPREDICTORVERIFIER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINETWORKPREDICTORVERIFIER \
  NS_IMETHOD OnPredictPrefetch(nsIURI *uri, uint32_t status) override; \
  NS_IMETHOD OnPredictPreconnect(nsIURI *uri) override; \
  NS_IMETHOD OnPredictDNS(nsIURI *uri) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINETWORKPREDICTORVERIFIER \
  nsresult OnPredictPrefetch(nsIURI *uri, uint32_t status); \
  nsresult OnPredictPreconnect(nsIURI *uri); \
  nsresult OnPredictDNS(nsIURI *uri); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINETWORKPREDICTORVERIFIER(_to) \
  NS_IMETHOD OnPredictPrefetch(nsIURI *uri, uint32_t status) override { return _to OnPredictPrefetch(uri, status); } \
  NS_IMETHOD OnPredictPreconnect(nsIURI *uri) override { return _to OnPredictPreconnect(uri); } \
  NS_IMETHOD OnPredictDNS(nsIURI *uri) override { return _to OnPredictDNS(uri); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINETWORKPREDICTORVERIFIER(_to) \
  NS_IMETHOD OnPredictPrefetch(nsIURI *uri, uint32_t status) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnPredictPrefetch(uri, status); } \
  NS_IMETHOD OnPredictPreconnect(nsIURI *uri) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnPredictPreconnect(uri); } \
  NS_IMETHOD OnPredictDNS(nsIURI *uri) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnPredictDNS(uri); } 


#endif /* __gen_nsINetworkPredictorVerifier_h__ */
