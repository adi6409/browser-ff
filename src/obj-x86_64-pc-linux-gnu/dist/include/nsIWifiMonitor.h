/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/wifi/nsIWifiMonitor.idl
 */

#ifndef __gen_nsIWifiMonitor_h__
#define __gen_nsIWifiMonitor_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIWifiListener; /* forward declaration */


/* starting interface:    nsIWifiMonitor */
#define NS_IWIFIMONITOR_IID_STR "f289701e-d9af-4685-bc2f-e4226ff7c018"

#define NS_IWIFIMONITOR_IID \
  {0xf289701e, 0xd9af, 0x4685, \
    { 0xbc, 0x2f, 0xe4, 0x22, 0x6f, 0xf7, 0xc0, 0x18 }}

class NS_NO_VTABLE nsIWifiMonitor : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWIFIMONITOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWifiMonitor;

  /* void startWatching (in nsIWifiListener aListener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD StartWatching(nsIWifiListener *aListener) = 0;

  /* void stopWatching (in nsIWifiListener aListener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD StopWatching(nsIWifiListener *aListener) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWifiMonitor, NS_IWIFIMONITOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWIFIMONITOR \
  NS_IMETHOD StartWatching(nsIWifiListener *aListener) override; \
  NS_IMETHOD StopWatching(nsIWifiListener *aListener) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWIFIMONITOR \
  nsresult StartWatching(nsIWifiListener *aListener); \
  nsresult StopWatching(nsIWifiListener *aListener); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWIFIMONITOR(_to) \
  NS_IMETHOD StartWatching(nsIWifiListener *aListener) override { return _to StartWatching(aListener); } \
  NS_IMETHOD StopWatching(nsIWifiListener *aListener) override { return _to StopWatching(aListener); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWIFIMONITOR(_to) \
  NS_IMETHOD StartWatching(nsIWifiListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StartWatching(aListener); } \
  NS_IMETHOD StopWatching(nsIWifiListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StopWatching(aListener); } 


#endif /* __gen_nsIWifiMonitor_h__ */
