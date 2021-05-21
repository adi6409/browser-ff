/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cookie/nsICookiePermission.idl
 */

#ifndef __gen_nsICookiePermission_h__
#define __gen_nsICookiePermission_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
typedef int32_t  nsCookieAccess;


/* starting interface:    nsICookiePermission */
#define NS_ICOOKIEPERMISSION_IID_STR "11ddd4ed-8f5b-40b3-b2a0-27c20ea1c88d"

#define NS_ICOOKIEPERMISSION_IID \
  {0x11ddd4ed, 0x8f5b, 0x40b3, \
    { 0xb2, 0xa0, 0x27, 0xc2, 0x0e, 0xa1, 0xc8, 0x8d }}

class NS_NO_VTABLE nsICookiePermission : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICOOKIEPERMISSION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICookiePermission;

  enum {
    ACCESS_DEFAULT = 0,
    ACCESS_ALLOW = 1,
    ACCESS_DENY = 2,
    ACCESS_SESSION = 8
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICookiePermission, NS_ICOOKIEPERMISSION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICOOKIEPERMISSION \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICOOKIEPERMISSION \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICOOKIEPERMISSION(_to) \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICOOKIEPERMISSION(_to) \


#endif /* __gen_nsICookiePermission_h__ */
