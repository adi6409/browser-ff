/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/notification/nsINotificationStorage.idl
 */

#ifndef __gen_nsINotificationStorage_h__
#define __gen_nsINotificationStorage_h__


#ifndef __gen_domstubs_h__
#include "domstubs.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsINotificationStorageCallback */
#define NS_INOTIFICATIONSTORAGECALLBACK_IID_STR "c1622232-259c-43b0-b52e-89c39dcd9796"

#define NS_INOTIFICATIONSTORAGECALLBACK_IID \
  {0xc1622232, 0x259c, 0x43b0, \
    { 0xb5, 0x2e, 0x89, 0xc3, 0x9d, 0xcd, 0x97, 0x96 }}

class NS_NO_VTABLE nsINotificationStorageCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INOTIFICATIONSTORAGECALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINotificationStorageCallback;

  /* void handle (in AString id, in AString title, in AString dir, in AString lang, in AString body, in AString tag, in AString icon, in AString data, in AString behavior, in AString serviceWorkerRegistrationScope); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Handle(const nsAString& id, const nsAString& title, const nsAString& dir, const nsAString& lang, const nsAString& body, const nsAString& tag, const nsAString& icon, const nsAString& data, const nsAString& behavior, const nsAString& serviceWorkerRegistrationScope) = 0;

  /* void done (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Done(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINotificationStorageCallback, NS_INOTIFICATIONSTORAGECALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINOTIFICATIONSTORAGECALLBACK \
  NS_IMETHOD Handle(const nsAString& id, const nsAString& title, const nsAString& dir, const nsAString& lang, const nsAString& body, const nsAString& tag, const nsAString& icon, const nsAString& data, const nsAString& behavior, const nsAString& serviceWorkerRegistrationScope) override; \
  NS_IMETHOD Done(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINOTIFICATIONSTORAGECALLBACK \
  nsresult Handle(const nsAString& id, const nsAString& title, const nsAString& dir, const nsAString& lang, const nsAString& body, const nsAString& tag, const nsAString& icon, const nsAString& data, const nsAString& behavior, const nsAString& serviceWorkerRegistrationScope); \
  nsresult Done(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINOTIFICATIONSTORAGECALLBACK(_to) \
  NS_IMETHOD Handle(const nsAString& id, const nsAString& title, const nsAString& dir, const nsAString& lang, const nsAString& body, const nsAString& tag, const nsAString& icon, const nsAString& data, const nsAString& behavior, const nsAString& serviceWorkerRegistrationScope) override { return _to Handle(id, title, dir, lang, body, tag, icon, data, behavior, serviceWorkerRegistrationScope); } \
  NS_IMETHOD Done(void) override { return _to Done(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINOTIFICATIONSTORAGECALLBACK(_to) \
  NS_IMETHOD Handle(const nsAString& id, const nsAString& title, const nsAString& dir, const nsAString& lang, const nsAString& body, const nsAString& tag, const nsAString& icon, const nsAString& data, const nsAString& behavior, const nsAString& serviceWorkerRegistrationScope) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Handle(id, title, dir, lang, body, tag, icon, data, behavior, serviceWorkerRegistrationScope); } \
  NS_IMETHOD Done(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Done(); } 


/* starting interface:    nsINotificationStorage */
#define NS_INOTIFICATIONSTORAGE_IID_STR "17f85e52-fe57-440e-9ba1-5c312ca02b95"

#define NS_INOTIFICATIONSTORAGE_IID \
  {0x17f85e52, 0xfe57, 0x440e, \
    { 0x9b, 0xa1, 0x5c, 0x31, 0x2c, 0xa0, 0x2b, 0x95 }}

