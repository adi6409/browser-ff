/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpfe/appshell/nsIWindowMediatorListener.idl
 */

#ifndef __gen_nsIWindowMediatorListener_h__
#define __gen_nsIWindowMediatorListener_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIAppWindow; /* forward declaration */


/* starting interface:    nsIWindowMediatorListener */
#define NS_IWINDOWMEDIATORLISTENER_IID_STR "2f276982-0d60-4377-a595-d350ba516395"

#define NS_IWINDOWMEDIATORLISTENER_IID \
  {0x2f276982, 0x0d60, 0x4377, \
    { 0xa5, 0x95, 0xd3, 0x50, 0xba, 0x51, 0x63, 0x95 }}

class NS_NO_VTABLE nsIWindowMediatorListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWINDOWMEDIATORLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWindowMediatorListener;

  /* void onOpenWindow (in nsIAppWindow window); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnOpenWindow(nsIAppWindow *window) = 0;

  /* void onCloseWindow (in nsIAppWindow window); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnCloseWindow(nsIAppWindow *window) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWindowMediatorListener, NS_IWINDOWMEDIATORLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWINDOWMEDIATORLISTENER \
  NS_IMETHOD OnOpenWindow(nsIAppWindow *window) override; \
  NS_IMETHOD OnCloseWindow(nsIAppWindow *window) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWINDOWMEDIATORLISTENER \
  nsresult OnOpenWindow(nsIAppWindow *window); \
  nsresult OnCloseWindow(nsIAppWindow *window); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWINDOWMEDIATORLISTENER(_to) \
  NS_IMETHOD OnOpenWindow(nsIAppWindow *window) override { return _to OnOpenWindow(window); } \
  NS_IMETHOD OnCloseWindow(nsIAppWindow *window) override { return _to OnCloseWindow(window); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWINDOWMEDIATORLISTENER(_to) \
  NS_IMETHOD OnOpenWindow(nsIAppWindow *window) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnOpenWindow(window); } \
  NS_IMETHOD OnCloseWindow(nsIAppWindow *window) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnCloseWindow(window); } 


#endif /* __gen_nsIWindowMediatorListener_h__ */
