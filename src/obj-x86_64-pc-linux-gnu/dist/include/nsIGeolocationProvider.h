/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/system/nsIGeolocationProvider.idl
 */

#ifndef __gen_nsIGeolocationProvider_h__
#define __gen_nsIGeolocationProvider_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsIDOMGeoPosition; /* forward declaration */

class nsIGeolocationPrompt; /* forward declaration */


/* starting interface:    nsIGeolocationUpdate */
#define NS_IGEOLOCATIONUPDATE_IID_STR "643dc5e9-b911-4b2c-8d44-603162696baf"

#define NS_IGEOLOCATIONUPDATE_IID \
  {0x643dc5e9, 0xb911, 0x4b2c, \
    { 0x8d, 0x44, 0x60, 0x31, 0x62, 0x69, 0x6b, 0xaf }}

class NS_NO_VTABLE nsIGeolocationUpdate : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IGEOLOCATIONUPDATE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIGeolocationUpdate;

  /* void update (in nsIDOMGeoPosition position); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Update(nsIDOMGeoPosition *position) = 0;

  /* [can_run_script] void notifyError (in unsigned short error); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD NotifyError(uint16_t error) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIGeolocationUpdate, NS_IGEOLOCATIONUPDATE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIGEOLOCATIONUPDATE \
  NS_IMETHOD Update(nsIDOMGeoPosition *position) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD NotifyError(uint16_t error) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIGEOLOCATIONUPDATE \
  nsresult Update(nsIDOMGeoPosition *position); \
  MOZ_CAN_RUN_SCRIPT nsresult NotifyError(uint16_t error); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIGEOLOCATIONUPDATE(_to) \
  NS_IMETHOD Update(nsIDOMGeoPosition *position) override { return _to Update(position); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD NotifyError(uint16_t error) override { return _to NotifyError(error); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIGEOLOCATIONUPDATE(_to) \
  NS_IMETHOD Update(nsIDOMGeoPosition *position) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Update(position); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD NotifyError(uint16_t error) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyError(error); } 


/* starting interface:    nsIGeolocationProvider */
#define NS_IGEOLOCATIONPROVIDER_IID_STR "ac4a133b-9f92-4f7c-b369-d40cb6b17650"

#define NS_IGEOLOCATIONPROVIDER_IID \
  {0xac4a133b, 0x9f92, 0x4f7c, \
    { 0xb3, 0x69, 0xd4, 0x0c, 0xb6, 0xb1, 0x76, 0x50 }}

class NS_NO_VTABLE nsIGeolocationProvider : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IGEOLOCATIONPROVIDER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIGeolocationProvider;

  /* void startup (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Startup(void) = 0;

  /* void watch (in nsIGeolocationUpdate callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Watch(nsIGeolocationUpdate *callback) = 0;

  /* void shutdown (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Shutdown(void) = 0;

  /* void setHighAccuracy (in boolean enable); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetHighAccuracy(bool enable) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIGeolocationProvider, NS_IGEOLOCATIONPROVIDER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIGEOLOCATIONPROVIDER \
  NS_IMETHOD Startup(void) override; \
  NS_IMETHOD Watch(nsIGeolocationUpdate *callback) override; \
  NS_IMETHOD Shutdown(void) override; \
  NS_IMETHOD SetHighAccuracy(bool enable) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIGEOLOCATIONPROVIDER \
  nsresult Startup(void); \
  nsresult Watch(nsIGeolocationUpdate *callback); \
  nsresult Shutdown(void); \
  nsresult SetHighAccuracy(bool enable); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIGEOLOCATIONPROVIDER(_to) \
  NS_IMETHOD Startup(void) override { return _to Startup(); } \
  NS_IMETHOD Watch(nsIGeolocationUpdate *callback) override { return _to Watch(callback); } \
  NS_IMETHOD Shutdown(void) override { return _to Shutdown(); } \
  NS_IMETHOD SetHighAccuracy(bool enable) override { return _to SetHighAccuracy(enable); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIGEOLOCATIONPROVIDER(_to) \
  NS_IMETHOD Startup(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Startup(); } \
  NS_IMETHOD Watch(nsIGeolocationUpdate *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Watch(callback); } \
  NS_IMETHOD Shutdown(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Shutdown(); } \
  NS_IMETHOD SetHighAccuracy(bool enable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetHighAccuracy(enable); } 

/*
    This must be implemented by geolocation providers.  It
    must support nsIGeolocationProvider.
*/
#define NS_GEOLOCATION_PROVIDER_CONTRACTID "@mozilla.org/geolocation/provider;1"

#endif /* __gen_nsIGeolocationProvider_h__ */
