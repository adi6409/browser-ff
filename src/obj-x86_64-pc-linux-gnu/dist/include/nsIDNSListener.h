/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/dns/nsIDNSListener.idl
 */

#ifndef __gen_nsIDNSListener_h__
#define __gen_nsIDNSListener_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsICancelable; /* forward declaration */

class nsIDNSRecord; /* forward declaration */

class nsIDNSByTypeRecord; /* forward declaration */


/* starting interface:    nsIDNSListener */
#define NS_IDNSLISTENER_IID_STR "27d49bfe-280c-49e0-bbaa-f6200c232c3d"

#define NS_IDNSLISTENER_IID \
  {0x27d49bfe, 0x280c, 0x49e0, \
    { 0xbb, 0xaa, 0xf6, 0x20, 0x0c, 0x23, 0x2c, 0x3d }}

class NS_NO_VTABLE nsIDNSListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDNSLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDNSListener;

  /* void onLookupComplete (in nsICancelable aRequest, in nsIDNSRecord aRecord, in nsresult aStatus); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnLookupComplete(nsICancelable *aRequest, nsIDNSRecord *aRecord, nsresult aStatus) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDNSListener, NS_IDNSLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDNSLISTENER \
  NS_IMETHOD OnLookupComplete(nsICancelable *aRequest, nsIDNSRecord *aRecord, nsresult aStatus) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDNSLISTENER \
  nsresult OnLookupComplete(nsICancelable *aRequest, nsIDNSRecord *aRecord, nsresult aStatus); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDNSLISTENER(_to) \
  NS_IMETHOD OnLookupComplete(nsICancelable *aRequest, nsIDNSRecord *aRecord, nsresult aStatus) override { return _to OnLookupComplete(aRequest, aRecord, aStatus); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDNSLISTENER(_to) \
  NS_IMETHOD OnLookupComplete(nsICancelable *aRequest, nsIDNSRecord *aRecord, nsresult aStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnLookupComplete(aRequest, aRecord, aStatus); } 


#endif /* __gen_nsIDNSListener_h__ */
