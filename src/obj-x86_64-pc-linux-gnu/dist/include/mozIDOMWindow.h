/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/mozIDOMWindow.idl
 */

#ifndef __gen_mozIDOMWindow_h__
#define __gen_mozIDOMWindow_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    mozIDOMWindow */
#define MOZIDOMWINDOW_IID_STR "75fbabd6-7a2e-4787-aa33-449a33512135"

#define MOZIDOMWINDOW_IID \
  {0x75fbabd6, 0x7a2e, 0x4787, \
    { 0xaa, 0x33, 0x44, 0x9a, 0x33, 0x51, 0x21, 0x35 }}

class NS_NO_VTABLE mozIDOMWindow : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZIDOMWINDOW_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIDOMWindow;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIDOMWindow, MOZIDOMWINDOW_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZIDOMWINDOW \
  /* no methods! */

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZIDOMWINDOW \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZIDOMWINDOW(_to) \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZIDOMWINDOW(_to) \
  /* no methods! */


/* starting interface:    mozIDOMWindowProxy */
#define MOZIDOMWINDOWPROXY_IID_STR "53ca090c-e739-48b9-8911-208c72f9191e"

#define MOZIDOMWINDOWPROXY_IID \
  {0x53ca090c, 0xe739, 0x48b9, \
    { 0x89, 0x11, 0x20, 0x8c, 0x72, 0xf9, 0x19, 0x1e }}

class NS_NO_VTABLE mozIDOMWindowProxy : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZIDOMWINDOWPROXY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIDOMWindowProxy;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIDOMWindowProxy, MOZIDOMWINDOWPROXY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZIDOMWINDOWPROXY \
  /* no methods! */

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZIDOMWINDOWPROXY \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZIDOMWINDOWPROXY(_to) \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZIDOMWINDOWPROXY(_to) \
  /* no methods! */


#endif /* __gen_mozIDOMWindow_h__ */
