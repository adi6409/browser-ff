/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/modules/libpref/nsIPrefLocalizedString.idl
 */

#ifndef __gen_nsIPrefLocalizedString_h__
#define __gen_nsIPrefLocalizedString_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsISupportsPrimitives_h__
#include "nsISupportsPrimitives.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIPrefLocalizedString */
#define NS_IPREFLOCALIZEDSTRING_IID_STR "ae419e24-1dd1-11b2-b39a-d3e5e7073802"

#define NS_IPREFLOCALIZEDSTRING_IID \
  {0xae419e24, 0x1dd1, 0x11b2, \
    { 0xb3, 0x9a, 0xd3, 0xe5, 0xe7, 0x07, 0x38, 0x02 }}

class NS_NO_VTABLE nsIPrefLocalizedString : public nsISupportsString {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPREFLOCALIZEDSTRING_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPrefLocalizedString;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPrefLocalizedString, NS_IPREFLOCALIZEDSTRING_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPREFLOCALIZEDSTRING \
  /* no methods! */

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPREFLOCALIZEDSTRING \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPREFLOCALIZEDSTRING(_to) \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPREFLOCALIZEDSTRING(_to) \
  /* no methods! */


#define NS_PREFLOCALIZEDSTRING_CID                     \
  { /* {064d9cee-1dd2-11b2-83e3-d25ab0193c26} */       \
    0x064d9cee,                                        \
    0x1dd2,                                            \
    0x11b2,                                            \
    { 0x83, 0xe3, 0xd2, 0x5a, 0xb0, 0x19, 0x3c, 0x26 } \
  }
#define NS_PREFLOCALIZEDSTRING_CONTRACTID "@mozilla.org/pref-localizedstring;1"

#endif /* __gen_nsIPrefLocalizedString_h__ */
