/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/wifi/nsIWifiListener.idl
 */

#ifndef __gen_nsIWifiListener_h__
#define __gen_nsIWifiListener_h__


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
class nsIWifiAccessPoint; /* forward declaration */


/* starting interface:    nsIWifiListener */
#define NS_IWIFILISTENER_IID_STR "bcd4bede-f4a5-4a62-9071-d7a60174e376"

#define NS_IWIFILISTENER_IID \
  {0xbcd4bede, 0xf4a5, 0x4a62, \
    { 0x90, 0x71, 0xd7, 0xa6, 0x01, 0x74, 0xe3, 0x76 }}

class NS_NO_VTABLE nsIWifiListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWIFILISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWifiListener;

  /* void onChange (in Array<nsIWifiAccessPoint> accessPoints); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnChange(const nsTArray<RefPtr<nsIWifiAccessPoint>>& accessPoints) = 0;

  /* void onError (in nsresult error); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnError(nsresult error) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWifiListener, NS_IWIFILISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWIFILISTENER \
  NS_IMETHOD OnChange(const nsTArray<RefPtr<nsIWifiAccessPoint>>& accessPoints) override; \
  NS_IMETHOD OnError(nsresult error) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWIFILISTENER \
  nsresult OnChange(const nsTArray<RefPtr<nsIWifiAccessPoint>>& accessPoints); \
  nsresult OnError(nsresult error); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWIFILISTENER(_to) \
  NS_IMETHOD OnChange(const nsTArray<RefPtr<nsIWifiAccessPoint>>& accessPoints) override { return _to OnChange(accessPoints); } \
  NS_IMETHOD OnError(nsresult error) override { return _to OnError(error); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWIFILISTENER(_to) \
  NS_IMETHOD OnChange(const nsTArray<RefPtr<nsIWifiAccessPoint>>& accessPoints) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnChange(accessPoints); } \
  NS_IMETHOD OnError(nsresult error) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnError(error); } 


#endif /* __gen_nsIWifiListener_h__ */
