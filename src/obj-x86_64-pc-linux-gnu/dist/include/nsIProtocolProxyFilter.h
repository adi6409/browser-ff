/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIProtocolProxyFilter.idl
 */

#ifndef __gen_nsIProtocolProxyFilter_h__
#define __gen_nsIProtocolProxyFilter_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIChannel; /* forward declaration */

class nsIProtocolProxyService; /* forward declaration */

class nsIProxyInfo; /* forward declaration */

class nsIURI; /* forward declaration */


/* starting interface:    nsIProxyProtocolFilterResult */
#define NS_IPROXYPROTOCOLFILTERRESULT_IID_STR "009e6c3f-fb64-40c5-8093-f1495c64773e"

#define NS_IPROXYPROTOCOLFILTERRESULT_IID \
  {0x009e6c3f, 0xfb64, 0x40c5, \
    { 0x80, 0x93, 0xf1, 0x49, 0x5c, 0x64, 0x77, 0x3e }}

class NS_NO_VTABLE nsIProxyProtocolFilterResult : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPROXYPROTOCOLFILTERRESULT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIProxyProtocolFilterResult;

  /* void onProxyFilterResult (in nsIProxyInfo aProxy); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnProxyFilterResult(nsIProxyInfo *aProxy) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIProxyProtocolFilterResult, NS_IPROXYPROTOCOLFILTERRESULT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPROXYPROTOCOLFILTERRESULT \
  NS_IMETHOD OnProxyFilterResult(nsIProxyInfo *aProxy) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPROXYPROTOCOLFILTERRESULT \
  nsresult OnProxyFilterResult(nsIProxyInfo *aProxy); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPROXYPROTOCOLFILTERRESULT(_to) \
  NS_IMETHOD OnProxyFilterResult(nsIProxyInfo *aProxy) override { return _to OnProxyFilterResult(aProxy); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPROXYPROTOCOLFILTERRESULT(_to) \
  NS_IMETHOD OnProxyFilterResult(nsIProxyInfo *aProxy) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnProxyFilterResult(aProxy); } 


/* starting interface:    nsIProtocolProxyFilter */
#define NS_IPROTOCOLPROXYFILTER_IID_STR "f424abd3-32b4-456c-9f45-b7e3376cb0d1"

#define NS_IPROTOCOLPROXYFILTER_IID \
  {0xf424abd3, 0x32b4, 0x456c, \
    { 0x9f, 0x45, 0xb7, 0xe3, 0x37, 0x6c, 0xb0, 0xd1 }}

class NS_NO_VTABLE nsIProtocolProxyFilter : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPROTOCOLPROXYFILTER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIProtocolProxyFilter;

  /* void applyFilter (in nsIURI aURI, in nsIProxyInfo aProxy, in nsIProxyProtocolFilterResult aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ApplyFilter(nsIURI *aURI, nsIProxyInfo *aProxy, nsIProxyProtocolFilterResult *aCallback) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIProtocolProxyFilter, NS_IPROTOCOLPROXYFILTER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPROTOCOLPROXYFILTER \
  NS_IMETHOD ApplyFilter(nsIURI *aURI, nsIProxyInfo *aProxy, nsIProxyProtocolFilterResult *aCallback) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPROTOCOLPROXYFILTER \
  nsresult ApplyFilter(nsIURI *aURI, nsIProxyInfo *aProxy, nsIProxyProtocolFilterResult *aCallback); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPROTOCOLPROXYFILTER(_to) \
  NS_IMETHOD ApplyFilter(nsIURI *aURI, nsIProxyInfo *aProxy, nsIProxyProtocolFilterResult *aCallback) override { return _to ApplyFilter(aURI, aProxy, aCallback); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPROTOCOLPROXYFILTER(_to) \
  NS_IMETHOD ApplyFilter(nsIURI *aURI, nsIProxyInfo *aProxy, nsIProxyProtocolFilterResult *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ApplyFilter(aURI, aProxy, aCallback); } 


/* starting interface:    nsIProtocolProxyChannelFilter */
#define NS_IPROTOCOLPROXYCHANNELFILTER_IID_STR "245b0880-82c5-4e6e-be6d-bc586aa55a90"

#define NS_IPROTOCOLPROXYCHANNELFILTER_IID \
  {0x245b0880, 0x82c5, 0x4e6e, \
    { 0xbe, 0x6d, 0xbc, 0x58, 0x6a, 0xa5, 0x5a, 0x90 }}

class NS_NO_VTABLE nsIProtocolProxyChannelFilter : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPROTOCOLPROXYCHANNELFILTER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIProtocolProxyChannelFilter;

  /* void applyFilter (in nsIChannel aChannel, in nsIProxyInfo aProxy, in nsIProxyProtocolFilterResult aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ApplyFilter(nsIChannel *aChannel, nsIProxyInfo *aProxy, nsIProxyProtocolFilterResult *aCallback) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIProtocolProxyChannelFilter, NS_IPROTOCOLPROXYCHANNELFILTER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPROTOCOLPROXYCHANNELFILTER \
  NS_IMETHOD ApplyFilter(nsIChannel *aChannel, nsIProxyInfo *aProxy, nsIProxyProtocolFilterResult *aCallback) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPROTOCOLPROXYCHANNELFILTER \
  nsresult ApplyFilter(nsIChannel *aChannel, nsIProxyInfo *aProxy, nsIProxyProtocolFilterResult *aCallback); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPROTOCOLPROXYCHANNELFILTER(_to) \
  NS_IMETHOD ApplyFilter(nsIChannel *aChannel, nsIProxyInfo *aProxy, nsIProxyProtocolFilterResult *aCallback) override { return _to ApplyFilter(aChannel, aProxy, aCallback); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPROTOCOLPROXYCHANNELFILTER(_to) \
  NS_IMETHOD ApplyFilter(nsIChannel *aChannel, nsIProxyInfo *aProxy, nsIProxyProtocolFilterResult *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ApplyFilter(aChannel, aProxy, aCallback); } 


#endif /* __gen_nsIProtocolProxyFilter_h__ */
