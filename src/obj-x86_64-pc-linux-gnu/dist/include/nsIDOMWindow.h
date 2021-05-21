/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIDOMWindow.idl
 */

#ifndef __gen_nsIDOMWindow_h__
#define __gen_nsIDOMWindow_h__


#ifndef __gen_domstubs_h__
#include "domstubs.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIControllers; /* forward declaration */

class nsIPrompt; /* forward declaration */

class nsIVariant; /* forward declaration */


/* starting interface:    nsIDOMWindow */
#define NS_IDOMWINDOW_IID_STR "b8343993-0383-4add-9930-ad176b189240"

#define NS_IDOMWINDOW_IID \
  {0xb8343993, 0x0383, 0x4add, \
    { 0x99, 0x30, 0xad, 0x17, 0x6b, 0x18, 0x92, 0x40 }}

class NS_NO_VTABLE nsIDOMWindow : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOMWINDOW_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDOMWindow;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDOMWindow, NS_IDOMWINDOW_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOMWINDOW \
  /* no methods! */

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOMWINDOW \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOMWINDOW(_to) \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOMWINDOW(_to) \
  /* no methods! */


#endif /* __gen_nsIDOMWindow_h__ */
