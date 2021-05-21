/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsITaskbarProgress.idl
 */

#ifndef __gen_nsITaskbarProgress_h__
#define __gen_nsITaskbarProgress_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIBaseWindow_h__
#include "nsIBaseWindow.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
typedef int32_t  nsTaskbarProgressState;


/* starting interface:    nsITaskbarProgress */
#define NS_ITASKBARPROGRESS_IID_STR "23ac257d-ef3c-4033-b424-be7fef91a86c"

#define NS_ITASKBARPROGRESS_IID \
  {0x23ac257d, 0xef3c, 0x4033, \
    { 0xb4, 0x24, 0xbe, 0x7f, 0xef, 0x91, 0xa8, 0x6c }}

class NS_NO_VTABLE nsITaskbarProgress : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITASKBARPROGRESS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITaskbarProgress;

  enum {
    STATE_NO_PROGRESS = 0,
    STATE_INDETERMINATE = 1,
    STATE_NORMAL = 2,
    STATE_ERROR = 3,
    STATE_PAUSED = 4
  };

  /* void setProgressState (in nsTaskbarProgressState state, [optional] in unsigned long long currentValue, [optional] in unsigned long long maxValue); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetProgressState(nsTaskbarProgressState state, uint64_t currentValue, uint64_t maxValue) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITaskbarProgress, NS_ITASKBARPROGRESS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITASKBARPROGRESS \
  NS_IMETHOD SetProgressState(nsTaskbarProgressState state, uint64_t currentValue, uint64_t maxValue) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITASKBARPROGRESS \
  nsresult SetProgressState(nsTaskbarProgressState state, uint64_t currentValue, uint64_t maxValue); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITASKBARPROGRESS(_to) \
  NS_IMETHOD SetProgressState(nsTaskbarProgressState state, uint64_t currentValue, uint64_t maxValue) override { return _to SetProgressState(state, currentValue, maxValue); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITASKBARPROGRESS(_to) \
  NS_IMETHOD SetProgressState(nsTaskbarProgressState state, uint64_t currentValue, uint64_t maxValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetProgressState(state, currentValue, maxValue); } 


#endif /* __gen_nsITaskbarProgress_h__ */
