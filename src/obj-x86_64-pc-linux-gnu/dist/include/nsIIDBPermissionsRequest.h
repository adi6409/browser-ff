/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/indexedDB/nsIIDBPermissionsRequest.idl
 */

#ifndef __gen_nsIIDBPermissionsRequest_h__
#define __gen_nsIIDBPermissionsRequest_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIObserver; /* forward declaration */

namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIIDBPermissionsRequest */
#define NS_IIDBPERMISSIONSREQUEST_IID_STR "c3493c65-0530-496e-995c-bcd38dbfce21"

#define NS_IIDBPERMISSIONSREQUEST_IID \
  {0xc3493c65, 0x0530, 0x496e, \
    { 0x99, 0x5c, 0xbc, 0xd3, 0x8d, 0xbf, 0xce, 0x21 }}

class NS_NO_VTABLE nsIIDBPermissionsRequest : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IIDBPERMISSIONSREQUEST_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIIDBPermissionsRequest;

  /* readonly attribute Element browserElement; */
  NS_IMETHOD GetBrowserElement(mozilla::dom::Element **aBrowserElement) = 0;

  /* readonly attribute nsIObserver responseObserver; */
  NS_IMETHOD GetResponseObserver(nsIObserver **aResponseObserver) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIIDBPermissionsRequest, NS_IIDBPERMISSIONSREQUEST_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIIDBPERMISSIONSREQUEST \
  NS_IMETHOD GetBrowserElement(mozilla::dom::Element **aBrowserElement) override; \
  NS_IMETHOD GetResponseObserver(nsIObserver **aResponseObserver) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIIDBPERMISSIONSREQUEST \
  nsresult GetBrowserElement(mozilla::dom::Element **aBrowserElement); \
  nsresult GetResponseObserver(nsIObserver **aResponseObserver); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIIDBPERMISSIONSREQUEST(_to) \
  NS_IMETHOD GetBrowserElement(mozilla::dom::Element **aBrowserElement) override { return _to GetBrowserElement(aBrowserElement); } \
  NS_IMETHOD GetResponseObserver(nsIObserver **aResponseObserver) override { return _to GetResponseObserver(aResponseObserver); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIIDBPERMISSIONSREQUEST(_to) \
  NS_IMETHOD GetBrowserElement(mozilla::dom::Element **aBrowserElement) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBrowserElement(aBrowserElement); } \
  NS_IMETHOD GetResponseObserver(nsIObserver **aResponseObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResponseObserver(aResponseObserver); } 


#endif /* __gen_nsIIDBPermissionsRequest_h__ */
