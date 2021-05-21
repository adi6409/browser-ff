/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIURIWithSpecialOrigin.idl
 */

#ifndef __gen_nsIURIWithSpecialOrigin_h__
#define __gen_nsIURIWithSpecialOrigin_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */


/* starting interface:    nsIURIWithSpecialOrigin */
#define NS_IURIWITHSPECIALORIGIN_IID_STR "4f65569b-d6fc-4580-94d9-21e775658a2a"

#define NS_IURIWITHSPECIALORIGIN_IID \
  {0x4f65569b, 0xd6fc, 0x4580, \
    { 0x94, 0xd9, 0x21, 0xe7, 0x75, 0x65, 0x8a, 0x2a }}

class NS_NO_VTABLE nsIURIWithSpecialOrigin : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURIWITHSPECIALORIGIN_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIURIWithSpecialOrigin;

  /* readonly attribute nsIURI origin; */
  NS_IMETHOD GetOrigin(nsIURI **aOrigin) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIURIWithSpecialOrigin, NS_IURIWITHSPECIALORIGIN_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURIWITHSPECIALORIGIN \
  NS_IMETHOD GetOrigin(nsIURI **aOrigin) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURIWITHSPECIALORIGIN \
  nsresult GetOrigin(nsIURI **aOrigin); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURIWITHSPECIALORIGIN(_to) \
  NS_IMETHOD GetOrigin(nsIURI **aOrigin) override { return _to GetOrigin(aOrigin); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURIWITHSPECIALORIGIN(_to) \
  NS_IMETHOD GetOrigin(nsIURI **aOrigin) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOrigin(aOrigin); } 


#endif /* __gen_nsIURIWithSpecialOrigin_h__ */
