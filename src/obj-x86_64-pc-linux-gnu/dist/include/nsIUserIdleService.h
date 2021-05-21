/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIUserIdleService.idl
 */

#ifndef __gen_nsIUserIdleService_h__
#define __gen_nsIUserIdleService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIObserver; /* forward declaration */


/* starting interface:    nsIUserIdleService */
#define NS_IUSERIDLESERVICE_IID_STR "cc52f19a-63ae-4a1c-9cc3-e79eace0b471"

#define NS_IUSERIDLESERVICE_IID \
  {0xcc52f19a, 0x63ae, 0x4a1c, \
    { 0x9c, 0xc3, 0xe7, 0x9e, 0xac, 0xe0, 0xb4, 0x71 }}

class NS_NO_VTABLE nsIUserIdleService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IUSERIDLESERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUserIdleService;

  /* readonly attribute unsigned long idleTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIdleTime(uint32_t *aIdleTime) = 0;

  /* void addIdleObserver (in nsIObserver observer, in unsigned long time); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddIdleObserver(nsIObserver *observer, uint32_t time) = 0;

  /* void removeIdleObserver (in nsIObserver observer, in unsigned long time); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveIdleObserver(nsIObserver *observer, uint32_t time) = 0;

  /* attribute boolean disabled; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDisabled(bool *aDisabled) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDisabled(bool aDisabled) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUserIdleService, NS_IUSERIDLESERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIUSERIDLESERVICE \
  NS_IMETHOD GetIdleTime(uint32_t *aIdleTime) override; \
  NS_IMETHOD AddIdleObserver(nsIObserver *observer, uint32_t time) override; \
  NS_IMETHOD RemoveIdleObserver(nsIObserver *observer, uint32_t time) override; \
  NS_IMETHOD GetDisabled(bool *aDisabled) override; \
  NS_IMETHOD SetDisabled(bool aDisabled) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIUSERIDLESERVICE \
  nsresult GetIdleTime(uint32_t *aIdleTime); \
  nsresult AddIdleObserver(nsIObserver *observer, uint32_t time); \
  nsresult RemoveIdleObserver(nsIObserver *observer, uint32_t time); \
  nsresult GetDisabled(bool *aDisabled); \
  nsresult SetDisabled(bool aDisabled); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIUSERIDLESERVICE(_to) \
  NS_IMETHOD GetIdleTime(uint32_t *aIdleTime) override { return _to GetIdleTime(aIdleTime); } \
  NS_IMETHOD AddIdleObserver(nsIObserver *observer, uint32_t time) override { return _to AddIdleObserver(observer, time); } \
  NS_IMETHOD RemoveIdleObserver(nsIObserver *observer, uint32_t time) override { return _to RemoveIdleObserver(observer, time); } \
  NS_IMETHOD GetDisabled(bool *aDisabled) override { return _to GetDisabled(aDisabled); } \
  NS_IMETHOD SetDisabled(bool aDisabled) override { return _to SetDisabled(aDisabled); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIUSERIDLESERVICE(_to) \
  NS_IMETHOD GetIdleTime(uint32_t *aIdleTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIdleTime(aIdleTime); } \
  NS_IMETHOD AddIdleObserver(nsIObserver *observer, uint32_t time) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddIdleObserver(observer, time); } \
  NS_IMETHOD RemoveIdleObserver(nsIObserver *observer, uint32_t time) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveIdleObserver(observer, time); } \
  NS_IMETHOD GetDisabled(bool *aDisabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDisabled(aDisabled); } \
  NS_IMETHOD SetDisabled(bool aDisabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDisabled(aDisabled); } 

    /**
     * Observer topic notification for idle window: OBSERVER_TOPIC_IDLE.
     * Observer topic notification for active window: OBSERVER_TOPIC_ACTIVE.
     */
    #define OBSERVER_TOPIC_IDLE "idle"
    #define OBSERVER_TOPIC_ACTIVE "active"
    #define OBSERVER_TOPIC_IDLE_DAILY "idle-daily"

#endif /* __gen_nsIUserIdleService_h__ */
