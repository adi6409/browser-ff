/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/timermanager/nsIUpdateTimerManager.idl
 */

#ifndef __gen_nsIUpdateTimerManager_h__
#define __gen_nsIUpdateTimerManager_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsITimerCallback; /* forward declaration */


/* starting interface:    nsIUpdateTimerManager */
#define NS_IUPDATETIMERMANAGER_IID_STR "0765c92c-6145-4253-9db4-594d8023087e"

#define NS_IUPDATETIMERMANAGER_IID \
  {0x0765c92c, 0x6145, 0x4253, \
    { 0x9d, 0xb4, 0x59, 0x4d, 0x80, 0x23, 0x08, 0x7e }}

class NS_NO_VTABLE nsIUpdateTimerManager : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IUPDATETIMERMANAGER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUpdateTimerManager;

  /* void registerTimer (in AString id, in nsITimerCallback callback, in unsigned long interval, [optional] in boolean skipFirst); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RegisterTimer(const nsAString& id, nsITimerCallback *callback, uint32_t interval, bool skipFirst) = 0;

  /* void unregisterTimer (in AString id); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UnregisterTimer(const nsAString& id) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUpdateTimerManager, NS_IUPDATETIMERMANAGER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIUPDATETIMERMANAGER \
  NS_IMETHOD RegisterTimer(const nsAString& id, nsITimerCallback *callback, uint32_t interval, bool skipFirst) override; \
  NS_IMETHOD UnregisterTimer(const nsAString& id) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIUPDATETIMERMANAGER \
  nsresult RegisterTimer(const nsAString& id, nsITimerCallback *callback, uint32_t interval, bool skipFirst); \
  nsresult UnregisterTimer(const nsAString& id); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIUPDATETIMERMANAGER(_to) \
  NS_IMETHOD RegisterTimer(const nsAString& id, nsITimerCallback *callback, uint32_t interval, bool skipFirst) override { return _to RegisterTimer(id, callback, interval, skipFirst); } \
  NS_IMETHOD UnregisterTimer(const nsAString& id) override { return _to UnregisterTimer(id); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIUPDATETIMERMANAGER(_to) \
  NS_IMETHOD RegisterTimer(const nsAString& id, nsITimerCallback *callback, uint32_t interval, bool skipFirst) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterTimer(id, callback, interval, skipFirst); } \
  NS_IMETHOD UnregisterTimer(const nsAString& id) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnregisterTimer(id); } 


#endif /* __gen_nsIUpdateTimerManager_h__ */
