/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/antitracking/nsIURLDecorationAnnotationsService.idl
 */

#ifndef __gen_nsIURLDecorationAnnotationsService_h__
#define __gen_nsIURLDecorationAnnotationsService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIURLDecorationAnnotationsService */
#define NS_IURLDECORATIONANNOTATIONSSERVICE_IID_STR "937d0c66-6821-4e3f-9e04-50dbc2b2b476"

#define NS_IURLDECORATIONANNOTATIONSSERVICE_IID \
  {0x937d0c66, 0x6821, 0x4e3f, \
    { 0x9e, 0x04, 0x50, 0xdb, 0xc2, 0xb2, 0xb4, 0x76 }}

class NS_NO_VTABLE nsIURLDecorationAnnotationsService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURLDECORATIONANNOTATIONSSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIURLDecorationAnnotationsService;

  /* Promise ensureUpdated (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EnsureUpdated(::mozilla::dom::Promise * * _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIURLDecorationAnnotationsService, NS_IURLDECORATIONANNOTATIONSSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURLDECORATIONANNOTATIONSSERVICE \
  NS_IMETHOD EnsureUpdated(::mozilla::dom::Promise * * _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURLDECORATIONANNOTATIONSSERVICE \
  nsresult EnsureUpdated(::mozilla::dom::Promise * * _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURLDECORATIONANNOTATIONSSERVICE(_to) \
  NS_IMETHOD EnsureUpdated(::mozilla::dom::Promise * * _retval) override { return _to EnsureUpdated(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURLDECORATIONANNOTATIONSSERVICE(_to) \
  NS_IMETHOD EnsureUpdated(::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnsureUpdated(_retval); } 


#endif /* __gen_nsIURLDecorationAnnotationsService_h__ */
