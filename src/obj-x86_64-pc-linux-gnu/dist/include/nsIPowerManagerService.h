/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/power/nsIPowerManagerService.idl
 */

#ifndef __gen_nsIPowerManagerService_h__
#define __gen_nsIPowerManagerService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#define NS_POWERMANAGERSERVICE_CID { 0x18c2e238, 0x3a0a, 0x4153, {0x89, 0xfc, 0x16, 0x6b, 0x3b, 0x14, 0x65, 0xa1 } }
#define POWERMANAGERSERVICE_CONTRACTID "@mozilla.org/power/powermanagerservice;1"
class nsIDOMMozWakeLockListener; /* forward declaration */

class mozIDOMWindow; /* forward declaration */

class nsIWakeLock; /* forward declaration */


/* starting interface:    nsIPowerManagerService */
#define NS_IPOWERMANAGERSERVICE_IID_STR "ba7ca4c1-9d92-4425-a83b-85dd7fa953f7"

#define NS_IPOWERMANAGERSERVICE_IID \
  {0xba7ca4c1, 0x9d92, 0x4425, \
    { 0xa8, 0x3b, 0x85, 0xdd, 0x7f, 0xa9, 0x53, 0xf7 }}

class NS_NO_VTABLE nsIPowerManagerService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPOWERMANAGERSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPowerManagerService;

  /* void addWakeLockListener (in nsIDOMMozWakeLockListener aListener); */
  NS_IMETHOD AddWakeLockListener(nsIDOMMozWakeLockListener *aListener) = 0;

  /* void removeWakeLockListener (in nsIDOMMozWakeLockListener aListener); */
  NS_IMETHOD RemoveWakeLockListener(nsIDOMMozWakeLockListener *aListener) = 0;

  /* AString getWakeLockState (in AString aTopic); */
  NS_IMETHOD GetWakeLockState(const nsAString& aTopic, nsAString& _retval) = 0;

  /* nsIWakeLock newWakeLock (in AString aTopic, [optional] in mozIDOMWindow aWindow); */
  NS_IMETHOD NewWakeLock(const nsAString& aTopic, mozIDOMWindow *aWindow, nsIWakeLock **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPowerManagerService, NS_IPOWERMANAGERSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPOWERMANAGERSERVICE \
  NS_IMETHOD AddWakeLockListener(nsIDOMMozWakeLockListener *aListener) override; \
  NS_IMETHOD RemoveWakeLockListener(nsIDOMMozWakeLockListener *aListener) override; \
  NS_IMETHOD GetWakeLockState(const nsAString& aTopic, nsAString& _retval) override; \
  NS_IMETHOD NewWakeLock(const nsAString& aTopic, mozIDOMWindow *aWindow, nsIWakeLock **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPOWERMANAGERSERVICE \
  nsresult AddWakeLockListener(nsIDOMMozWakeLockListener *aListener); \
  nsresult RemoveWakeLockListener(nsIDOMMozWakeLockListener *aListener); \
  nsresult GetWakeLockState(const nsAString& aTopic, nsAString& _retval); \
  nsresult NewWakeLock(const nsAString& aTopic, mozIDOMWindow *aWindow, nsIWakeLock **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPOWERMANAGERSERVICE(_to) \
  NS_IMETHOD AddWakeLockListener(nsIDOMMozWakeLockListener *aListener) override { return _to AddWakeLockListener(aListener); } \
  NS_IMETHOD RemoveWakeLockListener(nsIDOMMozWakeLockListener *aListener) override { return _to RemoveWakeLockListener(aListener); } \
  NS_IMETHOD GetWakeLockState(const nsAString& aTopic, nsAString& _retval) override { return _to GetWakeLockState(aTopic, _retval); } \
  NS_IMETHOD NewWakeLock(const nsAString& aTopic, mozIDOMWindow *aWindow, nsIWakeLock **_retval) override { return _to NewWakeLock(aTopic, aWindow, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPOWERMANAGERSERVICE(_to) \
  NS_IMETHOD AddWakeLockListener(nsIDOMMozWakeLockListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddWakeLockListener(aListener); } \
  NS_IMETHOD RemoveWakeLockListener(nsIDOMMozWakeLockListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveWakeLockListener(aListener); } \
  NS_IMETHOD GetWakeLockState(const nsAString& aTopic, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWakeLockState(aTopic, _retval); } \
  NS_IMETHOD NewWakeLock(const nsAString& aTopic, mozIDOMWindow *aWindow, nsIWakeLock **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewWakeLock(aTopic, aWindow, _retval); } 


#endif /* __gen_nsIPowerManagerService_h__ */
