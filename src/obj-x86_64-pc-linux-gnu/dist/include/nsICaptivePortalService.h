/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsICaptivePortalService.idl
 */

#ifndef __gen_nsICaptivePortalService_h__
#define __gen_nsICaptivePortalService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsICaptivePortalServiceCallback */
#define NS_ICAPTIVEPORTALSERVICECALLBACK_IID_STR "b5fd5629-d04c-4138-9529-9311f291ecd4"

#define NS_ICAPTIVEPORTALSERVICECALLBACK_IID \
  {0xb5fd5629, 0xd04c, 0x4138, \
    { 0x95, 0x29, 0x93, 0x11, 0xf2, 0x91, 0xec, 0xd4 }}

class NS_NO_VTABLE nsICaptivePortalServiceCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICAPTIVEPORTALSERVICECALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICaptivePortalServiceCallback;

  /* void complete (in bool success, in nsresult error); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Complete(bool success, nsresult error) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICaptivePortalServiceCallback, NS_ICAPTIVEPORTALSERVICECALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICAPTIVEPORTALSERVICECALLBACK \
  NS_IMETHOD Complete(bool success, nsresult error) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICAPTIVEPORTALSERVICECALLBACK \
  nsresult Complete(bool success, nsresult error); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICAPTIVEPORTALSERVICECALLBACK(_to) \
  NS_IMETHOD Complete(bool success, nsresult error) override { return _to Complete(success, error); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICAPTIVEPORTALSERVICECALLBACK(_to) \
  NS_IMETHOD Complete(bool success, nsresult error) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Complete(success, error); } 


/* starting interface:    nsICaptivePortalService */
#define NS_ICAPTIVEPORTALSERVICE_IID_STR "bdbe0555-fc3d-4f7b-9205-c309ceb2d641"

#define NS_ICAPTIVEPORTALSERVICE_IID \
  {0xbdbe0555, 0xfc3d, 0x4f7b, \
    { 0x92, 0x05, 0xc3, 0x09, 0xce, 0xb2, 0xd6, 0x41 }}

class NS_NO_VTABLE nsICaptivePortalService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICAPTIVEPORTALSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICaptivePortalService;

  enum {
    UNKNOWN = 0,
    NOT_CAPTIVE = 1,
    UNLOCKED_PORTAL = 2,
    LOCKED_PORTAL = 3
  };

  /* void recheckCaptivePortal (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RecheckCaptivePortal(void) = 0;

  /* readonly attribute long state; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetState(int32_t *aState) = 0;

  /* readonly attribute unsigned long long lastChecked; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLastChecked(uint64_t *aLastChecked) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICaptivePortalService, NS_ICAPTIVEPORTALSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICAPTIVEPORTALSERVICE \
  NS_IMETHOD RecheckCaptivePortal(void) override; \
  NS_IMETHOD GetState(int32_t *aState) override; \
  NS_IMETHOD GetLastChecked(uint64_t *aLastChecked) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICAPTIVEPORTALSERVICE \
  nsresult RecheckCaptivePortal(void); \
  nsresult GetState(int32_t *aState); \
  nsresult GetLastChecked(uint64_t *aLastChecked); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICAPTIVEPORTALSERVICE(_to) \
  NS_IMETHOD RecheckCaptivePortal(void) override { return _to RecheckCaptivePortal(); } \
  NS_IMETHOD GetState(int32_t *aState) override { return _to GetState(aState); } \
  NS_IMETHOD GetLastChecked(uint64_t *aLastChecked) override { return _to GetLastChecked(aLastChecked); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICAPTIVEPORTALSERVICE(_to) \
  NS_IMETHOD RecheckCaptivePortal(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RecheckCaptivePortal(); } \
  NS_IMETHOD GetState(int32_t *aState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetState(aState); } \
  NS_IMETHOD GetLastChecked(uint64_t *aLastChecked) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastChecked(aLastChecked); } 

/**
 * This observer notification will be emitted when the captive portal state
 * changes. After receiving it, the ContentParent will send an IPC message
 * to the ContentChild, which will set the state in the captive portal service
 * in the child.
 */
#define NS_IPC_CAPTIVE_PORTAL_SET_STATE "ipc:network:captive-portal-set-state"
/**
 * This notification will be emitted when the captive portal service has
 * determined that we can connect to the internet.
 * The service will pass either "captive" if there is an unlocked captive portal
 * present, or "clear" if no captive portal was detected.
 * Note: this notification only gets sent in the parent process.
 */
#define NS_CAPTIVE_PORTAL_CONNECTIVITY "network:captive-portal-connectivity"

#endif /* __gen_nsICaptivePortalService_h__ */