class NS_NO_VTABLE nsINotificationStorage : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INOTIFICATIONSTORAGE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINotificationStorage;

  /* void put (in AString origin, in AString id, in AString title, in AString dir, in AString lang, in AString body, in AString tag, in AString icon, in AString alertName, in AString data, in AString behavior, in AString serviceWorkerRegistrationScope); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Put(const nsAString& origin, const nsAString& id, const nsAString& title, const nsAString& dir, const nsAString& lang, const nsAString& body, const nsAString& tag, const nsAString& icon, const nsAString& alertName, const nsAString& data, const nsAString& behavior, const nsAString& serviceWorkerRegistrationScope) = 0;

  /* void get (in AString origin, in AString tag, in nsINotificationStorageCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Get(const nsAString& origin, const nsAString& tag, nsINotificationStorageCallback *aCallback) = 0;

  /* void getByID (in AString origin, in AString id, in nsINotificationStorageCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetByID(const nsAString& origin, const nsAString& id, nsINotificationStorageCallback *aCallback) = 0;

  /* void delete (in AString origin, in AString id); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Delete(const nsAString& origin, const nsAString& id) = 0;

  /* boolean canPut (in AString origin); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CanPut(const nsAString& origin, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINotificationStorage, NS_INOTIFICATIONSTORAGE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINOTIFICATIONSTORAGE \
  NS_IMETHOD Put(const nsAString& origin, const nsAString& id, const nsAString& title, const nsAString& dir, const nsAString& lang, const nsAString& body, const nsAString& tag, const nsAString& icon, const nsAString& alertName, const nsAString& data, const nsAString& behavior, const nsAString& serviceWorkerRegistrationScope) override; \
  NS_IMETHOD Get(const nsAString& origin, const nsAString& tag, nsINotificationStorageCallback *aCallback) override; \
  NS_IMETHOD GetByID(const nsAString& origin, const nsAString& id, nsINotificationStorageCallback *aCallback) override; \
  NS_IMETHOD Delete(const nsAString& origin, const nsAString& id) override; \
  NS_IMETHOD CanPut(const nsAString& origin, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINOTIFICATIONSTORAGE \
  nsresult Put(const nsAString& origin, const nsAString& id, const nsAString& title, const nsAString& dir, const nsAString& lang, const nsAString& body, const nsAString& tag, const nsAString& icon, const nsAString& alertName, const nsAString& data, const nsAString& behavior, const nsAString& serviceWorkerRegistrationScope); \
  nsresult Get(const nsAString& origin, const nsAString& tag, nsINotificationStorageCallback *aCallback); \
  nsresult GetByID(const nsAString& origin, const nsAString& id, nsINotificationStorageCallback *aCallback); \
  nsresult Delete(const nsAString& origin, const nsAString& id); \
  nsresult CanPut(const nsAString& origin, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINOTIFICATIONSTORAGE(_to) \
  NS_IMETHOD Put(const nsAString& origin, const nsAString& id, const nsAString& title, const nsAString& dir, const nsAString& lang, const nsAString& body, const nsAString& tag, const nsAString& icon, const nsAString& alertName, const nsAString& data, const nsAString& behavior, const nsAString& serviceWorkerRegistrationScope) override { return _to Put(origin, id, title, dir, lang, body, tag, icon, alertName, data, behavior, serviceWorkerRegistrationScope); } \
  NS_IMETHOD Get(const nsAString& origin, const nsAString& tag, nsINotificationStorageCallback *aCallback) override { return _to Get(origin, tag, aCallback); } \
  NS_IMETHOD GetByID(const nsAString& origin, const nsAString& id, nsINotificationStorageCallback *aCallback) override { return _to GetByID(origin, id, aCallback); } \
  NS_IMETHOD Delete(const nsAString& origin, const nsAString& id) override { return _to Delete(origin, id); } \
  NS_IMETHOD CanPut(const nsAString& origin, bool *_retval) override { return _to CanPut(origin, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINOTIFICATIONSTORAGE(_to) \
  NS_IMETHOD Put(const nsAString& origin, const nsAString& id, const nsAString& title, const nsAString& dir, const nsAString& lang, const nsAString& body, const nsAString& tag, const nsAString& icon, const nsAString& alertName, const nsAString& data, const nsAString& behavior, const nsAString& serviceWorkerRegistrationScope) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Put(origin, id, title, dir, lang, body, tag, icon, alertName, data, behavior, serviceWorkerRegistrationScope); } \
  NS_IMETHOD Get(const nsAString& origin, const nsAString& tag, nsINotificationStorageCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Get(origin, tag, aCallback); } \
  NS_IMETHOD GetByID(const nsAString& origin, const nsAString& id, nsINotificationStorageCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetByID(origin, id, aCallback); } \
  NS_IMETHOD Delete(const nsAString& origin, const nsAString& id) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Delete(origin, id); } \
  NS_IMETHOD CanPut(const nsAString& origin, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CanPut(origin, _retval); } 

#define NS_NOTIFICATION_STORAGE_CONTRACTID "@mozilla.org/notificationStorage;1"

#endif /* __gen_nsINotificationStorage_h__ */
