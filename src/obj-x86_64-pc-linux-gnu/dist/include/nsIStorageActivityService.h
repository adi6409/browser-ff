/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/storage/nsIStorageActivityService.idl
 */

#ifndef __gen_nsIStorageActivityService_h__
#define __gen_nsIStorageActivityService_h__


#ifndef __gen_domstubs_h__
#include "domstubs.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIArray; /* forward declaration */

class nsIPrincipal; /* forward declaration */


/* starting interface:    nsIStorageActivityService */
#define NS_ISTORAGEACTIVITYSERVICE_IID_STR "fd1310ba-d1be-4327-988e-92b39fcff6f4"

#define NS_ISTORAGEACTIVITYSERVICE_IID \
  {0xfd1310ba, 0xd1be, 0x4327, \
    { 0x98, 0x8e, 0x92, 0xb3, 0x9f, 0xcf, 0xf6, 0xf4 }}

class NS_NO_VTABLE nsIStorageActivityService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISTORAGEACTIVITYSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIStorageActivityService;

  /* nsIArray getActiveOrigins (in PRTime from, in PRTime to); */
  NS_IMETHOD GetActiveOrigins(PRTime from, PRTime to, nsIArray **_retval) = 0;

  /* void moveOriginInTime (in nsIPrincipal origin, in PRTime when); */
  NS_IMETHOD MoveOriginInTime(nsIPrincipal *origin, PRTime when) = 0;

  /* void testOnlyReset (); */
  NS_IMETHOD TestOnlyReset(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIStorageActivityService, NS_ISTORAGEACTIVITYSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISTORAGEACTIVITYSERVICE \
  NS_IMETHOD GetActiveOrigins(PRTime from, PRTime to, nsIArray **_retval) override; \
  NS_IMETHOD MoveOriginInTime(nsIPrincipal *origin, PRTime when) override; \
  NS_IMETHOD TestOnlyReset(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISTORAGEACTIVITYSERVICE \
  nsresult GetActiveOrigins(PRTime from, PRTime to, nsIArray **_retval); \
  nsresult MoveOriginInTime(nsIPrincipal *origin, PRTime when); \
  nsresult TestOnlyReset(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISTORAGEACTIVITYSERVICE(_to) \
  NS_IMETHOD GetActiveOrigins(PRTime from, PRTime to, nsIArray **_retval) override { return _to GetActiveOrigins(from, to, _retval); } \
  NS_IMETHOD MoveOriginInTime(nsIPrincipal *origin, PRTime when) override { return _to MoveOriginInTime(origin, when); } \
  NS_IMETHOD TestOnlyReset(void) override { return _to TestOnlyReset(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISTORAGEACTIVITYSERVICE(_to) \
  NS_IMETHOD GetActiveOrigins(PRTime from, PRTime to, nsIArray **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetActiveOrigins(from, to, _retval); } \
  NS_IMETHOD MoveOriginInTime(nsIPrincipal *origin, PRTime when) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MoveOriginInTime(origin, when); } \
  NS_IMETHOD TestOnlyReset(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TestOnlyReset(); } 

#define STORAGE_ACTIVITY_SERVICE_CONTRACTID "@mozilla.org/storage/activity-service;1"

#endif /* __gen_nsIStorageActivityService_h__ */
