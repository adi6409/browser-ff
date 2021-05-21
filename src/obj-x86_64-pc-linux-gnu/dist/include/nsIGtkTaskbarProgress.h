/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIGtkTaskbarProgress.idl
 */

#ifndef __gen_nsIGtkTaskbarProgress_h__
#define __gen_nsIGtkTaskbarProgress_h__


#ifndef __gen_nsITaskbarProgress_h__
#include "nsITaskbarProgress.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIDOMWindowProxy; /* forward declaration */


/* starting interface:    nsIGtkTaskbarProgress */
#define NS_IGTKTASKBARPROGRESS_IID_STR "39f6fc5a-2386-4bc6-941c-d7479253bc3f"

#define NS_IGTKTASKBARPROGRESS_IID \
  {0x39f6fc5a, 0x2386, 0x4bc6, \
    { 0x94, 0x1c, 0xd7, 0x47, 0x92, 0x53, 0xbc, 0x3f }}

class NS_NO_VTABLE nsIGtkTaskbarProgress : public nsITaskbarProgress {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IGTKTASKBARPROGRESS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIGtkTaskbarProgress;

  /* void setPrimaryWindow (in mozIDOMWindowProxy aWindow); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPrimaryWindow(mozIDOMWindowProxy *aWindow) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIGtkTaskbarProgress, NS_IGTKTASKBARPROGRESS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIGTKTASKBARPROGRESS \
  NS_IMETHOD SetPrimaryWindow(mozIDOMWindowProxy *aWindow) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIGTKTASKBARPROGRESS \
  nsresult SetPrimaryWindow(mozIDOMWindowProxy *aWindow); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIGTKTASKBARPROGRESS(_to) \
  NS_IMETHOD SetPrimaryWindow(mozIDOMWindowProxy *aWindow) override { return _to SetPrimaryWindow(aWindow); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIGTKTASKBARPROGRESS(_to) \
  NS_IMETHOD SetPrimaryWindow(mozIDOMWindowProxy *aWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPrimaryWindow(aWindow); } 


#endif /* __gen_nsIGtkTaskbarProgress_h__ */
