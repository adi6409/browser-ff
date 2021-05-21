/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/media/autoplay/nsIAutoplay.idl
 */

#ifndef __gen_nsIAutoplay_h__
#define __gen_nsIAutoplay_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIAutoplay */
#define NS_IAUTOPLAY_IID_STR "048a24f6-c4d6-47bc-bea2-f6038d1db80a"

#define NS_IAUTOPLAY_IID \
  {0x048a24f6, 0xc4d6, 0x47bc, \
    { 0xbe, 0xa2, 0xf6, 0x03, 0x8d, 0x1d, 0xb8, 0x0a }}

class NS_NO_VTABLE nsIAutoplay : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IAUTOPLAY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAutoplay;

  enum {
    ALLOWED = 0U,
    BLOCKED = 1U,
    BLOCKED_ALL = 5U
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAutoplay, NS_IAUTOPLAY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIAUTOPLAY \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIAUTOPLAY \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIAUTOPLAY(_to) \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIAUTOPLAY(_to) \


#endif /* __gen_nsIAutoplay_h__ */
