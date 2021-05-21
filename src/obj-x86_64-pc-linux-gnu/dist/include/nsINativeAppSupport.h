/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/xre/nsINativeAppSupport.idl
 */

#ifndef __gen_nsINativeAppSupport_h__
#define __gen_nsINativeAppSupport_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsINativeAppSupport */
#define NS_INATIVEAPPSUPPORT_IID_STR "5fdf8480-1f98-11d4-8077-00600811a9c3"

#define NS_INATIVEAPPSUPPORT_IID \
  {0x5fdf8480, 0x1f98, 0x11d4, \
    { 0x80, 0x77, 0x00, 0x60, 0x08, 0x11, 0xa9, 0xc3 }}

class NS_NO_VTABLE nsINativeAppSupport : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INATIVEAPPSUPPORT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINativeAppSupport;

  /* boolean start (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Start(bool *_retval) = 0;

  /* void enable (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Enable(void) = 0;

  /* void onLastWindowClosing (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnLastWindowClosing(void) = 0;

  /* void ReOpen (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ReOpen(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINativeAppSupport, NS_INATIVEAPPSUPPORT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINATIVEAPPSUPPORT \
  NS_IMETHOD Start(bool *_retval) override; \
  NS_IMETHOD Enable(void) override; \
  NS_IMETHOD OnLastWindowClosing(void) override; \
  NS_IMETHOD ReOpen(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINATIVEAPPSUPPORT \
  nsresult Start(bool *_retval); \
  nsresult Enable(void); \
  nsresult OnLastWindowClosing(void); \
  nsresult ReOpen(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINATIVEAPPSUPPORT(_to) \
  NS_IMETHOD Start(bool *_retval) override { return _to Start(_retval); } \
  NS_IMETHOD Enable(void) override { return _to Enable(); } \
  NS_IMETHOD OnLastWindowClosing(void) override { return _to OnLastWindowClosing(); } \
  NS_IMETHOD ReOpen(void) override { return _to ReOpen(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINATIVEAPPSUPPORT(_to) \
  NS_IMETHOD Start(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Start(_retval); } \
  NS_IMETHOD Enable(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Enable(); } \
  NS_IMETHOD OnLastWindowClosing(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnLastWindowClosing(); } \
  NS_IMETHOD ReOpen(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReOpen(); } 


#endif /* __gen_nsINativeAppSupport_h__ */
