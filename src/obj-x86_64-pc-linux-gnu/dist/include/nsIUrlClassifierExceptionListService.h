/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/url-classifier/nsIUrlClassifierExceptionListService.idl
 */

#ifndef __gen_nsIUrlClassifierExceptionListService_h__
#define __gen_nsIUrlClassifierExceptionListService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIUrlClassifierExceptionListObserver */
#define NS_IURLCLASSIFIEREXCEPTIONLISTOBSERVER_IID_STR "f7c918e5-94bf-4b6e-9758-ef7bdab6af7e"

#define NS_IURLCLASSIFIEREXCEPTIONLISTOBSERVER_IID \
  {0xf7c918e5, 0x94bf, 0x4b6e, \
    { 0x97, 0x58, 0xef, 0x7b, 0xda, 0xb6, 0xaf, 0x7e }}

class NS_NO_VTABLE nsIUrlClassifierExceptionListObserver : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURLCLASSIFIEREXCEPTIONLISTOBSERVER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUrlClassifierExceptionListObserver;

  /* void onExceptionListUpdate (in ACString aList); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnExceptionListUpdate(const nsACString& aList) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUrlClassifierExceptionListObserver, NS_IURLCLASSIFIEREXCEPTIONLISTOBSERVER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURLCLASSIFIEREXCEPTIONLISTOBSERVER \
  NS_IMETHOD OnExceptionListUpdate(const nsACString& aList) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURLCLASSIFIEREXCEPTIONLISTOBSERVER \
  nsresult OnExceptionListUpdate(const nsACString& aList); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURLCLASSIFIEREXCEPTIONLISTOBSERVER(_to) \
  NS_IMETHOD OnExceptionListUpdate(const nsACString& aList) override { return _to OnExceptionListUpdate(aList); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURLCLASSIFIEREXCEPTIONLISTOBSERVER(_to) \
  NS_IMETHOD OnExceptionListUpdate(const nsACString& aList) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnExceptionListUpdate(aList); } 


/* starting interface:    nsIUrlClassifierExceptionListService */
#define NS_IURLCLASSIFIEREXCEPTIONLISTSERVICE_IID_STR "75c3d1a3-e977-4079-9e27-b3b56bdb76ea"

#define NS_IURLCLASSIFIEREXCEPTIONLISTSERVICE_IID \
  {0x75c3d1a3, 0xe977, 0x4079, \
    { 0x9e, 0x27, 0xb3, 0xb5, 0x6b, 0xdb, 0x76, 0xea }}

class NS_NO_VTABLE nsIUrlClassifierExceptionListService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURLCLASSIFIEREXCEPTIONLISTSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUrlClassifierExceptionListService;

  /* void registerAndRunExceptionListObserver (in ACString aFeature, in ACString aPrefName, in nsIUrlClassifierExceptionListObserver aObserver); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RegisterAndRunExceptionListObserver(const nsACString& aFeature, const nsACString& aPrefName, nsIUrlClassifierExceptionListObserver *aObserver) = 0;

  /* void unregisterExceptionListObserver (in ACString aFeature, in nsIUrlClassifierExceptionListObserver aObserver); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UnregisterExceptionListObserver(const nsACString& aFeature, nsIUrlClassifierExceptionListObserver *aObserver) = 0;

  /* void clear (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Clear(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUrlClassifierExceptionListService, NS_IURLCLASSIFIEREXCEPTIONLISTSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURLCLASSIFIEREXCEPTIONLISTSERVICE \
  NS_IMETHOD RegisterAndRunExceptionListObserver(const nsACString& aFeature, const nsACString& aPrefName, nsIUrlClassifierExceptionListObserver *aObserver) override; \
  NS_IMETHOD UnregisterExceptionListObserver(const nsACString& aFeature, nsIUrlClassifierExceptionListObserver *aObserver) override; \
  NS_IMETHOD Clear(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURLCLASSIFIEREXCEPTIONLISTSERVICE \
  nsresult RegisterAndRunExceptionListObserver(const nsACString& aFeature, const nsACString& aPrefName, nsIUrlClassifierExceptionListObserver *aObserver); \
  nsresult UnregisterExceptionListObserver(const nsACString& aFeature, nsIUrlClassifierExceptionListObserver *aObserver); \
  nsresult Clear(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURLCLASSIFIEREXCEPTIONLISTSERVICE(_to) \
  NS_IMETHOD RegisterAndRunExceptionListObserver(const nsACString& aFeature, const nsACString& aPrefName, nsIUrlClassifierExceptionListObserver *aObserver) override { return _to RegisterAndRunExceptionListObserver(aFeature, aPrefName, aObserver); } \
  NS_IMETHOD UnregisterExceptionListObserver(const nsACString& aFeature, nsIUrlClassifierExceptionListObserver *aObserver) override { return _to UnregisterExceptionListObserver(aFeature, aObserver); } \
  NS_IMETHOD Clear(void) override { return _to Clear(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURLCLASSIFIEREXCEPTIONLISTSERVICE(_to) \
  NS_IMETHOD RegisterAndRunExceptionListObserver(const nsACString& aFeature, const nsACString& aPrefName, nsIUrlClassifierExceptionListObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterAndRunExceptionListObserver(aFeature, aPrefName, aObserver); } \
  NS_IMETHOD UnregisterExceptionListObserver(const nsACString& aFeature, nsIUrlClassifierExceptionListObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnregisterExceptionListObserver(aFeature, aObserver); } \
  NS_IMETHOD Clear(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Clear(); } 


#endif /* __gen_nsIUrlClassifierExceptionListService_h__ */
